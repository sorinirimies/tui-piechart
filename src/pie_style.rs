//! Pie looks beyond the plain disc: segmented ring, HUD band and rose (polar area),
//! plus gaps and exploded slices. All drawn with braille dots.

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::many_single_char_names,
    clippy::similar_names,
    clippy::too_many_lines
)]

use std::f64::consts::TAU;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;

use crate::braille::{polar, Braille};
use crate::PieChart;

/// How a [`PieChart`] draws its slices.
///
/// Everything except [`PieStyle::Solid`] is drawn with braille dots.
///
/// # Examples
///
/// ```
/// use tui_piechart::{PieChart, PieStyle};
///
/// let chart = PieChart::default().pie_style(PieStyle::Hud).center("CPU");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PieStyle {
    /// Solid wedges (default); honours `hole`, `start_angle` and the resolution.
    #[default]
    Solid,
    /// A ring built from separate cells, like a segmented readout.
    Segmented,
    /// Thin band between an outer tick scale and a dashed inner ring.
    Hud,
    /// Equal angles, radius follows the value (a polar-area / rose chart).
    Rose,
}

impl PieStyle {
    /// Every style, in showcase order.
    pub const ALL: [PieStyle; 4] = [
        PieStyle::Solid,
        PieStyle::Segmented,
        PieStyle::Hud,
        PieStyle::Rose,
    ];

    /// Display name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            PieStyle::Solid => "Solid",
            PieStyle::Segmented => "Segmented",
            PieStyle::Hud => "Hud",
            PieStyle::Rose => "Rose",
        }
    }
}

const GUIDE: Color = Color::DarkGray;

impl PieChart<'_> {
    /// Whether the braille canvas renderer must be used.
    pub(crate) fn needs_canvas(&self) -> bool {
        self.pie_style != PieStyle::Solid || self.gap > 0.0 || self.explode > 0.0
    }

    /// Draws the donut-hole text, if any.
    pub(crate) fn draw_center(&self, area: Rect, buf: &mut Buffer) {
        let Some(text) = self.center else { return };
        let has_hole =
            self.hole > 0.0 || matches!(self.pie_style, PieStyle::Segmented | PieStyle::Hud);
        let w = Span::raw(text).width();
        if !has_hole || text.is_empty() || area.is_empty() || w > usize::from(area.width) {
            return;
        }
        let x = area.x + ((usize::from(area.width) - w) / 2) as u16;
        let y = area.y + area.height / 2;
        buf.set_string(x, y, text, Style::default().add_modifier(Modifier::BOLD));
    }

    /// `(start_turn, len_turns, color, value)` for every slice with a positive share.
    fn arcs(&self) -> Vec<(f64, f64, Color, f64)> {
        let mut at = self.start;
        let mut out = Vec::with_capacity(self.slices.len());
        for s in &self.slices {
            let len = self.percentage(s) / 100.0;
            if len > 0.0 {
                out.push((at, len, s.color, s.value));
            }
            at += len;
        }
        out
    }

    /// True when a dot at `dist` on the arc `pos` (0..1) of a `len`-turn slice lies in the gap.
    fn in_gap(&self, pos: f64, len: f64, dist: f64) -> bool {
        if self.gap <= 0.0 || len >= 0.999 {
            return false;
        }
        let half = self.gap / 2.0;
        let arc = len * TAU * dist;
        pos * arc < half || (1.0 - pos) * arc < half
    }

    pub(crate) fn render_piechart_canvas(&self, area: Rect, buf: &mut Buffer) {
        let (pie_area, legend_area) = self.calculate_layout(area);
        if !pie_area.is_empty() {
            let mut g = Braille::new(pie_area.width, pie_area.height);
            let (cx, cy) = (g.dots_w() / 2.0, g.dots_h() / 2.0);
            let r = cx.min(cy) - 1.0 - self.explode;
            if r >= 2.0 {
                let arcs = self.arcs();
                match self.pie_style {
                    PieStyle::Solid => self.canvas_solid(&mut g, (cx, cy), r, &arcs),
                    PieStyle::Segmented => self.canvas_segmented(&mut g, (cx, cy), r, &arcs),
                    PieStyle::Hud => self.canvas_hud(&mut g, (cx, cy), r, &arcs),
                    PieStyle::Rose => self.canvas_rose(&mut g, (cx, cy), r),
                }
                g.blit(pie_area, buf);
            }
            self.draw_center(pie_area, buf);
        }
        if let Some(legend_area) = legend_area {
            self.render_legend(buf, legend_area);
        }
    }

    fn slice_center(&self, c: (f64, f64), start: f64, len: f64) -> (f64, f64) {
        if self.explode > 0.0 && len < 0.999 {
            polar(c.0, c.1, self.explode, start + len / 2.0)
        } else {
            c
        }
    }

    fn canvas_solid(
        &self,
        g: &mut Braille,
        c: (f64, f64),
        r: f64,
        arcs: &[(f64, f64, Color, f64)],
    ) {
        for &(start, len, color, _) in arcs {
            let sc = self.slice_center(c, start, len);
            g.ring_dist(sc, (self.hole * r, r), (start, len), |pos, d| {
                (!self.in_gap(pos, len, d)).then_some((color, 1))
            });
        }
    }

    fn canvas_segmented(
        &self,
        g: &mut Braille,
        c: (f64, f64),
        r: f64,
        arcs: &[(f64, f64, Color, f64)],
    ) {
        let inner = self.hole.max(0.55) * r;
        let cells = ((r * 1.5) as usize).clamp(24, 72) as f64;
        for &(start, len, color, _) in arcs {
            let sc = self.slice_center(c, start, len);
            g.ring(sc, (inner, r), (start, len), |pos| {
                let turn = (start + pos * len) * cells;
                let local = turn - turn.floor();
                (0.15..=0.85).contains(&local).then_some((color, 1))
            });
        }
    }

    fn canvas_hud(&self, g: &mut Braille, c: (f64, f64), r: f64, arcs: &[(f64, f64, Color, f64)]) {
        // Outer tick scale.
        for k in 0..60 {
            let turn = f64::from(k) / 60.0;
            let len = if k % 5 == 0 { 3.0 } else { 1.5 };
            g.line(
                polar(c.0, c.1, r, turn),
                polar(c.0, c.1, r - len, turn),
                GUIDE,
                0,
            );
        }
        // Thin colored band.
        let band_out = (r - 4.5).max(2.0);
        let band_in = (band_out - 3.0).max(0.0);
        for &(start, len, color, _) in arcs {
            let sc = self.slice_center(c, start, len);
            g.ring_dist(sc, (band_in, band_out), (start, len), |pos, d| {
                (!self.in_gap(pos, len, d)).then_some((color, 1))
            });
        }
        // Dashed inner ring.
        let dash = band_in - 4.0;
        if dash > 3.0 {
            g.ring(c, (dash - 1.5, dash), (0.0, 1.0), |pos| {
                ((pos * 36.0).fract() < 0.5).then_some((GUIDE, 0))
            });
        }
    }

    fn canvas_rose(&self, g: &mut Braille, c: (f64, f64), r: f64) {
        let shown: Vec<_> = self.slices.iter().filter(|s| s.value > 0.0).collect();
        if shown.is_empty() {
            return;
        }
        let max = shown.iter().map(|s| s.value).fold(0.0_f64, f64::max);
        // Guide rings.
        for k in 1..=3 {
            let rad = r * f64::from(k) / 3.0;
            g.ring(c, (rad - 1.5, rad), (0.0, 1.0), |pos| {
                ((pos * 48.0).fract() < 0.35).then_some((GUIDE, 0))
            });
        }
        let len = 1.0 / shown.len() as f64;
        for (i, s) in shown.iter().enumerate() {
            let start = self.start + i as f64 * len;
            let rad = r * (s.value / max).sqrt();
            let sc = self.slice_center(c, start, len);
            g.ring_dist(sc, (self.hole * rad, rad), (start, len), |pos, d| {
                (!self.in_gap(pos, len, d)).then_some((s.color, 1))
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PieSlice, Resolution};
    use ratatui::widgets::Widget;

    fn slices() -> Vec<PieSlice<'static>> {
        vec![
            PieSlice::new("A", 50.0, Color::Red),
            PieSlice::new("B", 30.0, Color::Green),
            PieSlice::new("C", 20.0, Color::Blue),
        ]
    }

    fn draw(chart: PieChart<'_>, w: u16, h: u16) -> Buffer {
        let area = Rect::new(0, 0, w, h);
        let mut buf = Buffer::empty(area);
        chart.render(area, &mut buf);
        buf
    }

    fn colors(buf: &Buffer) -> Vec<Color> {
        let mut v: Vec<Color> = buf
            .content()
            .iter()
            .filter(|c| c.symbol() != " ")
            .map(|c| c.fg)
            .collect();
        v.sort_by_key(|c| format!("{c:?}"));
        v.dedup();
        v
    }

    fn dots(buf: &Buffer) -> usize {
        buf.content().iter().filter(|c| c.symbol() != " ").count()
    }

    #[test]
    fn every_style_renders_all_slice_colors() {
        for style in PieStyle::ALL {
            let b = draw(
                PieChart::new(slices()).pie_style(style).show_legend(false),
                30,
                15,
            );
            let cs = colors(&b);
            for c in [Color::Red, Color::Green, Color::Blue] {
                assert!(cs.contains(&c), "{style:?} lacks {c:?}");
            }
        }
    }

    #[test]
    fn styles_do_not_panic_on_odd_input() {
        for style in PieStyle::ALL {
            for (w, h) in [(0, 0), (1, 1), (3, 2), (10, 5), (80, 40)] {
                for sl in [
                    vec![],
                    vec![PieSlice::new("z", 0.0, Color::Red)],
                    slices(),
                    vec![PieSlice::new("n", f64::NAN, Color::Red)],
                    vec![PieSlice::new("one", 5.0, Color::Red)],
                ] {
                    let c = PieChart::new(sl)
                        .pie_style(style)
                        .gap(3.0)
                        .explode(4.0)
                        .hole(0.5)
                        .center("x");
                    let _ = draw(c, w, h);
                }
            }
        }
    }

    #[test]
    fn hole_removes_center_dots() {
        let solid = dots(&draw(
            PieChart::new(slices())
                .resolution(Resolution::Braille)
                .show_legend(false),
            30,
            15,
        ));
        let donut = dots(&draw(
            PieChart::new(slices())
                .resolution(Resolution::Braille)
                .hole(0.6)
                .show_legend(false),
            30,
            15,
        ));
        assert!(donut < solid);
        let solid = dots(&draw(PieChart::new(slices()).show_legend(false), 30, 15));
        let donut = dots(&draw(
            PieChart::new(slices()).hole(0.6).show_legend(false),
            30,
            15,
        ));
        assert!(donut < solid);
    }

    #[test]
    fn gap_and_explode_change_the_picture() {
        let base = draw(
            PieChart::new(slices())
                .pie_style(PieStyle::Solid)
                .gap(0.1)
                .show_legend(false),
            30,
            15,
        );
        let gapped = draw(PieChart::new(slices()).gap(4.0).show_legend(false), 30, 15);
        assert!(dots(&gapped) <= dots(&base));
        assert_ne!(base, gapped);
        let exploded = draw(
            PieChart::new(slices()).explode(4.0).show_legend(false),
            30,
            15,
        );
        assert_ne!(base, exploded);
    }

    #[test]
    fn start_angle_rotates() {
        let a = draw(PieChart::new(slices()).show_legend(false), 30, 15);
        let b = draw(
            PieChart::new(slices()).start_angle(90.0).show_legend(false),
            30,
            15,
        );
        assert_ne!(a, b);
        assert_eq!(
            draw(
                PieChart::new(slices())
                    .start_angle(360.0)
                    .show_legend(false),
                30,
                15
            ),
            a
        );
    }

    #[test]
    fn center_text_needs_a_hole() {
        let t = |c: PieChart<'_>| {
            draw(c, 30, 15)
                .content()
                .iter()
                .map(ratatui::buffer::Cell::symbol)
                .collect::<String>()
        };
        assert!(t(PieChart::new(slices())
            .hole(0.6)
            .center("HELLO")
            .show_legend(false))
        .contains("HELLO"));
        assert!(!t(PieChart::new(slices()).center("HELLO").show_legend(false)).contains("HELLO"));
        assert!(t(PieChart::new(slices())
            .pie_style(PieStyle::Hud)
            .center("HELLO")
            .show_legend(false))
        .contains("HELLO"));
    }

    #[test]
    fn rose_radius_follows_value() {
        let big = vec![
            PieSlice::new("a", 100.0, Color::Red),
            PieSlice::new("b", 1.0, Color::Blue),
        ];
        let b = draw(
            PieChart::new(big)
                .pie_style(PieStyle::Rose)
                .show_legend(false),
            30,
            15,
        );
        let count = |c: Color| {
            b.content()
                .iter()
                .filter(|x| x.symbol() != " " && x.fg == c)
                .count()
        };
        assert!(count(Color::Red) > count(Color::Blue) * 3);
    }

    #[test]
    fn builders_sanitise() {
        let c = PieChart::default()
            .hole(5.0)
            .gap(f64::NAN)
            .explode(-3.0)
            .start_angle(f64::INFINITY);
        assert!((c.hole - 0.95).abs() < 1e-12);
        assert_eq!((c.gap, c.explode, c.start), (0.0, 0.0, 0.0));
    }

    #[test]
    fn style_names() {
        for s in PieStyle::ALL {
            assert_ne!(s.name(), "");
        }
    }
}
