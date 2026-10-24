//! [`Funnel`]: stages narrowing from top to bottom, as bars or a smooth trapezoid.

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

use crate::util::{block_inner, contrast, palette, put, sane, text_width, truncate};

/// One stage of a [`Funnel`].
#[derive(Debug, Clone, PartialEq)]
pub struct FunnelStage<'a> {
    label: &'a str,
    value: f64,
    color: Option<Color>,
}

impl<'a> FunnelStage<'a> {
    /// A stage with `value` items.
    #[must_use]
    pub const fn new(label: &'a str, value: f64) -> Self {
        Self {
            label,
            value,
            color: None,
        }
    }

    /// Fixes the stage color (default: the palette, by position).
    #[must_use]
    pub const fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

/// How a [`Funnel`] draws its stages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FunnelStyle {
    /// Centered bars, one per stage (default).
    #[default]
    Bars,
    /// A continuous trapezoid: each stage tapers towards the next one.
    Trapezoid,
}

/// A conversion funnel.
///
/// # Examples
///
/// ```
/// use tui_piechart::{Funnel, FunnelStage};
///
/// let funnel = Funnel::new(vec![
///     FunnelStage::new("Visits", 10_000.0),
///     FunnelStage::new("Sign-ups", 2_400.0),
///     FunnelStage::new("Paid", 380.0),
/// ]);
/// ```
#[derive(Debug, Clone)]
pub struct Funnel<'a> {
    stages: Vec<FunnelStage<'a>>,
    style: FunnelStyle,
    show_values: bool,
    block: Option<Block<'a>>,
    base: Style,
}

impl<'a> Funnel<'a> {
    /// Creates a funnel from stages, widest first.
    #[must_use]
    pub fn new(stages: Vec<FunnelStage<'a>>) -> Self {
        Self {
            stages,
            style: FunnelStyle::default(),
            show_values: true,
            block: None,
            base: Style::default(),
        }
    }

    /// Chooses bars or a trapezoid.
    #[must_use]
    pub const fn style(mut self, style: FunnelStyle) -> Self {
        self.style = style;
        self
    }

    /// Shows or hides the value / conversion column on the right.
    #[must_use]
    pub const fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Wraps the funnel in a block.
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

    /// Share of each stage relative to the previous one (the first is `100`).
    #[must_use]
    pub fn conversions(&self) -> Vec<f64> {
        let mut prev = None;
        self.stages
            .iter()
            .map(|s| {
                let v = sane(s.value);
                let c = match prev {
                    None => 100.0,
                    Some(p) if p > 0.0 => v / p * 100.0,
                    Some(_) => 0.0,
                };
                prev = Some(v);
                c
            })
            .collect()
    }

    fn color_of(&self, i: usize) -> Color {
        self.stages[i].color.unwrap_or_else(|| palette(i))
    }
}

/// Draws a horizontal span `[left, right)` in half-cell units (`2 * cell`),
/// using half blocks at ragged edges.
fn span_row(buf: &mut Buffer, area: Rect, y: u16, left: f64, right: f64, color: Color) {
    let (l, r) = (left.round() as i64, right.round() as i64);
    if r <= l {
        return;
    }
    let style = Style::default().fg(color);
    for cell in (l / 2)..=((r - 1) / 2) {
        let (a, b) = (cell * 2, cell * 2 + 2);
        let (lo, hi) = (l.max(a), r.min(b));
        let ch = match (lo == a, hi == b) {
            (true, true) => '█',
            (true, false) => '▌',
            (false, true) => '▐',
            (false, false) => continue,
        };
        if cell >= 0 && cell < i64::from(area.width) {
            put(buf, area.x + cell as u16, y, ch, style);
        }
    }
}

impl Widget for Funnel<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.base);
        let inner = block_inner(self.block.as_ref(), area, buf);
        let n = self.stages.len();
        if inner.is_empty() || n == 0 {
            return;
        }
        let max = self
            .stages
            .iter()
            .map(|s| sane(s.value))
            .fold(0.0, f64::max);
        if max <= 0.0 {
            return;
        }
        let conv = self.conversions();
        let value_w = if self.show_values {
            self.stages
                .iter()
                .zip(&conv)
                .map(|(s, c)| text_width(&format!("{} ({c:.0}%)", fmt_value(sane(s.value)))))
                .max()
                .unwrap_or(0)
        } else {
            0
        };
        let value_w = if usize::from(inner.width) > value_w + 12 {
            value_w
        } else {
            0
        };
        let bar_area = Rect::new(
            inner.x,
            inner.y,
            inner.width - value_w as u16 - u16::from(value_w > 0),
            inner.height,
        );
        let rows = usize::from(inner.height);
        let per = (rows / n).max(1);
        let half_w = f64::from(bar_area.width) * 2.0; // half-cell units
        for (i, stage) in self.stages.iter().enumerate() {
            let top = i * per;
            if top >= rows {
                break;
            }
            let color = self.color_of(i);
            let v = sane(stage.value);
            let next = self.stages.get(i + 1).map_or(v, |s| sane(s.value));
            let h = per.min(rows - top);
            for r in 0..h {
                let w_units = match self.style {
                    FunnelStyle::Bars => v / max * half_w,
                    FunnelStyle::Trapezoid => {
                        let t = (r as f64 + 0.5) / per as f64;
                        (v + (next - v) * t) / max * half_w
                    }
                };
                let left = (half_w - w_units) / 2.0;
                span_row(
                    buf,
                    bar_area,
                    inner.y + (top + r) as u16,
                    left,
                    left + w_units,
                    color,
                );
            }
            // Label on the middle row, inside the bar when it fits.
            let mid = inner.y + (top + h / 2) as u16;
            let bar_cells = (v / max * f64::from(bar_area.width)) as usize;
            let label = truncate(stage.label, usize::from(bar_area.width));
            let lw = text_width(&label);
            let fits = lw <= bar_cells;
            // Inside the bar when it fits, otherwise at the left edge.
            let x = if fits {
                bar_area.x + ((usize::from(bar_area.width) - lw) / 2) as u16
            } else {
                bar_area.x
            };
            let style = if fits {
                Style::default()
                    .fg(contrast(color))
                    .bg(color)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(color)
            };
            buf.set_string(x, mid, &label, style);
            if value_w > 0 {
                let text = format!("{} ({:.0}%)", fmt_value(v), conv[i]);
                buf.set_string(
                    bar_area.right() + 1,
                    mid,
                    truncate(&text, value_w),
                    Style::default().fg(color),
                );
            }
        }
    }
}

fn fmt_value(v: f64) -> String {
    if v >= 1_000_000.0 {
        format!("{:.1}M", v / 1_000_000.0)
    } else if v >= 10_000.0 {
        format!("{:.0}k", v / 1_000.0)
    } else if (v - v.round()).abs() < 1e-9 {
        format!("{v:.0}")
    } else {
        format!("{v:.1}")
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::assert_is_empty, clippy::float_cmp)]
    use super::*;

    fn stages() -> Vec<FunnelStage<'static>> {
        vec![
            FunnelStage::new("Visits", 10_000.0),
            FunnelStage::new("Sign-ups", 2_400.0),
            FunnelStage::new("Paid", 380.0).color(Color::Red),
        ]
    }

    fn draw(f: Funnel<'_>, w: u16, h: u16) -> Buffer {
        let a = Rect::new(0, 0, w, h);
        let mut b = Buffer::empty(a);
        f.render(a, &mut b);
        b
    }

    fn text(b: &Buffer) -> String {
        b.content()
            .iter()
            .map(ratatui::buffer::Cell::symbol)
            .collect()
    }

    #[test]
    fn conversions_are_relative_to_previous() {
        let c = Funnel::new(stages()).conversions();
        assert_eq!(c[0], 100.0);
        assert!((c[1] - 24.0).abs() < 1e-9);
        assert!((c[2] - 15.833_333).abs() < 1e-4);
        assert!(
            Funnel::new(vec![FunnelStage::new("a", 0.0), FunnelStage::new("b", 5.0)])
                .conversions()
                .iter()
                .all(|c| c.is_finite())
        );
    }

    #[test]
    fn renders_labels_and_values() {
        let t = text(&draw(Funnel::new(stages()), 60, 9));
        assert!(t.contains("Visits") && t.contains("Sign-ups") && t.contains("24%"));
        assert!(t.contains("10k") || t.contains("10000"));
    }

    #[test]
    fn bars_narrow_downwards() {
        let b = draw(Funnel::new(stages()).show_values(false), 60, 9);
        let filled = |y: u16| (0..60).filter(|x| b[(*x, y)].symbol() == "█").count();
        assert!(filled(0) > filled(4) && filled(4) > filled(8));
    }

    #[test]
    fn trapezoid_tapers_within_a_stage() {
        let b = draw(
            Funnel::new(stages())
                .style(FunnelStyle::Trapezoid)
                .show_values(false),
            60,
            12,
        );
        let filled = |y: u16| (0..60).filter(|x| b[(*x, y)].symbol() != " ").count();
        assert!(filled(0) >= filled(3));
        assert_ne!(b, draw(Funnel::new(stages()).show_values(false), 60, 12));
    }

    #[test]
    fn stage_color_applies() {
        let b = draw(Funnel::new(stages()).show_values(false), 60, 9);
        assert!(b
            .content()
            .iter()
            .any(|c| c.symbol() == "█" && c.fg == Color::Red));
    }

    #[test]
    fn robust() {
        for style in [FunnelStyle::Bars, FunnelStyle::Trapezoid] {
            for (w, h) in [(0, 0), (1, 1), (5, 2), (20, 1), (60, 30)] {
                for st in [
                    vec![],
                    stages(),
                    vec![FunnelStage::new("n", f64::NAN)],
                    vec![FunnelStage::new("z", 0.0); 3],
                    (0..50)
                        .map(|i| FunnelStage::new("s", f64::from(i)))
                        .collect(),
                ] {
                    let _ = draw(Funnel::new(st).style(style).block(Block::bordered()), w, h);
                }
            }
        }
    }

    #[test]
    fn value_formatting() {
        assert_eq!(fmt_value(5.0), "5");
        assert_eq!(fmt_value(5.5), "5.5");
        assert_eq!(fmt_value(12_000.0), "12k");
        assert_eq!(fmt_value(2_500_000.0), "2.5M");
    }
}
