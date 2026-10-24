//! Small helpers shared by the chart widgets (crate-private).
#![allow(clippy::cast_possible_truncation)]

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Widget};

/// Default series palette, used when a widget gets no explicit colors.
pub(crate) const PALETTE: [Color; 8] = [
    Color::Cyan,
    Color::Magenta,
    Color::Yellow,
    Color::Green,
    Color::LightRed,
    Color::LightBlue,
    Color::LightMagenta,
    Color::LightGreen,
];

/// Palette color for series `i` (wraps around).
pub(crate) fn palette(i: usize) -> Color {
    PALETTE[i % PALETTE.len()]
}

/// Display width of `s` in terminal cells.
pub(crate) fn text_width(s: &str) -> usize {
    Span::raw(s).width()
}

/// Truncates `s` to at most `max` cells.
pub(crate) fn truncate(s: &str, max: usize) -> String {
    let mut out = String::new();
    let mut w = 0;
    for ch in s.chars() {
        let cw = text_width(ch.encode_utf8(&mut [0; 4]));
        if w + cw > max {
            break;
        }
        out.push(ch);
        w += cw;
    }
    out
}

/// Sets one cell's glyph and style, ignoring positions outside the buffer.
pub(crate) fn put(buf: &mut Buffer, x: u16, y: u16, ch: char, style: Style) {
    if let Some(cell) = buf.cell_mut((x, y)) {
        cell.set_char(ch).set_style(style);
    }
}

/// Renders an optional block and returns the inner area.
pub(crate) fn block_inner(block: Option<&Block<'_>>, area: Rect, buf: &mut Buffer) -> Rect {
    match block {
        Some(b) => {
            let inner = b.inner(area);
            b.clone().render(area, buf);
            inner
        }
        None => area,
    }
}

/// Finite, non-negative value (NaN, infinities and negatives become `0`).
pub(crate) fn sane(v: f64) -> f64 {
    if v.is_finite() && v > 0.0 {
        v
    } else {
        0.0
    }
}

/// Readable text color (black or white) on `bg`.
pub(crate) fn contrast(bg: Color) -> Color {
    let (r, g, b) = match bg {
        Color::Rgb(r, g, b) => (u32::from(r), u32::from(g), u32::from(b)),
        Color::Black | Color::Blue | Color::Red | Color::Magenta | Color::DarkGray => (0, 0, 0),
        _ => (255, 255, 255),
    };
    if (r * 299 + g * 587 + b * 114) / 1000 > 140 {
        Color::Black
    } else {
        Color::White
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_respects_width() {
        assert_eq!(truncate("hello", 3), "hel");
        assert_eq!(truncate("hi", 9), "hi");
        assert_eq!(truncate("日本語", 4), "日本");
        assert_eq!(truncate("x", 0), "");
    }

    #[test]
    fn sane_filters() {
        assert_eq!(sane(f64::NAN), 0.0);
        assert_eq!(sane(-1.0), 0.0);
        assert_eq!(sane(f64::INFINITY), 0.0);
        assert_eq!(sane(2.5), 2.5);
    }

    #[test]
    fn palette_wraps() {
        assert_eq!(palette(0), palette(PALETTE.len()));
    }

    #[test]
    fn contrast_picks_readable() {
        assert_eq!(contrast(Color::Rgb(250, 250, 250)), Color::Black);
        assert_eq!(contrast(Color::Rgb(5, 5, 5)), Color::White);
    }
}
