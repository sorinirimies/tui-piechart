//! Internal braille dot canvas shared by the radial widgets (gauges, radial bars).
//!
//! One terminal cell holds a 2×4 dot grid, so a cell is roughly square in dots.
//! Angles are expressed as *turns*: `0.0` is 12 o'clock, increasing clockwise.

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::many_single_char_names,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::float_cmp
)]

use std::f64::consts::TAU;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;

/// Bit for the dot at `(dx, dy)` inside a cell (`dx` 0..2, `dy` 0..4).
const BITS: [[u8; 4]; 2] = [[0x01, 0x02, 0x04, 0x40], [0x08, 0x10, 0x20, 0x80]];

/// A colored braille dot grid covering `w`×`h` terminal cells.
pub(crate) struct Braille {
    w: usize,
    h: usize,
    bits: Vec<u8>,
    color: Vec<Color>,
    prio: Vec<u8>,
}

/// Point on a circle: `turn` 0 = top, clockwise.
pub(crate) fn polar(cx: f64, cy: f64, r: f64, turn: f64) -> (f64, f64) {
    let a = turn * TAU;
    (cx + r * a.sin(), cy - r * a.cos())
}

impl Braille {
    pub(crate) fn new(w: u16, h: u16) -> Self {
        let (w, h) = (usize::from(w), usize::from(h));
        Self {
            w,
            h,
            bits: vec![0; w * h],
            color: vec![Color::Reset; w * h],
            prio: vec![0; w * h],
        }
    }

    /// Width in dots.
    pub(crate) fn dots_w(&self) -> f64 {
        (self.w * 2) as f64
    }

    /// Height in dots.
    pub(crate) fn dots_h(&self) -> f64 {
        (self.h * 4) as f64
    }

    /// Sets a dot. A cell takes the color of the highest-priority dot in it.
    pub(crate) fn plot(&mut self, x: f64, y: f64, color: Color, prio: u8) {
        if !(x >= 0.0 && y >= 0.0) {
            return;
        }
        let (xi, yi) = (x as usize, y as usize);
        if xi >= self.w * 2 || yi >= self.h * 4 {
            return;
        }
        let cell = (yi / 4) * self.w + xi / 2;
        if self.bits[cell] == 0 || prio >= self.prio[cell] {
            self.color[cell] = color;
            self.prio[cell] = prio;
        }
        self.bits[cell] |= BITS[xi % 2][yi % 4];
    }

    /// Paints every dot of the annulus `r_in..=r_out` that lies inside the sweep
    /// `start` + `len` turns. `paint` receives the position along the sweep
    /// (`0..=1`) and returns the color/priority, or `None` to leave the dot empty.
    pub(crate) fn ring(
        &mut self,
        center: (f64, f64),
        radii: (f64, f64),
        sweep: (f64, f64),
        mut paint: impl FnMut(f64) -> Option<(Color, u8)>,
    ) {
        self.ring_dist(center, radii, sweep, |pos, _| paint(pos));
    }

    /// Like [`Self::ring`], but `paint` also receives the dot's distance from the center.
    pub(crate) fn ring_dist(
        &mut self,
        (cx, cy): (f64, f64),
        (r_in, r_out): (f64, f64),
        (start, len): (f64, f64),
        mut paint: impl FnMut(f64, f64) -> Option<(Color, u8)>,
    ) {
        if r_out <= 0.0 || len <= 0.0 {
            return;
        }
        let x0 = (cx - r_out).floor().max(0.0) as usize;
        let x1 = ((cx + r_out).ceil().max(0.0) as usize).min(self.w * 2);
        let y0 = (cy - r_out).floor().max(0.0) as usize;
        let y1 = ((cy + r_out).ceil().max(0.0) as usize).min(self.h * 4);
        for py in y0..y1 {
            for px in x0..x1 {
                let dx = px as f64 + 0.5 - cx;
                let dy = py as f64 + 0.5 - cy;
                let d = dx.hypot(dy);
                if d < r_in || d > r_out {
                    continue;
                }
                let turn = (dx.atan2(-dy) / TAU).rem_euclid(1.0);
                let t = (turn - start).rem_euclid(1.0);
                if t > len {
                    continue;
                }
                if let Some((c, p)) = paint(t / len, d) {
                    self.plot(px as f64, py as f64, c, p);
                }
            }
        }
    }

    /// Straight line sampled every half dot.
    pub(crate) fn line(
        &mut self,
        (x0, y0): (f64, f64),
        (x1, y1): (f64, f64),
        color: Color,
        prio: u8,
    ) {
        let steps = ((x1 - x0).hypot(y1 - y0) * 2.0).ceil().max(1.0) as usize;
        for i in 0..=steps {
            let t = i as f64 / steps as f64;
            self.plot(x0 + (x1 - x0) * t, y0 + (y1 - y0) * t, color, prio);
        }
    }

    /// Writes the dots into `buf`, with the grid's top-left at `origin`.
    pub(crate) fn blit(&self, origin: Rect, buf: &mut Buffer) {
        for cy in 0..self.h {
            for cx in 0..self.w {
                let i = cy * self.w + cx;
                if self.bits[i] == 0 {
                    continue;
                }
                let ch = char::from_u32(0x2800 + u32::from(self.bits[i])).unwrap_or(' ');
                if let Some(cell) = buf.cell_mut((origin.x + cx as u16, origin.y + cy as u16)) {
                    cell.set_char(ch).set_fg(self.color[i]);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polar_cardinals() {
        let (x, y) = polar(10.0, 10.0, 5.0, 0.0);
        assert!((x - 10.0).abs() < 1e-9 && (y - 5.0).abs() < 1e-9);
        let (x, y) = polar(10.0, 10.0, 5.0, 0.25);
        assert!((x - 15.0).abs() < 1e-9 && (y - 10.0).abs() < 1e-9);
    }

    #[test]
    fn plot_ignores_out_of_range_and_nan() {
        let mut g = Braille::new(2, 2);
        g.plot(-1.0, 0.0, Color::Red, 1);
        g.plot(f64::NAN, 0.0, Color::Red, 1);
        g.plot(99.0, 99.0, Color::Red, 1);
        assert!(g.bits.iter().all(|b| *b == 0));
    }

    #[test]
    fn higher_priority_wins_cell_color() {
        let mut g = Braille::new(1, 1);
        g.plot(0.0, 0.0, Color::Red, 1);
        g.plot(1.0, 0.0, Color::Blue, 0);
        assert_eq!(g.color[0], Color::Red);
        g.plot(1.0, 1.0, Color::Green, 2);
        assert_eq!(g.color[0], Color::Green);
    }

    #[test]
    fn full_ring_lights_dots_and_sweep_limits_them() {
        let mut full = Braille::new(8, 4);
        full.ring((8.0, 8.0), (4.0, 7.0), (0.0, 1.0), |_| {
            Some((Color::Red, 1))
        });
        let mut half = Braille::new(8, 4);
        half.ring((8.0, 8.0), (4.0, 7.0), (0.0, 0.5), |_| {
            Some((Color::Red, 1))
        });
        let n = |g: &Braille| g.bits.iter().map(|b| b.count_ones()).sum::<u32>();
        assert!(n(&full) > n(&half) && n(&half) > 0);
    }
}
