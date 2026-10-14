#![no_main]
mod common;
use arbitrary::Arbitrary;
use common::*;
use libfuzzer_sys::fuzz_target;
use ratatui::widgets::Block;
use tui_piechart::title::Title;
use tui_piechart::*;

#[derive(Arbitrary, Debug)]
struct In {
    border: u8,
    title: String,
    style: u8,
    align: u8,
    bottom: bool,
    slices: Vec<SliceIn>,
    size: Size,
}

fuzz_target!(|i: In| {
    let t = Title::new(i.title.as_str());
    let t = if i.bottom { t.bottom() } else { t.top() };
    let t = match i.align % 3 { 0 => t.left(), 1 => t.center(), _ => t.right() };
    let block: Block = border(i.border).block().title(t.render());
    let slices: Vec<PieSlice> = i.slices.iter().take(MAX_SLICES).map(|s| PieSlice::new(&s.label, s.value, color(s.color))).collect();
    let _ = tstyle(i.style).apply(&i.title);
    render(PieChart::new(slices).block(block), i.size.area());
});
