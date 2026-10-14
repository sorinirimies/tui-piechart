#![no_main]
mod common;
use arbitrary::Arbitrary;
use common::*;
use libfuzzer_sys::fuzz_target;
use tui_piechart::*;

#[derive(Arbitrary, Debug)]
struct In {
    slices: Vec<SliceIn>,
    pos: u8,
    layout: u8,
    align: u8,
    percentages: bool,
    marker: String,
    size: Size,
}

fuzz_target!(|i: In| {
    // Legend-focused: tiny/odd areas, many slices, wide labels.
    let slices: Vec<PieSlice> = i.slices.iter().take(MAX_SLICES).map(|s| PieSlice::new(&s.label, s.value, color(s.color))).collect();
    let c = PieChart::new(slices)
        .show_legend(true)
        .show_percentages(i.percentages)
        .legend_marker(&i.marker)
        .legend_position(lpos(i.pos))
        .legend_layout(llayout(i.layout))
        .legend_alignment(lalign(i.align));
    render(&c, i.size.area());
});
