#![no_main]
mod common;
use common::*;
use libfuzzer_sys::fuzz_target;
use tui_piechart::*;

// Raw bytes → slices with extreme floats; exercise accessors, Default, slices().
fuzz_target!(|data: &[u8]| {
    let mut slices = Vec::new();
    for ch in data.chunks(9).take(MAX_SLICES) {
        let mut b = [0u8; 8];
        let n = ch.len().min(8);
        b[..n].copy_from_slice(&ch[..n]);
        let v = f64::from_bits(u64::from_le_bytes(b));
        let col = ch.last().copied().unwrap_or(0);
        slices.push(PieSlice::new("s", v, ratatui::style::Color::Indexed(col)));
    }
    for s in &slices {
        let _ = (s.label(), s.value(), s.color());
    }
    let c = PieChart::default().slices(slices);
    render(&c, ratatui::layout::Rect::new(3, 2, 40, 20));
    render(&c, ratatui::layout::Rect::new(0, 0, 0, 0));
    render(&c, ratatui::layout::Rect::new(0, 0, 1, 1));
});
