#![no_main]
mod common;
use arbitrary::Arbitrary;
use common::*;
use libfuzzer_sys::fuzz_target;
use ratatui::widgets::Block;
use tui_piechart::*;

#[derive(Arbitrary, Debug)]
struct In {
    labels: Vec<String>,
    values: Vec<f64>,
    max: f64,
    mode: u8,
    flag: bool,
    block: bool,
    size: Size,
}

fuzz_target!(|i: In| {
    let n = i.labels.len().min(i.values.len()).min(64);
    let (labels, values) = (&i.labels[..n], &i.values[..n]);
    let block = || Block::bordered();
    let area = i.size.area();
    match i.mode % 6 {
        0 => {
            let rings = labels.iter().zip(values).map(|(l, v)| RadialRing::new(l, *v, i.max)).collect();
            let mut w = RadialBar::new(rings).legend(i.flag).sweep(if i.flag { RadialSweep::Full } else { RadialSweep::Arc });
            if i.block { w = w.block(block()); }
            render(w, area);
        }
        1 => {
            let st = labels.iter().zip(values).map(|(l, v)| FunnelStage::new(l, *v)).collect();
            let mut w = Funnel::new(st).style(if i.flag { FunnelStyle::Trapezoid } else { FunnelStyle::Bars });
            if i.block { w = w.block(block()); }
            render(w, area);
        }
        2 => {
            let axes = labels.iter().map(String::as_str).collect();
            let series = vec![RadarSeries::new("s", values.to_vec())];
            let mut w = RadarChart::new(axes, series).max(i.max).rings(i.mode as usize).legend(i.flag);
            if i.block { w = w.block(block()); }
            render(w, area);
        }
        3 => {
            let it = labels.iter().zip(values).map(|(l, v)| TreemapItem::new(l, *v)).collect();
            let mut w = Treemap::new(it).gap(i.flag);
            if i.block { w = w.block(block()); }
            render(w, area);
        }
        4 => {
            let st = labels
                .iter()
                .zip(values)
                .enumerate()
                .map(|(k, (l, v))| if k % 5 == 4 { WaterfallStep::total(l) } else { WaterfallStep::delta(l, *v) })
                .collect();
            let mut w = Waterfall::new(st);
            if i.block { w = w.block(block()); }
            render(w, area);
        }
        _ => {
            let items = labels.iter().map(String::as_str).zip(values.iter().copied()).collect();
            render(RingStack::new(items).legend(i.flag), area);
        }
    }
});
