//! [`RadarChart`]: several series compared over shared axes (spider / web chart).

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

use crate::braille::{polar, Braille};
use crate::util::{block_inner, palette, sane, text_width, truncate};

/// One polygon of a [`RadarChart`].
#[derive(Debug, Clone, PartialEq)]
pub struct RadarSeries<'a> {
    name: &'a str,
    values: Vec<f64>,
    color: Option<Color>,
}

impl<'a> RadarSeries<'a> {
    /// A series with one value per axis.
    #[must_use]
    pub fn new(name: &'a str, values: Vec<f64>) -> Self {
        Self {
            name,
            values,
            color: None,
        }
    }

    /// Fixes the series color (default: the palette, by position).
    #[must_use]
    pub const fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

/// A radar (spider) chart.
///
/// # Examples
///
/// ```
/// use tui_piechart::{RadarChart, RadarSeries};
///
/// let chart = RadarChart::new(
///     vec!["Speed", "Power", "Range", "Armor", "Agility"],
///     vec![
///         RadarSeries::new("Scout", vec![9.0, 3.0, 7.0, 2.0, 8.0]),
///         RadarSeries::new("Tank", vec![3.0, 8.0, 4.0, 9.0, 2.0]),
///     ],
/// );
/// ```
#[derive(Debug, Clone)]
pub struct RadarChart<'a> {
    axes: Vec<&'a str>,
    series: Vec<RadarSeries<'a>>,
    max: Option<f64>,
    rings: usize,
    legend: bool,
    block: Option<Block<'a>>,
    base: Style,
}

impl<'a> RadarChart<'a> {
    /// Creates a chart from axis labels and series.
    #[must_use]
    pub fn new(axes: Vec<&'a str>, series: Vec<RadarSeries<'a>>) -> Self {
        Self {
            axes,
            series,
            max: None,
            rings: 4,
            legend: true,
            block: None,
            base: Style::default(),
        }
    }

    /// Value at the outer ring (default: the largest value).
    #[must_use]
    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    /// Number of grid rings (`1..=10`, default 4).
    #[must_use]
    pub fn rings(mut self, rings: usize) -> Self {
        self.rings = rings.clamp(1, 10);
        self
    }

    /// Shows or hides the series legend.
    #[must_use]
    pub const fn legend(mut self, show: bool) -> Self {
        self.legend = show;
        self
    }

    /// Wraps the chart in a block.
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

    fn scale(&self) -> f64 {
        let m = self.max.map_or(0.0, sane);
        if m > 0.0 {
            return m;
        }
        let auto = self
            .series
            .iter()
            .flat_map(|s| s.values.iter())
            .map(|v| sane(*v))
            .fold(0.0, f64::max);
        if auto > 0.0 {
            auto
        } else {
            1.0
        }
    }
}

const GRID: Color = Color::DarkGray;

impl Widget for RadarChart<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.base);
        let inner = block_inner(self.block.as_ref(), area, buf);
        let n = self.axes.len();
        if inner.is_empty() || n < 3 {
            return;
        }
        let show_legend = self.legend && !self.series.is_empty() && inner.height >= 8;
        let legend_rows = u16::from(show_legend);
        let label_w = self
            .axes
            .iter()
            .map(|a| text_width(a))
            .max()
            .unwrap_or(0)
            .min(usize::from(inner.width) / 4) as u16;
        let grid = Rect::new(
            inner.x + label_w + 1,
            inner.y + 1,
            inner.width.saturating_sub(2 * (label_w + 1)),
            inner.height.saturating_sub(2 + legend_rows),
        );
        if grid.is_empty() {
            return;
        }
        let mut g = Braille::new(grid.width, grid.height);
        let c = (g.dots_w() / 2.0, g.dots_h() / 2.0);
        let r = c.0.min(c.1) - 1.0;
        if r < 3.0 {
            return;
        }
        let turn = |k: usize| k as f64 / n as f64;

        // Grid rings and spokes.
        for ring in 1..=self.rings {
            let rr = r * ring as f64 / self.rings as f64;
            for k in 0..n {
                g.line(
                    polar(c.0, c.1, rr, turn(k)),
                    polar(c.0, c.1, rr, turn((k + 1) % n)),
                    GRID,
                    0,
                );
            }
        }
        for k in 0..n {
            g.line(c, polar(c.0, c.1, r, turn(k)), GRID, 0);
        }

        // Series.
        let scale = self.scale();
        for (si, s) in self.series.iter().enumerate() {
            let color = s.color.unwrap_or_else(|| palette(si));
            let prio = 1 + si.min(200) as u8;
            let pts: Vec<(f64, f64)> = (0..n)
                .map(|k| {
                    let v = (sane(s.values.get(k).copied().unwrap_or(0.0)) / scale).min(1.0);
                    polar(c.0, c.1, r * v, turn(k))
                })
                .collect();
            for k in 0..n {
                g.line(pts[k], pts[(k + 1) % n], color, prio);
            }
            for &(x, y) in &pts {
                for (dx, dy) in [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)] {
                    g.plot(x + dx - 0.5, y + dy - 0.5, color, prio);
                }
            }
        }
        g.blit(grid, buf);

        // Axis labels around the web.
        let bold = Style::default().add_modifier(Modifier::BOLD);
        for (k, label) in self.axes.iter().enumerate() {
            let (dx, dy) = polar(c.0, c.1, r + 2.0, turn(k));
            let cx = i32::from(grid.x) + (dx / 2.0).floor() as i32;
            let cy = i32::from(grid.y) + (dy / 4.0).floor() as i32;
            let text = truncate(label, usize::from(label_w.max(1)));
            let w = text_width(&text) as i32;
            let s = (turn(k) * std::f64::consts::TAU).sin();
            let x = if s > 0.3 {
                cx + 1
            } else if s < -0.3 {
                cx - w
            } else {
                cx - w / 2
            };
            let y = cy.clamp(
                i32::from(inner.y),
                i32::from(inner.bottom()) - 1 - i32::from(legend_rows),
            );
            let x = x.clamp(
                i32::from(inner.x),
                (i32::from(inner.right()) - w).max(i32::from(inner.x)),
            );
            buf.set_string(x as u16, y as u16, &text, bold);
        }

        if show_legend {
            let y = inner.bottom() - 1;
            let mut x = inner.x;
            for (si, s) in self.series.iter().enumerate() {
                let color = s.color.unwrap_or_else(|| palette(si));
                let name = format!("■ {}  ", s.name);
                let w = text_width(&name) as u16;
                if x + w > inner.right() {
                    break;
                }
                buf.set_string(x, y, &name, Style::default().fg(color));
                x += w;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::assert_is_empty, clippy::float_cmp)]
    use super::*;

    fn chart() -> RadarChart<'static> {
        RadarChart::new(
            vec!["Speed", "Power", "Range", "Armor", "Agility"],
            vec![
                RadarSeries::new("Scout", vec![9.0, 3.0, 7.0, 2.0, 8.0]).color(Color::Red),
                RadarSeries::new("Tank", vec![3.0, 8.0, 4.0, 9.0, 2.0]).color(Color::Blue),
            ],
        )
    }

    fn draw(c: RadarChart<'_>, w: u16, h: u16) -> Buffer {
        let a = Rect::new(0, 0, w, h);
        let mut b = Buffer::empty(a);
        c.render(a, &mut b);
        b
    }

    fn text(b: &Buffer) -> String {
        b.content()
            .iter()
            .map(ratatui::buffer::Cell::symbol)
            .collect()
    }

    #[test]
    fn renders_axes_series_and_legend() {
        let b = draw(chart(), 60, 22);
        let t = text(&b);
        for l in [
            "Speed", "Power", "Range", "Armor", "Agility", "Scout", "Tank",
        ] {
            assert!(t.contains(l), "missing {l}");
        }
        for col in [Color::Red, Color::Blue] {
            assert!(b.content().iter().any(|c| c.fg == col && c.symbol() != " "));
        }
    }

    #[test]
    fn needs_three_axes() {
        let b = draw(RadarChart::new(vec!["a", "b"], vec![]), 40, 20);
        assert_eq!(text(&b).trim(), "");
    }

    #[test]
    fn values_change_picture_and_max_scales() {
        let a = draw(chart(), 60, 22);
        let b = draw(chart().max(100.0), 60, 22);
        assert_ne!(a, b);
    }

    #[test]
    fn legend_toggle() {
        assert!(!text(&draw(chart().legend(false), 60, 22)).contains("Scout"));
    }

    #[test]
    fn robust() {
        for (w, h) in [(0, 0), (1, 1), (6, 3), (20, 8), (80, 40)] {
            for axes in [vec![], vec!["a"; 3], vec!["long axis label"; 12]] {
                for series in [
                    vec![],
                    vec![RadarSeries::new("s", vec![])],
                    vec![RadarSeries::new(
                        "s",
                        vec![f64::NAN, -1.0, f64::INFINITY, 5.0],
                    )],
                ] {
                    let _ = draw(
                        RadarChart::new(axes.clone(), series)
                            .rings(99)
                            .max(f64::NAN),
                        w,
                        h,
                    );
                }
            }
        }
    }
}
