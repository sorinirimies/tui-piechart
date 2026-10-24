//! Shared helpers: bounded arbitrary inputs mapped onto tui-piechart types.
#![allow(dead_code)]

use arbitrary::Arbitrary;
use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};
use tui_piechart::*;

pub const MAX_SLICES: usize = 64;
pub const MAX_W: u16 = 200;
pub const MAX_H: u16 = 100;

#[derive(Arbitrary, Debug, Clone, Copy)]
pub struct Size {
    pub w: u8,
    pub h: u8,
    pub x: u8,
    pub y: u8,
}

impl Size {
    /// Area clamped to <= 200x100, offset from origin by up to 7 cells.
    pub fn area(&self) -> Rect {
        Rect::new(
            self.x as u16 % 8,
            self.y as u16 % 8,
            (self.w as u16) % (MAX_W + 1),
            (self.h as u16) % (MAX_H + 1),
        )
    }
}

/// Render any widget into a buffer that covers `area`.
pub fn render<W: Widget>(w: W, area: Rect) {
    let full = Rect::new(0, 0, area.right(), area.bottom());
    let mut buf = Buffer::empty(full);
    w.render(area, &mut buf);
}

pub fn color(c: (u8, u8, u8)) -> Color {
    Color::Rgb(c.0, c.1, c.2)
}

pub fn resolution(i: u8) -> Resolution {
    if i % 2 == 0 { Resolution::Standard } else { Resolution::Braille }
}
pub fn lpos(i: u8) -> LegendPosition {
    [LegendPosition::Right, LegendPosition::Left, LegendPosition::Top, LegendPosition::Bottom][i as usize % 4]
}
pub fn llayout(i: u8) -> LegendLayout {
    if i % 2 == 0 { LegendLayout::Vertical } else { LegendLayout::Horizontal }
}
pub fn lalign(i: u8) -> LegendAlignment {
    [LegendAlignment::Left, LegendAlignment::Center, LegendAlignment::Right][i as usize % 3]
}
pub fn border(i: u8) -> tui_piechart::border_style::BorderStyle {
    use tui_piechart::border_style::BorderStyle::*;
    [Standard, Rounded, Dashed, RoundedDashed, CornerGapped, RoundedCornerGapped,
     DoubleLineStandard, DoubleLineRounded, Thick, ThickRounded, ThickDashed, ThickCornerGapped][i as usize % 12]
}
pub fn tstyle(i: u8) -> TitleStyle {
    use TitleStyle::*;
    [Normal, Bold, Italic, BoldItalic, Script, BoldScript, SansSerif, BoldSansSerif, ItalicSansSerif, Monospace][i as usize % 10]
}

#[derive(Arbitrary, Debug, Clone)]
pub struct SliceIn {
    pub label: String,
    pub value: f64,
    pub color: (u8, u8, u8),
}

#[derive(Arbitrary, Debug)]
pub struct ChartIn {
    pub slices: Vec<SliceIn>,
    pub legend: bool,
    pub percentages: bool,
    pub pie_char: char,
    pub marker: String,
    pub resolution: u8,
    pub high_res: Option<bool>,
    pub pos: u8,
    pub layout: u8,
    pub align: u8,
    pub border: Option<u8>,
    pub title: Option<(String, u8, u8)>,
    pub size: Size,
    pub pie_style: u8,
    pub hole: f64,
    pub start: f64,
    pub gap: f64,
    pub explode: f64,
    pub center: Option<String>,
}

/// Build + render a chart; borrows from `input`.
pub fn run_chart(input: &ChartIn) {
    let slices: Vec<PieSlice> = input
        .slices
        .iter()
        .take(MAX_SLICES)
        .map(|s| PieSlice::new(&s.label, s.value, color(s.color)))
        .collect();
    let mut c = PieChart::new(slices)
        .show_legend(input.legend)
        .show_percentages(input.percentages)
        .pie_char(input.pie_char)
        .legend_marker(&input.marker)
        .resolution(resolution(input.resolution))
        .legend_position(lpos(input.pos))
        .legend_layout(llayout(input.layout))
        .legend_alignment(lalign(input.align));
    c = c
        .pie_style(PieStyle::ALL[input.pie_style as usize % PieStyle::ALL.len()])
        .hole(input.hole)
        .start_angle(input.start)
        .gap(input.gap)
        .explode(input.explode);
    if let Some(t) = &input.center {
        c = c.center(t.as_str());
    }
    if let Some(h) = input.high_res {
        c = c.high_resolution(h);
    }
    if let Some(b) = input.border {
        c = c.block(border(b).block());
    }
    if let Some((t, _, _)) = &input.title {
        c = c.block(ratatui::widgets::Block::bordered().title(t.as_str()));
    }
    render(&c, input.size.area());
    render(c, input.size.area());
}
