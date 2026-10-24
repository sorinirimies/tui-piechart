//! Concentric progress rings: [`RadialBar`] (value / max per ring) and
//! [`RingStack`] (percentages, always a 270° sweep).

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

use crate::braille::Braille;
use crate::util::{block_inner, palette, text_width, truncate};

/// One ring of a [`RadialBar`].
#[derive(Debug, Clone, PartialEq)]
pub struct RadialRing<'a> {
    label: &'a str,
    value: f64,
    max: f64,
    color: Option<Color>,
}

impl<'a> RadialRing<'a> {
    /// A ring showing `value` out of `max`.
    #[must_use]
    pub const fn new(label: &'a str, value: f64, max: f64) -> Self {
        Self {
            label,
            value,
            max,
            color: None,
        }
    }

    /// Fixes the ring color (default: the palette, by position).
    #[must_use]
    pub const fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Fraction of the ring that is filled, `0..=1`.
    #[must_use]
    pub fn fraction(&self) -> f64 {
        if self.max.is_finite() && self.max > 0.0 && self.value.is_finite() {
            (self.value / self.max).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
}

/// Sweep of a [`RadialBar`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RadialSweep {
    /// 270° with the gap at the bottom (default).
    #[default]
    Arc,
    /// Full 360° rings starting at 12 o'clock.
    Full,
}

/// Concentric progress rings, one per [`RadialRing`], with an optional legend.
///
/// # Examples
///
/// ```
/// use tui_piechart::{RadialBar, RadialRing};
///
/// let chart = RadialBar::new(vec![
///     RadialRing::new("Move", 420.0, 600.0),
///     RadialRing::new("Exercise", 25.0, 30.0),
///     RadialRing::new("Stand", 9.0, 12.0),
/// ]);
/// ```
#[derive(Debug, Clone)]
pub struct RadialBar<'a> {
    rings: Vec<RadialRing<'a>>,
    sweep: RadialSweep,
    legend: bool,
    track: Color,
    block: Option<Block<'a>>,
    style: Style,
}

impl<'a> RadialBar<'a> {
    /// Creates a chart from rings, outermost first.
    #[must_use]
    pub fn new(rings: Vec<RadialRing<'a>>) -> Self {
        Self {
            rings,
            sweep: RadialSweep::default(),
            legend: true,
            track: Color::DarkGray,
            block: None,
            style: Style::default(),
        }
    }

    /// Chooses a 270° or full sweep.
    #[must_use]
    pub const fn sweep(mut self, sweep: RadialSweep) -> Self {
        self.sweep = sweep;
        self
    }

    /// Shows or hides the legend (shown by default when there is room).
    #[must_use]
    pub const fn legend(mut self, show: bool) -> Self {
        self.legend = show;
        self
    }

    /// Color of the unfilled track.
    #[must_use]
    pub const fn track_color(mut self, color: Color) -> Self {
        self.track = color;
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
        self.style = style;
        self
    }
}

fn ring_color(r: &RadialRing<'_>, i: usize) -> Color {
    r.color.unwrap_or_else(|| palette(i))
}

fn draw_rings(
    buf: &mut Buffer,
    area: Rect,
    rings: &[RadialRing<'_>],
    sweep: RadialSweep,
    track: Color,
) {
    if rings.is_empty() || area.is_empty() {
        return;
    }
    let mut g = Braille::new(area.width, area.height);
    let (w, h) = (g.dots_w(), g.dots_h());
    let c = (w / 2.0, h / 2.0);
    let r = w.min(h) / 2.0 - 0.5;
    let n = rings.len() as f64;
    let pitch = (r * 0.85) / n;
    if pitch >= 1.5 {
        let thick = (pitch - 1.2).max(1.5).min(pitch);
        let sw = match sweep {
            RadialSweep::Arc => (0.625, 0.75),
            RadialSweep::Full => (0.0, 1.0),
        };
        for (i, ring) in rings.iter().enumerate() {
            let outer = r - i as f64 * pitch;
            let inner = (outer - thick).max(0.0);
            let frac = ring.fraction();
            let color = ring_color(ring, i);
            g.ring(c, (inner, outer), sw, |pos| {
                Some(if frac >= 1.0 || pos < frac {
                    (color, 1)
                } else {
                    (track, 0)
                })
            });
        }
    }
    g.blit(area, buf);
}

fn draw_legend(buf: &mut Buffer, area: Rect, rings: &[RadialRing<'_>]) {
    let label_w = area.width.saturating_sub(7) as usize;
    for (i, ring) in rings.iter().enumerate() {
        let y = area.y + i as u16;
        if y >= area.bottom() {
            break;
        }
        let color = ring_color(ring, i);
        buf.set_string(area.x, y, "■", Style::default().fg(color));
        let label = truncate(ring.label, label_w);
        buf.set_string(area.x + 2, y, &label, Style::default());
        let pct = format!("{:.0}%", ring.fraction() * 100.0);
        let px = area.right().saturating_sub(pct.len() as u16);
        if px > area.x + 2 + text_width(&label) as u16 {
            buf.set_string(
                px,
                y,
                &pct,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            );
        }
    }
}

/// Splits `inner` into the rings area and an optional legend area.
fn split(inner: Rect, rings: &[RadialRing<'_>], legend: bool) -> (Rect, Option<Rect>) {
    if !legend || rings.is_empty() {
        return (inner, None);
    }
    let longest = rings.iter().map(|r| text_width(r.label)).max().unwrap_or(0);
    let lw = (longest + 8).clamp(14, 32) as u16;
    // Keep at least a 12-column chart.
    if inner.width < lw + 12 {
        return (inner, None);
    }
    let chart_w = inner.width - lw - 1;
    let chart = Rect::new(inner.x, inner.y, chart_w, inner.height);
    let ly = inner.y + inner.height.saturating_sub(rings.len() as u16) / 2;
    let leg = Rect::new(
        inner.x + chart_w + 1,
        ly,
        lw,
        (rings.len() as u16).min(inner.bottom() - ly),
    );
    (chart, Some(leg))
}

impl Widget for RadialBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let inner = block_inner(self.block.as_ref(), area, buf);
        let (chart, legend) = split(inner, &self.rings, self.legend);
        draw_rings(buf, chart, &self.rings, self.sweep, self.track);
        if let Some(l) = legend {
            draw_legend(buf, l, &self.rings);
        }
    }
}

/// Concentric 270° rings, one per `(label, percent)`, with a legend.
///
/// A convenience over [`RadialBar`] for percentage readouts.
///
/// # Examples
///
/// ```
/// use tui_piechart::RingStack;
///
/// let stack = RingStack::new(vec![("CPU", 72.0), ("RAM", 48.0), ("Disk", 91.0)]);
/// ```
#[derive(Debug, Clone)]
pub struct RingStack<'a> {
    items: Vec<(&'a str, f64)>,
    colors: Vec<Color>,
    inner: RadialBar<'a>,
}

impl<'a> RingStack<'a> {
    /// Creates a ring stack from `(label, percent)` pairs, outermost first.
    #[must_use]
    pub fn new(items: Vec<(&'a str, f64)>) -> Self {
        Self {
            items,
            colors: Vec::new(),
            inner: RadialBar::new(Vec::new()),
        }
    }

    /// Ring colors, by position (the palette fills in the rest).
    #[must_use]
    pub fn colors(mut self, colors: Vec<Color>) -> Self {
        self.colors = colors;
        self
    }

    /// Shows or hides the legend.
    #[must_use]
    pub fn legend(mut self, show: bool) -> Self {
        self.inner = self.inner.legend(show);
        self
    }

    /// Wraps the chart in a block.
    #[must_use]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.inner = self.inner.block(block);
        self
    }
}

impl Widget for RingStack<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rings = self
            .items
            .iter()
            .enumerate()
            .map(|(i, (label, pct))| {
                RadialRing::new(label, *pct, 100.0)
                    .color(self.colors.get(i).copied().unwrap_or_else(|| palette(i)))
            })
            .collect();
        RadialBar {
            rings,
            ..self.inner
        }
        .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::assert_is_empty, clippy::float_cmp)]
    use super::*;

    fn draw(w: impl Widget, width: u16, height: u16) -> Buffer {
        let a = Rect::new(0, 0, width, height);
        let mut b = Buffer::empty(a);
        w.render(a, &mut b);
        b
    }

    fn text(b: &Buffer) -> String {
        b.content()
            .iter()
            .map(ratatui::buffer::Cell::symbol)
            .collect()
    }

    fn rings() -> Vec<RadialRing<'static>> {
        vec![
            RadialRing::new("Move", 420.0, 600.0),
            RadialRing::new("Exercise", 25.0, 30.0).color(Color::Red),
            RadialRing::new("Stand", 9.0, 12.0),
        ]
    }

    #[test]
    fn fraction_guards() {
        assert_eq!(RadialRing::new("a", 5.0, 0.0).fraction(), 0.0);
        assert_eq!(RadialRing::new("a", f64::NAN, 10.0).fraction(), 0.0);
        assert_eq!(RadialRing::new("a", 50.0, 10.0).fraction(), 1.0);
        assert_eq!(RadialRing::new("a", -5.0, 10.0).fraction(), 0.0);
        assert!((RadialRing::new("a", 3.0, 12.0).fraction() - 0.25).abs() < 1e-12);
    }

    #[test]
    fn renders_rings_and_legend() {
        let b = draw(RadialBar::new(rings()), 50, 14);
        let t = text(&b);
        assert!(t.contains("Move") && t.contains("Exercise") && t.contains("70%"));
        assert!(b
            .content()
            .iter()
            .any(|c| c.fg == Color::Red && c.symbol() != " "));
    }

    #[test]
    fn legend_can_be_hidden() {
        let t = text(&draw(RadialBar::new(rings()).legend(false), 50, 14));
        assert!(!t.contains("Move"));
    }

    #[test]
    fn full_sweep_differs_from_arc() {
        let a = draw(RadialBar::new(rings()).legend(false), 30, 14);
        let b = draw(
            RadialBar::new(rings())
                .legend(false)
                .sweep(RadialSweep::Full),
            30,
            14,
        );
        assert_ne!(a, b);
    }

    #[test]
    fn value_changes_picture() {
        let lo = draw(
            RadialBar::new(vec![RadialRing::new("a", 10.0, 100.0)]).legend(false),
            20,
            10,
        );
        let hi = draw(
            RadialBar::new(vec![RadialRing::new("a", 90.0, 100.0)]).legend(false),
            20,
            10,
        );
        assert_ne!(lo, hi);
    }

    #[test]
    fn robust_to_odd_sizes_and_input() {
        for (w, h) in [(0, 0), (1, 1), (4, 2), (13, 3), (60, 30)] {
            let _ = draw(RadialBar::new(rings()), w, h);
            let _ = draw(RadialBar::new(vec![]), w, h);
            let _ = draw(
                RadialBar::new(vec![RadialRing::new("x", f64::NAN, f64::NAN)]),
                w,
                h,
            );
            let many = (0..40)
                .map(|i| RadialRing::new("r", f64::from(i), 40.0))
                .collect();
            let _ = draw(RadialBar::new(many), w, h);
        }
    }

    #[test]
    fn ring_stack_renders_percentages() {
        let t = text(&draw(
            RingStack::new(vec![("CPU", 72.0), ("RAM", 48.0)])
                .block(Block::bordered().title("Sys")),
            50,
            14,
        ));
        assert!(t.contains("CPU") && t.contains("72%") && t.contains("Sys"));
    }

    #[test]
    fn ring_stack_custom_colors() {
        let b = draw(
            RingStack::new(vec![("A", 100.0)])
                .colors(vec![Color::Magenta])
                .legend(false),
            20,
            10,
        );
        assert!(b
            .content()
            .iter()
            .any(|c| c.fg == Color::Magenta && c.symbol() != " "));
    }
}
