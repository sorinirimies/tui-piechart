#![no_main]
mod common;
use arbitrary::Arbitrary;
use common::*;
use libfuzzer_sys::fuzz_target;
use ratatui::widgets::Block;
use tui_piechart::*;

#[derive(Arbitrary, Debug)]
struct In {
    style: u8,
    value: f64,
    min: f64,
    max: f64,
    label: String,
    text: Option<String>,
    zones: bool,
    show_value: bool,
    block: bool,
    color: (u8, u8, u8),
    size: Size,
}

fuzz_target!(|i: In| {
    let style = GaugeStyle::ALL[i.style as usize % GaugeStyle::ALL.len()];
    let mut g = GaugeChart::new(i.value)
        .range(i.min, i.max)
        .style(style)
        .label(&i.label)
        .zones(i.zones)
        .show_value(i.show_value)
        .color(color(i.color));
    if let Some(t) = &i.text {
        g = g.value_text(t.as_str());
    }
    if i.block {
        g = g.block(Block::bordered().title("g"));
    }
    let f = g.fraction();
    assert!((0.0..=1.0).contains(&f));
    render(g, i.size.area());
});
