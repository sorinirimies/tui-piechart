//! # Gauge showcase
//!
//! Every [`GaugeStyle`], animated.
//!
//! **Controls:** Tab/←/→ switch page · z toggle zone colors · space pause · q/Esc quit
//!
//! Run with: `cargo run --example gauges`

use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Paragraph},
    DefaultTerminal, Frame,
};
use tui_piechart::{GaugeChart, GaugeStyle};

const PAGES: [(&str, &[GaugeStyle]); 3] = [
    (
        "Radial",
        &[
            GaugeStyle::Arc,
            GaugeStyle::Segmented,
            GaugeStyle::Dial,
            GaugeStyle::Meter,
            GaugeStyle::Reticle,
            GaugeStyle::Ring,
        ],
    ),
    (
        "Linear",
        &[
            GaugeStyle::Bar,
            GaugeStyle::Chevron,
            GaugeStyle::Smooth,
            GaugeStyle::Dots,
            GaugeStyle::Led,
            GaugeStyle::Battery,
        ],
    ),
    (
        "Vertical",
        &[
            GaugeStyle::Level,
            GaugeStyle::Thermometer,
            GaugeStyle::Signal,
        ],
    ),
];

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let res = run(&mut terminal);
    ratatui::restore();
    res
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let (mut page, mut zones, mut paused) = (0usize, true, false);
    let mut t = 0.0_f64;
    let mut last = Instant::now();
    loop {
        terminal.draw(|f| draw(f, page, zones, t))?;
        if event::poll(Duration::from_millis(33))? {
            if let Event::Key(k) = event::read()? {
                match k.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Tab | KeyCode::Right => page = (page + 1) % PAGES.len(),
                    KeyCode::Left => page = (page + PAGES.len() - 1) % PAGES.len(),
                    KeyCode::Char('z') => zones = !zones,
                    KeyCode::Char(' ') => paused = !paused,
                    _ => {}
                }
            }
        }
        if !paused {
            t += last.elapsed().as_secs_f64();
        }
        last = Instant::now();
    }
}

fn draw(f: &mut Frame, page: usize, zones: bool, t: f64) {
    let [head, body] =
        Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(f.area());
    let (name, styles) = PAGES[page];
    f.render_widget(
        Paragraph::new(Line::from(format!(
            " Gauges · {name} ({}/{})   Tab page · z zones [{}] · space pause · q quit",
            page + 1,
            PAGES.len(),
            if zones { "on" } else { "off" }
        )))
        .style(Style::default().fg(Color::Gray)),
        head,
    );
    let (cols, rows) = match page {
        0 => (3, 2),
        1 => (2, 3),
        _ => (3, 1),
    };
    let row_cells = Layout::vertical(vec![Constraint::Ratio(1, rows); rows as usize]).split(body);
    for (r, row) in row_cells.iter().enumerate() {
        let col_cells =
            Layout::horizontal(vec![Constraint::Ratio(1, cols); cols as usize]).split(*row);
        for (c, cell) in col_cells.iter().enumerate() {
            let i = r * cols as usize + c;
            if let Some(style) = styles.get(i) {
                cell_gauge(f, *cell, *style, zones, t + i as f64 * 0.9);
            }
        }
    }
}

fn cell_gauge(f: &mut Frame, area: Rect, style: GaugeStyle, zones: bool, t: f64) {
    let value = 50.0 + 50.0 * (t * 0.8).sin();
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(format!(" {} ", style.name()));
    f.render_widget(
        GaugeChart::new(value)
            .style(style)
            .label("CPU")
            .zones(zones)
            .color(Color::Cyan)
            .block(block),
        area,
    );
}
