//! [`Treemap`]: nested rectangles sized by value (squarified layout).

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::many_single_char_names,
    clippy::too_many_lines,
    clippy::cast_possible_wrap
)]

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Widget};

use crate::util::{block_inner, contrast, palette, sane, text_width, truncate};

/// One tile of a [`Treemap`].
#[derive(Debug, Clone, PartialEq)]
pub struct TreemapItem<'a> {
    label: &'a str,
    value: f64,
    color: Option<Color>,
}

impl<'a> TreemapItem<'a> {
    /// A tile sized by `value`.
    #[must_use]
    pub const fn new(label: &'a str, value: f64) -> Self {
        Self {
            label,
            value,
            color: None,
        }
    }

    /// Fixes the tile color (default: the palette, by position).
    #[must_use]
    pub const fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

/// A squarified treemap.
///
/// # Examples
///
/// ```
/// use tui_piechart::{Treemap, TreemapItem};
///
/// let map = Treemap::new(vec![
///     TreemapItem::new("src", 420.0),
///     TreemapItem::new("tests", 180.0),
///     TreemapItem::new("docs", 90.0),
/// ]);
/// ```
#[derive(Debug, Clone)]
pub struct Treemap<'a> {
    items: Vec<TreemapItem<'a>>,
    gap: bool,
    show_values: bool,
    block: Option<Block<'a>>,
    base: Style,
}

impl<'a> Treemap<'a> {
    /// Creates a treemap from items (any order; non-positive values are skipped).
    #[must_use]
    pub fn new(items: Vec<TreemapItem<'a>>) -> Self {
        Self {
            items,
            gap: true,
            show_values: true,
            block: None,
            base: Style::default(),
        }
    }

    /// Leaves a one-cell gutter on the right and bottom of each tile.
    #[must_use]
    pub const fn gap(mut self, gap: bool) -> Self {
        self.gap = gap;
        self
    }

    /// Shows or hides the value under each label.
    #[must_use]
    pub const fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Wraps the treemap in a block.
    #[must_use]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Base style of the whole area.
    #[must_use]
    pub const fn base_style(mut self, style: Style) -> Self {
        self.base = style;
        self
    }
}

fn worst(row: &[f64], sum: f64, side: f64) -> f64 {
    row.iter().fold(0.0_f64, |acc, a| {
        let r = (side * side * a / (sum * sum)).max(sum * sum / (side * side * a));
        acc.max(r)
    })
}

/// Squarified layout of `areas` (descending, sum = `w * h`) inside `(x, y, w, h)`.
/// Returns one `(x, y, w, h)` per area, in input order.
fn squarify(
    areas: &[f64],
    (mut x, mut y, mut w, mut h): (f64, f64, f64, f64),
) -> Vec<(f64, f64, f64, f64)> {
    let mut out = Vec::with_capacity(areas.len());
    let mut i = 0;
    while i < areas.len() {
        let side = w.min(h).max(f64::EPSILON);
        let mut j = i + 1;
        let mut sum = areas[i];
        let mut best = worst(&areas[i..j], sum, side);
        while j < areas.len() {
            let s2 = sum + areas[j];
            let w2 = worst(&areas[i..=j], s2, side);
            if w2 <= best {
                sum = s2;
                best = w2;
                j += 1;
            } else {
                break;
            }
        }
        if w >= h {
            let cw = sum / h;
            let mut yy = y;
            for a in &areas[i..j] {
                let ch = a / cw;
                out.push((x, yy, cw, ch));
                yy += ch;
            }
            x += cw;
            w -= cw;
        } else {
            let rh = sum / w;
            let mut xx = x;
            for a in &areas[i..j] {
                let cw = a / rh;
                out.push((xx, y, cw, rh));
                xx += cw;
            }
            y += rh;
            h -= rh;
        }
        i = j;
    }
    out
}

/// Tile rectangles in cells for `values`, laid out inside `w`×`h` cells.
/// Cells are about twice as tall as wide, which the layout compensates for.
/// Returns `(index_into_values, x, y, w, h)` for every positive value.
#[must_use]
pub fn treemap_layout(values: &[f64], w: u16, h: u16) -> Vec<(usize, u16, u16, u16, u16)> {
    let mut order: Vec<usize> = (0..values.len())
        .filter(|i| sane(values[*i]) > 0.0)
        .collect();
    order.sort_by(|a, b| values[*b].total_cmp(&values[*a]));
    let total: f64 = order.iter().map(|i| values[*i]).sum();
    if order.is_empty() || w == 0 || h == 0 || total <= 0.0 {
        return Vec::new();
    }
    let (vw, vh) = (f64::from(w) * 2.0, f64::from(h));
    let areas: Vec<f64> = order.iter().map(|i| values[*i] / total * vw * vh).collect();
    let rects = squarify(&areas, (0.0, 0.0, vw, vh));
    let col = |v: f64| ((v / 2.0).round().max(0.0) as u16).min(w);
    let row = |v: f64| (v.round().max(0.0) as u16).min(h);
    order
        .iter()
        .zip(rects)
        .map(|(i, (x, y, rw, rh))| {
            let (x0, x1) = (col(x), col(x + rw));
            let (y0, y1) = (row(y), row(y + rh));
            (*i, x0, y0, x1.saturating_sub(x0), y1.saturating_sub(y0))
        })
        .collect()
}

impl Widget for Treemap<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.base);
        let inner = block_inner(self.block.as_ref(), area, buf);
        if inner.is_empty() {
            return;
        }
        let values: Vec<f64> = self.items.iter().map(|i| i.value).collect();
        for (idx, x, y, w, h) in treemap_layout(&values, inner.width, inner.height) {
            if w == 0 || h == 0 {
                continue;
            }
            let item = &self.items[idx];
            let color = item.color.unwrap_or_else(|| palette(idx));
            let (fw, fh) = if self.gap && w >= 3 && h >= 2 {
                (w - 1, h - 1)
            } else {
                (w, h)
            };
            let fill = Style::default().bg(color).fg(contrast(color));
            for dy in 0..fh {
                for dx in 0..fw {
                    if let Some(c) = buf.cell_mut((inner.x + x + dx, inner.y + y + dy)) {
                        c.set_char(' ').set_style(fill);
                    }
                }
            }
            let label = truncate(item.label, usize::from(fw));
            if !label.is_empty() {
                buf.set_string(
                    inner.x + x,
                    inner.y + y,
                    &label,
                    fill.add_modifier(Modifier::BOLD),
                );
            }
            if self.show_values && fh >= 2 {
                let v = format!("{:.0}", sane(item.value));
                if text_width(&v) <= usize::from(fw) {
                    buf.set_string(inner.x + x, inner.y + y + 1, v, fill);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::assert_is_empty, clippy::float_cmp)]
    use super::*;

    fn items() -> Vec<TreemapItem<'static>> {
        vec![
            TreemapItem::new("src", 420.0).color(Color::Red),
            TreemapItem::new("tests", 180.0).color(Color::Green),
            TreemapItem::new("docs", 90.0).color(Color::Blue),
            TreemapItem::new("ci", 30.0),
        ]
    }

    fn draw(t: Treemap<'_>, w: u16, h: u16) -> Buffer {
        let a = Rect::new(0, 0, w, h);
        let mut b = Buffer::empty(a);
        t.render(a, &mut b);
        b
    }

    #[test]
    fn layout_tiles_cover_area_without_overlap() {
        let vals = [420.0, 180.0, 90.0, 30.0, 12.0];
        let (w, h) = (60u16, 20u16);
        let tiles = treemap_layout(&vals, w, h);
        assert_eq!(tiles.len(), vals.len());
        let mut grid = vec![0u8; usize::from(w) * usize::from(h)];
        for (_, x, y, tw, th) in &tiles {
            for dy in 0..*th {
                for dx in 0..*tw {
                    grid[usize::from(y + dy) * usize::from(w) + usize::from(x + dx)] += 1;
                }
            }
        }
        assert!(grid.iter().all(|c| *c == 1), "tiles overlap or leave holes");
    }

    #[test]
    fn bigger_value_gets_bigger_tile() {
        let tiles = treemap_layout(&[10.0, 1000.0, 100.0], 50, 20);
        let area = |i: usize| {
            let t = tiles.iter().find(|t| t.0 == i).unwrap();
            u32::from(t.3) * u32::from(t.4)
        };
        assert!(area(1) > area(2) && area(2) > area(0));
    }

    #[test]
    fn layout_skips_bad_values_and_empty_area() {
        assert!(treemap_layout(&[0.0, -1.0, f64::NAN], 10, 10).is_empty());
        assert!(treemap_layout(&[1.0], 0, 10).is_empty());
        assert_eq!(treemap_layout(&[f64::NAN, 5.0], 10, 4).len(), 1);
    }

    #[test]
    fn single_item_fills_everything() {
        let t = treemap_layout(&[7.0], 12, 5);
        assert_eq!(t, vec![(0, 0, 0, 12, 5)]);
    }

    #[test]
    fn renders_labels_and_colors() {
        let b = draw(Treemap::new(items()), 60, 20);
        let text: String = b
            .content()
            .iter()
            .map(ratatui::buffer::Cell::symbol)
            .collect();
        assert!(text.contains("src") && text.contains("tests"));
        for c in [Color::Red, Color::Green, Color::Blue] {
            assert!(b.content().iter().any(|x| x.bg == c));
        }
    }

    #[test]
    fn gap_leaves_background_cells() {
        let with = draw(Treemap::new(items()).gap(true), 60, 20);
        let without = draw(Treemap::new(items()).gap(false), 60, 20);
        let bg = |b: &Buffer| b.content().iter().filter(|c| c.bg == Color::Reset).count();
        assert!(bg(&with) > bg(&without));
        assert_eq!(bg(&without), 0);
    }

    #[test]
    fn extreme_ratios_do_not_panic() {
        let vals = [1e300, 1.0, 1e-300, 5e-324, 1e150];
        for (w, h) in [(7, 3), (50, 20), (1, 1)] {
            let _ = treemap_layout(&vals, w, h);
        }
    }

    #[test]
    fn robust() {
        for (w, h) in [(0, 0), (1, 1), (3, 2), (10, 3), (100, 40)] {
            for it in [
                vec![],
                items(),
                vec![TreemapItem::new("n", f64::NAN)],
                (0..200)
                    .map(|i| TreemapItem::new("x", f64::from(i + 1)))
                    .collect(),
            ] {
                let _ = draw(Treemap::new(it).block(Block::bordered()), w, h);
            }
        }
    }
}
