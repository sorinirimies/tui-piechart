//! # Chart showcase
//!
//! RadialBar, RingStack, Funnel, RadarChart, Treemap and Waterfall.
//!
//! **Controls:** Tab/←/→ switch page · t toggle funnel style · q/Esc quit
//!
//! Run with: `cargo run --example charts`

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Paragraph},
    DefaultTerminal, Frame,
};
use tui_piechart::{
    Funnel, FunnelStage, FunnelStyle, RadarChart, RadarSeries, RadialBar, RadialRing, RingStack,
    Treemap, TreemapItem, Waterfall, WaterfallStep,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let res = run(&mut terminal);
    ratatui::restore();
    res
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let (mut page, mut trapezoid) = (0usize, true);
    loop {
        terminal.draw(|f| draw(f, page, trapezoid))?;
        if let Event::Key(k) = event::read()? {
            match k.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Tab | KeyCode::Right => page = (page + 1) % 2,
                KeyCode::Left => page = (page + 1) % 2, // two pages: same as Right
                KeyCode::Char('t') => trapezoid = !trapezoid,
                _ => {}
            }
        }
    }
}

fn frame(title: &str) -> Block<'_> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(format!(" {title} "))
}

fn draw(f: &mut Frame, page: usize, trapezoid: bool) {
    let [head, body] =
        Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(f.area());
    f.render_widget(
        Paragraph::new(Line::from(format!(
            " Charts · page {}/2   Tab page · t funnel style · q quit",
            page + 1
        ))),
        head,
    );
    let cols = Layout::horizontal([Constraint::Ratio(1, 3); 3]).split(body);
    if page == 0 {
        f.render_widget(
            RadialBar::new(vec![
                RadialRing::new("Move", 420.0, 600.0),
                RadialRing::new("Exercise", 25.0, 30.0),
                RadialRing::new("Stand", 9.0, 12.0),
            ])
            .block(frame("RadialBar")),
            cols[0],
        );
        f.render_widget(
            RingStack::new(vec![
                ("CPU", 72.0),
                ("RAM", 48.0),
                ("Disk", 91.0),
                ("Net", 20.0),
            ])
            .block(frame("RingStack")),
            cols[1],
        );
        f.render_widget(
            Funnel::new(vec![
                FunnelStage::new("Visits", 10_000.0),
                FunnelStage::new("Sign-ups", 2_400.0),
                FunnelStage::new("Trials", 900.0),
                FunnelStage::new("Paid", 380.0),
            ])
            .style(if trapezoid {
                FunnelStyle::Trapezoid
            } else {
                FunnelStyle::Bars
            })
            .block(frame("Funnel")),
            cols[2],
        );
    } else {
        f.render_widget(
            RadarChart::new(
                vec!["Speed", "Power", "Range", "Armor", "Agility"],
                vec![
                    RadarSeries::new("Scout", vec![9.0, 3.0, 7.0, 2.0, 8.0]),
                    RadarSeries::new("Tank", vec![3.0, 8.0, 4.0, 9.0, 2.0]),
                ],
            )
            .block(frame("RadarChart")),
            cols[0],
        );
        f.render_widget(
            Treemap::new(vec![
                TreemapItem::new("src", 420.0),
                TreemapItem::new("tests", 180.0),
                TreemapItem::new("docs", 90.0),
                TreemapItem::new("examples", 70.0),
                TreemapItem::new("ci", 30.0),
                TreemapItem::new("misc", 12.0),
            ])
            .block(frame("Treemap")),
            cols[1],
        );
        f.render_widget(
            Waterfall::new(vec![
                WaterfallStep::delta("Rev", 120.0),
                WaterfallStep::delta("Cost", -45.0),
                WaterfallStep::delta("Tax", -20.0),
                WaterfallStep::delta("Misc", 15.0),
                WaterfallStep::total("Net"),
            ])
            .block(frame("Waterfall")),
            cols[2],
        );
    }
}
