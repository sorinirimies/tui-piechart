//! Gauge widget with many looks: radial dials, segmented rings, LED meters,
//! progress bars, batteries and more.
//!
//! Inspired by the dodai-ui gauge family. Radial styles are drawn with braille
//! dots (8× resolution); linear styles use block and shape glyphs.
//!
//! ```
//! use ratatui::style::Color;
//! use tui_piechart::{GaugeChart, GaugeStyle};
//!
//! let gauge = GaugeChart::new(72.0)
//!     .style(GaugeStyle::Dial)
//!     .label("CPU")
//!     .zones(true)
//!     .color(Color::Cyan);
//! ```

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::many_single_char_names,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::float_cmp
)]

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Widget};

use crate::braille::{polar, Braille};

/// How a [`GaugeChart`] draws its value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GaugeStyle {
    /// Continuous 270° arc (default).
    #[default]
    Arc,
    /// 270° ring of separate blocks.
    Segmented,
    /// Speedometer: tick ring, value arc, needle and hub.
    Dial,
    /// Top half-dial with a tick ring and a thick value arc.
    Meter,
    /// Full ring with cardinal ticks, a bright head and a dashed inner ring.
    Reticle,
    /// Full 360° donut ring starting at 12 o'clock.
    Ring,
    /// Row of slanted cells (▰▰▰▱▱).
    Bar,
    /// Row of arrow heads (❯❯❯).
    Chevron,
    /// Smooth horizontal bar with 1/8-cell resolution.
    Smooth,
    /// Row of dots (●●●○○).
    Dots,
    /// Row of LED cells colored by zone, like a VU meter.
    Led,
    /// Battery outline with a terminal nub.
    Battery,
    /// Vertical stack of cells, filling upwards.
    Level,
    /// Vertical thermometer with a bulb.
    Thermometer,
    /// Ascending signal-strength bars.
    Signal,
}

impl GaugeStyle {
    /// Every gauge style, in showcase order.
    pub const ALL: [GaugeStyle; 15] = [
        GaugeStyle::Arc,
        GaugeStyle::Segmented,
        GaugeStyle::Dial,
        GaugeStyle::Meter,
        GaugeStyle::Reticle,
        GaugeStyle::Ring,
        GaugeStyle::Bar,
        GaugeStyle::Chevron,
        GaugeStyle::Smooth,
        GaugeStyle::Dots,
        GaugeStyle::Led,
        GaugeStyle::Battery,
        GaugeStyle::Level,
        GaugeStyle::Thermometer,
        GaugeStyle::Signal,
    ];

    /// Display name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            GaugeStyle::Arc => "Arc",
            GaugeStyle::Segmented => "Segmented",
            GaugeStyle::Dial => "Dial",
            GaugeStyle::Meter => "Meter",
            GaugeStyle::Reticle => "Reticle",
            GaugeStyle::Ring => "Ring",
            GaugeStyle::Bar => "Bar",
            GaugeStyle::Chevron => "Chevron",
            GaugeStyle::Smooth => "Smooth",
            GaugeStyle::Dots => "Dots",
            GaugeStyle::Led => "Led",
            GaugeStyle::Battery => "Battery",
            GaugeStyle::Level => "Level",
            GaugeStyle::Thermometer => "Thermometer",
            GaugeStyle::Signal => "Signal",
        }
    }

    /// Whether this style is drawn with braille dots on a circle.
    #[must_use]
    pub const fn is_radial(self) -> bool {
        matches!(
            self,
            GaugeStyle::Arc
                | GaugeStyle::Segmented
                | GaugeStyle::Dial
                | GaugeStyle::Meter
                | GaugeStyle::Reticle
                | GaugeStyle::Ring
        )
    }

    /// Whether this style grows vertically.
    #[must_use]
    pub const fn is_vertical(self) -> bool {
        matches!(
            self,
            GaugeStyle::Level | GaugeStyle::Thermometer | GaugeStyle::Signal
        )
    }

    /// A good `(width, height)` in cells for this style.
    #[must_use]
    pub const fn preferred_size(self) -> (u16, u16) {
        match self {
            GaugeStyle::Meter => (22, 7),
            GaugeStyle::Level => (8, 8),
            GaugeStyle::Thermometer => (5, 8),
            GaugeStyle::Signal => (14, 6),
            s if s.is_radial() => (20, 10),
            _ => (36, 1),
        }
    }
}

/// Colour zone of a position `frac` in `0..=1`: `0` ok, `1` warning, `2` danger.
#[must_use]
pub fn zone(frac: f64) -> usize {
    if frac < 0.6 {
        0
    } else if frac < 0.85 {
        1
    } else {
        2
    }
}

/// Fill coverage `0..=1` of cell `i` of `n` when `frac` of the range is lit.
#[must_use]
pub fn cell_lit(i: usize, n: usize, frac: f64) -> f64 {
    if n == 0 {
        return 0.0;
    }
    (frac.clamp(0.0, 1.0) * n as f64 - i as f64).clamp(0.0, 1.0)
}

/// Tick positions along a sweep: `(frac, is_major)`; `major` spans each split
/// into `minor` sub-spans.
#[must_use]
pub fn tick_fracs(major: usize, minor: usize) -> Vec<(f64, bool)> {
    let (major, minor) = (major.max(1), minor.max(1));
    let total = major * minor;
    (0..=total)
        .map(|i| (i as f64 / total as f64, i % minor == 0))
        .collect()
}

const H_EIGHTHS: [char; 9] = [' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];
const V_EIGHTHS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
/// 270° sweep with the gap at the bottom: starts at 7:30, clockwise.
const SWEEP: (f64, f64) = (0.625, 0.75);

/// A gauge showing one value inside a `min..=max` range.
///
/// # Examples
///
/// ```
/// use tui_piechart::{GaugeChart, GaugeStyle};
///
/// let g = GaugeChart::new(40.0).range(0.0, 200.0).style(GaugeStyle::Led);
/// assert!((g.fraction() - 0.2).abs() < 1e-9);
/// ```
#[derive(Debug, Clone)]
pub struct GaugeChart<'a> {
    value: f64,
    min: f64,
    max: f64,
    gauge_style: GaugeStyle,
    label: Option<&'a str>,
    value_text: Option<String>,
    show_value: bool,
    zones: bool,
    zone_colors: [Color; 3],
    color: Color,
    track: Color,
    style: Style,
    block: Option<Block<'a>>,
}

impl Default for GaugeChart<'_> {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            gauge_style: GaugeStyle::default(),
            label: None,
            value_text: None,
            show_value: true,
            zones: false,
            zone_colors: [Color::Green, Color::Yellow, Color::Red],
            color: Color::Cyan,
            track: Color::DarkGray,
            style: Style::default(),
            block: None,
        }
    }
}

impl<'a> GaugeChart<'a> {
    /// Creates a gauge showing `value` in the default `0..=100` range.
    #[must_use]
    pub fn new(value: f64) -> Self {
        Self {
            value,
            ..Self::default()
        }
    }

    /// Sets the value.
    #[must_use]
    pub fn value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    /// Sets the range the value is measured in.
    #[must_use]
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    /// Chooses the look.
    #[must_use]
    pub fn style(mut self, style: GaugeStyle) -> Self {
        self.gauge_style = style;
        self
    }

    /// Sets a short label (for example `"CPU"`).
    #[must_use]
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Replaces the default `72%` readout with custom text.
    #[must_use]
    pub fn value_text(mut self, text: impl Into<String>) -> Self {
        self.value_text = Some(text.into());
        self
    }

    /// Shows or hides the readout.
    #[must_use]
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Colors by position: green below 60 %, yellow below 85 %, red above.
    #[must_use]
    pub fn zones(mut self, on: bool) -> Self {
        self.zones = on;
        self
    }

    /// Overrides the ok / warning / danger colors used by [`Self::zones`].
    #[must_use]
    pub fn zone_colors(mut self, colors: [Color; 3]) -> Self {
        self.zone_colors = colors;
        self
    }

    /// Color of the filled part (ignored while zones are on).
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Color of the unfilled track.
    #[must_use]
    pub fn track_color(mut self, color: Color) -> Self {
        self.track = color;
        self
    }

    /// Base style of the whole widget area.
    #[must_use]
    pub fn base_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Wraps the gauge in a block.
    #[must_use]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// The value as a fraction of the range, clamped to `0..=1`.
    /// Non-finite values and empty ranges give `0`.
    #[must_use]
    pub fn fraction(&self) -> f64 {
        let span = self.max - self.min;
        if !(span.is_finite() && span > 0.0 && self.value.is_finite()) {
            return 0.0;
        }
        ((self.value - self.min) / span).clamp(0.0, 1.0)
    }

    fn readout(&self) -> String {
        if !self.show_value {
            return String::new();
        }
        self.value_text
            .clone()
            .unwrap_or_else(|| format!("{:.0}%", self.fraction() * 100.0))
    }

    fn color_at(&self, pos: f64) -> Color {
        if self.zones {
            self.zone_colors[zone(pos)]
        } else {
            self.color
        }
    }

    fn lit(pos: f64, frac: f64) -> bool {
        frac >= 1.0 || pos < frac
    }

    fn fill_style(&self, pos: f64) -> Style {
        Style::default().fg(self.color_at(pos))
    }

    fn track_style(&self) -> Style {
        Style::default().fg(self.track)
    }
}

fn text_width(s: &str) -> usize {
    Span::raw(s).width()
}

fn put(buf: &mut Buffer, x: u16, y: u16, ch: char, style: Style) {
    let mut tmp = [0u8; 4];
    buf.set_string(x, y, ch.encode_utf8(&mut tmp), style);
}

fn centered(buf: &mut Buffer, area: Rect, y: u16, text: &str, style: Style) {
    let w = text_width(text);
    if text.is_empty() || w > usize::from(area.width) || y < area.y || y >= area.bottom() {
        return;
    }
    let x = area.x + ((usize::from(area.width) - w) / 2) as u16;
    buf.set_string(x, y, text, style);
}

fn eighth(table: &[char; 9], coverage: f64) -> char {
    table[(coverage.clamp(0.0, 1.0) * 8.0).round() as usize]
}

impl Widget for GaugeChart<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let inner = match &self.block {
            Some(b) => {
                let inner = b.inner(area);
                b.clone().render(area, buf);
                inner
            }
            None => area,
        };
        if inner.width == 0 || inner.height == 0 {
            return;
        }
        if self.gauge_style.is_radial() {
            self.render_radial(inner, buf);
        } else if self.gauge_style.is_vertical() {
            self.render_vertical(inner, buf);
        } else {
            self.render_linear(inner, buf);
        }
    }
}

// ───────────────────────────── radial styles ─────────────────────────────

impl GaugeChart<'_> {
    fn render_radial(&self, inner: Rect, buf: &mut Buffer) {
        let style = self.gauge_style;
        let frac = self.fraction();
        let mut g = Braille::new(inner.width, inner.height);
        let (w, h) = (g.dots_w(), g.dots_h());
        let half = style == GaugeStyle::Meter;
        let (cx, cy) = if half {
            (w / 2.0, h - 4.0)
        } else {
            (w / 2.0, h / 2.0)
        };
        let r = if half {
            (w / 2.0).min(cy)
        } else {
            w.min(h) / 2.0
        } - 0.5;
        let readout = self.readout();
        let label = self.label.unwrap_or("");
        let bold = Style::default().add_modifier(Modifier::BOLD);

        if r >= 2.0 {
            let c = (cx, cy);
            match style {
                GaugeStyle::Arc => self.arc(&mut g, c, r, SWEEP, frac),
                GaugeStyle::Ring => self.arc(&mut g, c, r, (0.0, 1.0), frac),
                GaugeStyle::Segmented => self.segmented(&mut g, c, r, frac),
                GaugeStyle::Dial => self.dial(&mut g, c, r, SWEEP, frac, true),
                GaugeStyle::Meter => self.dial(&mut g, c, r, (0.75, 0.5), frac, false),
                _ => self.reticle(&mut g, c, r, frac),
            }
            g.blit(inner, buf);
        }

        match style {
            GaugeStyle::Dial | GaugeStyle::Meter => {
                let line = [readout.as_str(), label]
                    .iter()
                    .filter(|s| !s.is_empty())
                    .copied()
                    .collect::<Vec<_>>()
                    .join(" ");
                centered(
                    buf,
                    inner,
                    inner.bottom() - 1,
                    &line,
                    bold.fg(self.color_at(frac)),
                );
            }
            _ => {
                // Text only when it fits inside the hole.
                let hole = (r * 0.7) as usize;
                let mid = inner.y + inner.height / 2;
                if text_width(&readout) <= hole.max(1) {
                    centered(buf, inner, mid, &readout, bold.fg(self.color_at(frac)));
                }
                if text_width(label) <= hole.max(1) && inner.height >= 5 {
                    centered(buf, inner, mid + 1, label, self.track_style());
                }
            }
        }
    }

    fn thickness(r: f64) -> f64 {
        (r * 0.2).max(2.0)
    }

    fn arc(&self, g: &mut Braille, c: (f64, f64), r: f64, sweep: (f64, f64), frac: f64) {
        let t = Self::thickness(r);
        g.ring(c, (r - t, r), sweep, |pos| {
            Some(if Self::lit(pos, frac) {
                (self.color_at(pos), 1)
            } else {
                (self.track, 0)
            })
        });
    }

    fn segmented(&self, g: &mut Braille, c: (f64, f64), r: f64, frac: f64) {
        let t = Self::thickness(r);
        let n = ((r * 0.7) as usize).clamp(8, 24) as f64;
        g.ring(c, (r - t, r), SWEEP, |pos| {
            let seg = pos * n;
            let local = seg - seg.floor();
            if !(0.2..=0.8).contains(&local) {
                return None;
            }
            let centre = (seg.floor() + 0.5) / n;
            Some(if Self::lit(pos, frac) {
                (self.color_at(centre), 1)
            } else {
                (self.track, 0)
            })
        });
    }

    fn dial(
        &self,
        g: &mut Braille,
        c: (f64, f64),
        r: f64,
        sweep: (f64, f64),
        frac: f64,
        needle: bool,
    ) {
        let (start, len) = sweep;
        // Tick ring on the outside.
        for (pos, major) in tick_fracs(5, 4) {
            let turn = start + pos * len;
            let tl = if major { 3.0 } else { 1.5 };
            let lit = Self::lit(pos, frac) || (pos - frac).abs() < 1e-9;
            let (col, p) = if lit {
                (self.color_at(pos), 1)
            } else {
                (self.track, 0)
            };
            g.line(
                polar(c.0, c.1, r, turn),
                polar(c.0, c.1, r - tl, turn),
                col,
                p,
            );
        }
        let outer = r - 4.5;
        let t = if needle {
            Self::thickness(r) * 0.7
        } else {
            Self::thickness(r) * 1.1
        };
        let inner = (outer - t).max(0.0);
        self.arc_band(g, c, (inner, outer), sweep, frac);
        if needle {
            let tone = self.color_at(frac);
            g.line(
                c,
                polar(c.0, c.1, (inner - 1.0).max(1.0), start + frac * len),
                tone,
                2,
            );
            g.ring(c, (0.0, 2.5), (0.0, 1.0), |_| Some((tone, 2)));
        }
    }

    fn arc_band(
        &self,
        g: &mut Braille,
        c: (f64, f64),
        band: (f64, f64),
        sweep: (f64, f64),
        frac: f64,
    ) {
        g.ring(c, band, sweep, |pos| {
            Some(if Self::lit(pos, frac) {
                (self.color_at(pos), 1)
            } else {
                (self.track, 0)
            })
        });
    }

    fn reticle(&self, g: &mut Braille, c: (f64, f64), r: f64, frac: f64) {
        let t = 2.0_f64.min(r / 3.0);
        g.ring(c, (r - t, r), (0.0, 1.0), |pos| {
            if Self::lit(pos, frac) && frac < 1.0 && (frac - pos) < 0.025 {
                return Some((Color::White, 2)); // bright head
            }
            Some(if Self::lit(pos, frac) {
                (self.color_at(pos), 1)
            } else {
                (self.track, 0)
            })
        });
        for q in 0..4 {
            let turn = f64::from(q) * 0.25;
            g.line(
                polar(c.0, c.1, r - t - 1.0, turn),
                polar(c.0, c.1, r - t - 4.0, turn),
                self.track,
                1,
            );
        }
        // Dashed inner ring.
        let ri = r * 0.55;
        if ri > 3.0 {
            g.ring(c, (ri - 1.5, ri), (0.0, 1.0), |pos| {
                ((pos * 24.0).fract() < 0.5).then_some((self.track, 0))
            });
        }
    }
}

// ───────────────────────────── linear styles ─────────────────────────────

impl GaugeChart<'_> {
    fn render_linear(&self, inner: Rect, buf: &mut Buffer) {
        let y = inner.y + (inner.height - 1) / 2;
        let label = self.label.unwrap_or("");
        let value = self.readout();
        let total = usize::from(inner.width);
        let mut lw = if label.is_empty() {
            0
        } else {
            text_width(label) + 1
        };
        let mut vw = if value.is_empty() {
            0
        } else {
            text_width(&value) + 1
        };
        if total < lw + vw + 3 {
            lw = 0;
            vw = 0;
        }
        let bw = total - lw - vw;
        let x = inner.x;
        if lw > 0 {
            buf.set_string(x, y, label, self.track_style());
        }
        let bar_x = x + lw as u16;
        let frac = self.fraction();
        match self.gauge_style {
            GaugeStyle::Bar => self.cells(buf, (bar_x, y, bw), frac, ('▰', '▱'), 1, 1),
            GaugeStyle::Chevron => self.cells(buf, (bar_x, y, bw), frac, ('❯', '❯'), 2, 1),
            GaugeStyle::Dots => self.cells(buf, (bar_x, y, bw), frac, ('●', '○'), 2, 1),
            GaugeStyle::Led => self.cells(buf, (bar_x, y, bw), frac, ('▮', '▯'), 2, 1),
            GaugeStyle::Battery => self.battery(buf, (bar_x, y, bw), frac),
            _ => self.smooth(buf, (bar_x, y, bw), frac),
        }
        if vw > 0 {
            let vx = bar_x + bw as u16 + 1;
            let style = Style::default()
                .fg(self.color_at(frac))
                .add_modifier(Modifier::BOLD);
            buf.set_string(vx, y, &value, style);
        }
    }

    /// Row of equal cells; `pitch` columns per cell, the last `pitch - used` blank.
    fn cells(
        &self,
        buf: &mut Buffer,
        (x, y, w): (u16, u16, usize),
        frac: f64,
        glyphs: (char, char),
        pitch: usize,
        _used: usize,
    ) {
        let n = w / pitch;
        for i in 0..n {
            let pos = (i as f64 + 0.5) / n as f64;
            let lit = cell_lit(i, n, frac) >= 0.5;
            let cx = x + (i * pitch) as u16;
            if lit {
                put(buf, cx, y, glyphs.0, self.fill_style(pos));
            } else {
                put(buf, cx, y, glyphs.1, self.track_style());
            }
        }
    }

    fn smooth(&self, buf: &mut Buffer, (x, y, w): (u16, u16, usize), frac: f64) {
        for i in 0..w {
            let pos = (i as f64 + 0.5) / w as f64;
            let cov = cell_lit(i, w, frac);
            let cx = x + i as u16;
            if cov >= 1.0 - 1e-9 {
                put(buf, cx, y, '█', self.fill_style(pos));
            } else if cov > 0.0 {
                put(buf, cx, y, eighth(&H_EIGHTHS, cov), self.fill_style(pos));
            } else {
                put(buf, cx, y, '░', self.track_style());
            }
        }
    }

    fn battery(&self, buf: &mut Buffer, (x, y, w): (u16, u16, usize), frac: f64) {
        if w < 5 {
            return self.smooth(buf, (x, y, w), frac);
        }
        let body = w - 3; // '[' ... ']' '▪'
        let edge = self.track_style();
        put(buf, x, y, '[', edge);
        put(buf, x + 1 + body as u16, y, ']', edge);
        put(buf, x + 2 + body as u16, y, '▪', edge);
        self.smooth(buf, (x + 1, y, body), frac);
    }
}

// ───────────────────────────── vertical styles ─────────────────────────────

impl GaugeChart<'_> {
    fn render_vertical(&self, inner: Rect, buf: &mut Buffer) {
        let frac = self.fraction();
        let readout = self.readout();
        let label = self.label.unwrap_or("");
        let line = [readout.as_str(), label]
            .iter()
            .filter(|s| !s.is_empty())
            .copied()
            .collect::<Vec<_>>()
            .join(" ");
        let text_row = inner.height >= 3 && !line.is_empty();
        let rows = inner.height - u16::from(text_row);
        let width = usize::from(inner.width);
        match self.gauge_style {
            GaugeStyle::Level => self.level(buf, inner, rows, width.min(6), frac),
            GaugeStyle::Thermometer => self.thermometer(buf, inner, rows, frac),
            _ => self.signal(buf, inner, rows, frac),
        }
        if text_row {
            let style = Style::default()
                .fg(self.color_at(frac))
                .add_modifier(Modifier::BOLD);
            centered(buf, inner, inner.bottom() - 1, &line, style);
        }
    }

    fn level(&self, buf: &mut Buffer, inner: Rect, rows: u16, w: usize, frac: f64) {
        let x0 = inner.x + ((usize::from(inner.width) - w) / 2) as u16;
        let n = usize::from(rows);
        for i in 0..n {
            let y = inner.y + rows - 1 - i as u16;
            let cov = cell_lit(i, n, frac);
            let pos = (i as f64 + 0.5) / n as f64;
            let (ch, style) = if cov >= 1.0 - 1e-9 {
                ('▇', self.fill_style(pos))
            } else if cov > 0.0 {
                (eighth(&V_EIGHTHS, cov * 0.875), self.fill_style(pos))
            } else {
                ('▇', self.track_style())
            };
            for dx in 0..w {
                put(buf, x0 + dx as u16, y, ch, style);
            }
        }
    }

    fn thermometer(&self, buf: &mut Buffer, inner: Rect, rows: u16, frac: f64) {
        if rows < 2 {
            return;
        }
        let x = inner.x + inner.width / 2;
        let stem = usize::from(rows - 1);
        for i in 0..stem {
            let y = inner.y + rows - 2 - i as u16;
            let cov = cell_lit(i, stem, frac);
            let pos = (i as f64 + 0.5) / stem as f64;
            if cov >= 1.0 - 1e-9 {
                put(buf, x, y, '█', self.fill_style(pos));
            } else if cov > 0.0 {
                put(buf, x, y, eighth(&V_EIGHTHS, cov), self.fill_style(pos));
            } else {
                put(buf, x, y, '│', self.track_style());
            }
        }
        let bulb = if frac > 0.0 {
            self.fill_style(0.0)
        } else {
            self.track_style()
        };
        put(buf, x, inner.y + rows - 1, '●', bulb);
    }

    fn signal(&self, buf: &mut Buffer, inner: Rect, rows: u16, frac: f64) {
        let n = 5usize;
        let bw: usize = if usize::from(inner.width) >= n * 3 - 1 {
            2
        } else {
            1
        };
        let total = n * (bw + 1) - 1;
        if total > usize::from(inner.width) {
            return;
        }
        let x0 = inner.x + ((usize::from(inner.width) - total) / 2) as u16;
        for i in 0..n {
            let height = ((i + 1) * usize::from(rows)).div_ceil(n);
            let lit = cell_lit(i, n, frac) > 0.0;
            let pos = (i as f64 + 0.5) / n as f64;
            let style = if lit {
                self.fill_style(pos)
            } else {
                self.track_style()
            };
            for r in 0..height {
                let y = inner.y + rows - 1 - r as u16;
                for dx in 0..bw {
                    put(buf, x0 + (i * (bw + 1) + dx) as u16, y, '█', style);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(g: GaugeChart<'_>, w: u16, h: u16) -> Buffer {
        let area = Rect::new(0, 0, w, h);
        let mut buf = Buffer::empty(area);
        g.render(area, &mut buf);
        buf
    }

    fn text(buf: &Buffer) -> String {
        buf.content()
            .iter()
            .map(ratatui::buffer::Cell::symbol)
            .collect()
    }

    #[test]
    fn fraction_clamps_and_guards() {
        assert_eq!(GaugeChart::new(150.0).fraction(), 1.0);
        assert_eq!(GaugeChart::new(-5.0).fraction(), 0.0);
        assert_eq!(GaugeChart::new(f64::NAN).fraction(), 0.0);
        assert_eq!(GaugeChart::new(f64::INFINITY).fraction(), 0.0);
        assert_eq!(GaugeChart::new(5.0).range(10.0, 10.0).fraction(), 0.0);
        assert_eq!(GaugeChart::new(5.0).range(10.0, 0.0).fraction(), 0.0);
        assert!((GaugeChart::new(25.0).range(0.0, 50.0).fraction() - 0.5).abs() < 1e-12);
    }

    #[test]
    fn zones_thresholds() {
        assert_eq!(zone(0.0), 0);
        assert_eq!(zone(0.6), 1);
        assert_eq!(zone(0.84), 1);
        assert_eq!(zone(0.85), 2);
    }

    #[test]
    fn cell_lit_partial() {
        assert_eq!(cell_lit(0, 4, 0.5), 1.0);
        assert_eq!(cell_lit(2, 4, 0.5), 0.0);
        assert!((cell_lit(1, 4, 0.375) - 0.5).abs() < 1e-12);
        assert_eq!(cell_lit(0, 0, 1.0), 0.0);
    }

    #[test]
    fn tick_counts() {
        let t = tick_fracs(5, 4);
        assert_eq!(t.len(), 21);
        assert!(t[0].1 && t[4].1 && !t[1].1);
        assert_eq!(tick_fracs(0, 0).len(), 2);
    }

    #[test]
    fn all_styles_render_at_all_sizes_without_panic() {
        for style in GaugeStyle::ALL {
            for (w, h) in [
                (0, 0),
                (1, 1),
                (2, 1),
                (3, 2),
                (5, 3),
                (20, 10),
                (40, 1),
                (60, 30),
            ] {
                for v in [0.0, 50.0, 100.0, f64::NAN] {
                    let g = GaugeChart::new(v).style(style).label("CPU").zones(true);
                    let _ = render(g, w, h);
                }
            }
        }
    }

    #[test]
    fn rendering_into_offset_area_stays_inside() {
        for style in GaugeStyle::ALL {
            let area = Rect::new(3, 2, 20, 8);
            let mut buf = Buffer::empty(Rect::new(0, 0, 30, 14));
            GaugeChart::new(70.0).style(style).render(area, &mut buf);
            for (i, c) in buf.content().iter().enumerate() {
                let (x, y) = ((i % 30) as u16, (i / 30) as u16);
                if !area.contains((x, y).into()) {
                    assert_eq!(c.symbol(), " ", "{style:?} drew outside at {x},{y}");
                }
            }
        }
    }

    #[test]
    fn more_value_lights_more_dots() {
        let count = |s: &str| s.chars().filter(|c| *c != ' ' && *c != '\u{2800}').count();
        for style in GaugeStyle::ALL {
            let (w, h) = style.preferred_size();
            let draw = |v: f64| render(GaugeChart::new(v).style(style).show_value(false), w, h);
            let look = |b: &Buffer| -> Vec<_> {
                b.content()
                    .iter()
                    .map(|c| (c.symbol().to_string(), c.fg))
                    .collect()
            };
            let (lo, hi) = (draw(10.0), draw(90.0));
            assert_ne!(
                look(&lo),
                look(&hi),
                "{style:?} does not react to the value"
            );
            let (lo, hi) = (text(&lo), text(&hi));
            assert!(count(&hi) >= count(&lo) || style.is_radial());
        }
    }

    #[test]
    fn readout_and_label_are_drawn() {
        let t = text(&render(
            GaugeChart::new(72.0).style(GaugeStyle::Bar).label("CPU"),
            40,
            1,
        ));
        assert!(t.contains("CPU") && t.contains("72%"));
        let t = text(&render(
            GaugeChart::new(72.0).style(GaugeStyle::Dial).label("CPU"),
            24,
            12,
        ));
        assert!(t.contains("72%") && t.contains("CPU"));
        let t = text(&render(
            GaugeChart::new(5.0)
                .style(GaugeStyle::Smooth)
                .value_text("5 of 10"),
            30,
            1,
        ));
        assert!(t.contains("5 of 10"));
    }

    #[test]
    fn show_value_false_hides_readout() {
        let t = text(&render(
            GaugeChart::new(72.0)
                .style(GaugeStyle::Smooth)
                .show_value(false),
            30,
            1,
        ));
        assert!(!t.contains('%'));
    }

    #[test]
    fn zone_colors_apply() {
        let buf = render(
            GaugeChart::new(100.0)
                .style(GaugeStyle::Led)
                .zones(true)
                .show_value(false),
            40,
            1,
        );
        let fg: Vec<_> = buf.content().iter().map(|c| c.fg).collect();
        assert!(
            fg.contains(&Color::Green) && fg.contains(&Color::Yellow) && fg.contains(&Color::Red)
        );
    }

    #[test]
    fn block_is_rendered() {
        let t = text(&render(
            GaugeChart::new(50.0).block(Block::bordered().title("Load")),
            20,
            6,
        ));
        assert!(t.contains("Load"));
    }

    #[test]
    fn full_smooth_bar_is_solid() {
        let t = text(&render(
            GaugeChart::new(100.0)
                .style(GaugeStyle::Smooth)
                .show_value(false),
            10,
            1,
        ));
        assert_eq!(t, "██████████");
        let t = text(&render(
            GaugeChart::new(0.0)
                .style(GaugeStyle::Smooth)
                .show_value(false),
            10,
            1,
        ));
        assert_eq!(t, "░░░░░░░░░░");
    }

    #[test]
    fn style_metadata() {
        assert_eq!(GaugeStyle::ALL.len(), 15);
        for s in GaugeStyle::ALL {
            assert_ne!(s.name(), "");
            assert!(!(s.is_radial() && s.is_vertical()));
        }
    }
}
