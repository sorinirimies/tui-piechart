//! # Pie styles
//!
//! Solid, Segmented, Hud and Rose pies with donut hole, gaps, explode and rotation.
//!
//! **Controls:** Tab style · h hole · g gap · e explode · ←/→ rotate · q quit
//!
//! Run with: `cargo run --example pie_styles`

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout},
    style::Color,
    text::Line,
    widgets::{Block, BorderType, Paragraph},
    DefaultTerminal, Frame,
};
use tui_piechart::{PieChart, PieSlice, PieStyle};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let res = run(&mut terminal);
    ratatui::restore();
    res
}

struct State {
    style: usize,
    hole: bool,
    gap: bool,
    explode: bool,
    angle: f64,
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut s = State {
        style: 0,
        hole: true,
        gap: true,
        explode: false,
        angle: 0.0,
    };
    loop {
        terminal.draw(|f| draw(f, &s))?;
        if let Event::Key(k) = event::read()? {
            match k.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Tab => s.style = (s.style + 1) % PieStyle::ALL.len(),
                KeyCode::Char('h') => s.hole = !s.hole,
                KeyCode::Char('g') => s.gap = !s.gap,
                KeyCode::Char('e') => s.explode = !s.explode,
                KeyCode::Right => s.angle = (s.angle + 15.0) % 360.0,
                KeyCode::Left => s.angle = (s.angle + 345.0) % 360.0,
                _ => {}
            }
        }
    }
}

fn draw(f: &mut Frame, s: &State) {
    let [head, body] =
        Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(f.area());
    let style = PieStyle::ALL[s.style];
    f.render_widget(
        Paragraph::new(Line::from(format!(
            " {} · hole {} · gap {} · explode {} · {}°   (Tab h g e ←/→ q)",
            style.name(),
            s.hole,
            s.gap,
            s.explode,
            s.angle
        ))),
        head,
    );
    let slices = vec![
        PieSlice::new("Rust", 45.0, Color::Red),
        PieSlice::new("Go", 30.0, Color::Cyan),
        PieSlice::new("Python", 15.0, Color::Green),
        PieSlice::new("Other", 10.0, Color::Yellow),
    ];
    let mut chart = PieChart::new(slices)
        .pie_style(style)
        .start_angle(s.angle)
        .center("langs")
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(format!(" {} ", style.name())),
        );
    if s.hole {
        chart = chart.hole(0.55);
    }
    if s.gap {
        chart = chart.gap(2.0);
    }
    if s.explode {
        chart = chart.explode(3.0);
    }
    f.render_widget(chart, body);
}
