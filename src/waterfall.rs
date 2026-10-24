//! [`Waterfall`]: running totals built from up / down steps and total bars.

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

use crate::util::{block_inner, put, text_width, truncate};

/// One step of a [`Waterfall`].
#[derive(Debug, Clone, PartialEq)]
pub struct WaterfallStep<'a> {
    label: &'a str,
    kind: StepKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum StepKind {
    Delta(f64),
    Total,
}

impl<'a> WaterfallStep<'a> {
    /// A step that adds (`> 0`) or removes (`< 0`) `value` from the running total.
    #[must_use]
    pub const fn delta(label: &'a str, value: f64) -> Self {
        Self {
            label,
            kind: StepKind::Delta(value),
        }
    }

    /// A bar from zero up to the running total so far.
    #[must_use]
    pub const fn total(label: &'a str) -> Self {
        Self {
            label,
            kind: StepKind::Total,
        }
    }
}

/// A bar with its value range, as computed by [`waterfall_bars`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WaterfallBar {
    /// Lower end of the bar.
    pub low: f64,
    /// Upper end of the bar.
    pub high: f64,
    /// Running total after this step.
    pub end: f64,
    /// Whether this is a total bar.
    pub is_total: bool,
    /// Whether the step went down.
    pub is_decrease: bool,
}

/// Computes bar ranges for `steps` (non-finite deltas count as `0`).
#[must_use]
pub fn waterfall_bars(steps: &[WaterfallStep<'_>]) -> Vec<WaterfallBar> {
    let mut run = 0.0_f64;
    steps
        .iter()
        .map(|s| match s.kind {
            StepKind::Delta(v) => {
                let v = if v.is_finite() { v } else { 0.0 };
                let start = run;
                run += v;
                WaterfallBar {
                    low: start.min(run),
                    high: start.max(run),
                    end: run,
                    is_total: false,
                    is_decrease: v < 0.0,
                }
            }
            StepKind::Total => WaterfallBar {
                low: run.min(0.0),
                high: run.max(0.0),
                end: run,
                is_total: true,
                is_decrease: false,
            },
        })
        .collect()
}

/// A waterfall chart.
///
/// # Examples
///
/// ```
/// use tui_piechart::{Waterfall, WaterfallStep};
///
/// let chart = Waterfall::new(vec![
///     WaterfallStep::delta("Revenue", 120.0),
///     WaterfallStep::delta("Costs", -45.0),
///     WaterfallStep::delta("Tax", -20.0),
///     WaterfallStep::total("Profit"),
/// ]);
/// ```
#[derive(Debug, Clone)]
pub struct Waterfall<'a> {
    steps: Vec<WaterfallStep<'a>>,
    up: Color,
    down: Color,
    total: Color,
    block: Option<Block<'a>>,
    base: Style,
}

impl<'a> Waterfall<'a> {
    /// Creates a waterfall from steps.
    #[must_use]
    pub fn new(steps: Vec<WaterfallStep<'a>>) -> Self {
        Self {
            steps,
            up: Color::Green,
            down: Color::Red,
            total: Color::Cyan,
            block: None,
            base: Style::default(),
        }
    }

    /// Colors for increases, decreases and totals.
    #[must_use]
    pub const fn colors(mut self, up: Color, down: Color, total: Color) -> Self {
        self.up = up;
        self.down = down;
        self.total = total;
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
}

const LOWER: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

impl Widget for Waterfall<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.base);
        let inner = block_inner(self.block.as_ref(), area, buf);
        let n = self.steps.len();
        if inner.is_empty() || n == 0 {
            return;
        }
        let bars = waterfall_bars(&self.steps);
        let label_row = inner.height >= 3;
        let rows = inner.height - u16::from(label_row);
        let lo = bars.iter().map(|b| b.low).fold(0.0, f64::min);
        let hi = bars.iter().map(|b| b.high).fold(0.0, f64::max);
        let span = hi - lo;
        if span <= 0.0 || rows == 0 {
            return;
        }
        let gap: usize = usize::from(usize::from(inner.width) >= n * 2);
        let bw = ((usize::from(inner.width) + gap) / n)
            .saturating_sub(gap)
            .max(1);
        let used = n * bw + (n - 1) * gap;
        if used > usize::from(inner.width) {
            return;
        }
        let x0 = inner.x + ((usize::from(inner.width) - used) / 2) as u16;
        let units = f64::from(rows) * 8.0;
        let to_unit = |v: f64| ((v - lo) / span * units).round() as i64;

        for (i, (bar, step)) in bars.iter().zip(&self.steps).enumerate() {
            let color = if bar.is_total {
                self.total
            } else if bar.is_decrease {
                self.down
            } else {
                self.up
            };
            let (ya, mut yb) = (to_unit(bar.low), to_unit(bar.high));
            if yb <= ya && bar.high > bar.low {
                yb = ya + 1;
            }
            let bx = x0 + (i * (bw + gap)) as u16;
            for r in 0..i64::from(rows) {
                let (cell_lo, cell_hi) = (r * 8, r * 8 + 8);
                let a = ya.max(cell_lo) - cell_lo;
                let b = yb.min(cell_hi) - cell_lo;
                if b <= a {
                    continue;
                }
                let y = inner.y + (i64::from(rows) - 1 - r) as u16;
                let (ch, style) = if a == 0 {
                    (LOWER[b as usize], Style::default().fg(color))
                } else if b == 8 {
                    // Upper part only: a lower block with swapped colors.
                    (
                        LOWER[a as usize],
                        Style::default().fg(color).add_modifier(Modifier::REVERSED),
                    )
                } else {
                    (LOWER[(b - a) as usize], Style::default().fg(color))
                };
                for dx in 0..bw {
                    put(buf, bx + dx as u16, y, ch, style);
                }
            }
            if label_row {
                let cell_w = bw + gap;
                let text = truncate(step.label, cell_w.max(1));
                let tx = bx + (bw.saturating_sub(text_width(&text)) / 2) as u16;
                if tx + (text_width(&text) as u16) <= inner.right() {
                    buf.set_string(
                        tx,
                        inner.bottom() - 1,
                        &text,
                        Style::default().fg(Color::DarkGray),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::assert_is_empty, clippy::float_cmp)]
    use super::*;

    fn steps() -> Vec<WaterfallStep<'static>> {
        vec![
            WaterfallStep::delta("Rev", 120.0),
            WaterfallStep::delta("Cost", -45.0),
            WaterfallStep::delta("Tax", -20.0),
            WaterfallStep::total("Net"),
        ]
    }

    fn draw(w: Waterfall<'_>, width: u16, height: u16) -> Buffer {
        let a = Rect::new(0, 0, width, height);
        let mut b = Buffer::empty(a);
        w.render(a, &mut b);
        b
    }

    #[test]
    fn bars_track_running_total() {
        let b = waterfall_bars(&steps());
        assert_eq!((b[0].low, b[0].high, b[0].end), (0.0, 120.0, 120.0));
        assert_eq!((b[1].low, b[1].high, b[1].end), (75.0, 120.0, 75.0));
        assert!(b[1].is_decrease && !b[0].is_decrease);
        assert_eq!((b[2].low, b[2].high), (55.0, 75.0));
        assert!(b[3].is_total && b[3].low == 0.0 && b[3].high == 55.0);
    }

    #[test]
    fn negative_running_total_supported() {
        let b = waterfall_bars(&[WaterfallStep::delta("a", -10.0), WaterfallStep::total("t")]);
        assert_eq!((b[1].low, b[1].high), (-10.0, 0.0));
    }

    #[test]
    fn non_finite_delta_is_zero() {
        let b = waterfall_bars(&[WaterfallStep::delta("a", f64::NAN)]);
        assert_eq!((b[0].low, b[0].high, b[0].end), (0.0, 0.0, 0.0));
    }

    #[test]
    fn renders_colors_and_labels() {
        let b = draw(Waterfall::new(steps()), 40, 12);
        let text: String = b
            .content()
            .iter()
            .map(ratatui::buffer::Cell::symbol)
            .collect();
        assert!(text.contains("Rev") && text.contains("Net"));
        for c in [Color::Green, Color::Red, Color::Cyan] {
            assert!(
                b.content().iter().any(|x| x.fg == c && x.symbol() == "█"),
                "{c:?}"
            );
        }
    }

    #[test]
    fn floating_bar_does_not_touch_the_floor() {
        let b = draw(Waterfall::new(steps()), 40, 12);
        // Row just above the label row: the "Cost" bar floats, "Rev" and "Net" start there.
        let floor = 10u16;
        let cost_x = (0..40).find(|x| b[(*x, 2)].fg == Color::Red && b[(*x, 2)].symbol() != " ");
        assert!(cost_x.is_some());
        assert_eq!(b[(cost_x.unwrap(), floor)].symbol(), " ");
    }

    #[test]
    fn robust() {
        for (w, h) in [(0, 0), (1, 1), (3, 2), (10, 3), (80, 30)] {
            for st in [
                vec![],
                steps(),
                vec![WaterfallStep::delta("z", 0.0)],
                vec![WaterfallStep::delta("n", f64::INFINITY); 5],
                (0..100)
                    .map(|i| WaterfallStep::delta("s", f64::from(i) - 50.0))
                    .collect(),
            ] {
                let _ = draw(Waterfall::new(st).block(Block::bordered()), w, h);
            }
        }
    }
}
