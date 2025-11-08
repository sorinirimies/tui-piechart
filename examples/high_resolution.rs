//! # High Resolution Example (Animated)
//!
//! This example demonstrates the high resolution rendering mode using braille patterns
//! with smooth animations that showcase the quality difference.
//!
//! Braille characters provide 8 dots per character cell (2x4 grid) for 8x resolution.
//! The pie chart values animate continuously using sine waves at ~60 FPS, making the
//! superior smoothness of braille rendering immediately apparent.
//!
//! **Controls:**
//! - Space/Enter/H - Toggle between standard and high resolution modes
//! - q/Esc - Quit
//!
//! Run with: cargo run --example high_resolution

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph},
    DefaultTerminal, Frame,
};
use std::time::{Duration, Instant};
use tui_piechart::{PieChart, PieSlice};

struct App {
    high_res: bool,
    animation_time: f64,
    last_tick: Instant,
}

impl Default for App {
    fn default() -> Self {
        Self {
            high_res: false,
            animation_time: 0.0,
            last_tick: Instant::now(),
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app = App::default();
    let terminal = ratatui::init();
    let result = run(terminal, &mut app);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app: &mut App) -> Result<()> {
    loop {
        // Update animation time
        let now = Instant::now();
        let delta = now.duration_since(app.last_tick).as_secs_f64();
        app.last_tick = now;
        app.animation_time += delta;

        terminal.draw(|frame| render(frame, app))?;

        // Non-blocking event polling with 16ms timeout (~60 FPS)
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char(' ') | KeyCode::Enter | KeyCode::Char('h') => {
                        app.high_res = !app.high_res;
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app: &App) {
    let main_layout = Layout::vertical([
        Constraint::Length(7), // Header
        Constraint::Min(0),    // Content
        Constraint::Length(3), // Footer
    ])
    .split(frame.area());

    render_header(frame, main_layout[0], app);
    render_content(frame, main_layout[1], app);
    render_footer(frame, main_layout[2]);
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered()
        .title(" High Resolution Mode Demo (Animated) ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::horizontal(1));

    let mode = if app.high_res {
        Span::styled("HIGH RESOLUTION", Style::default().fg(Color::Green).bold())
    } else {
        Span::styled("STANDARD", Style::default().fg(Color::Yellow).bold())
    };

    let text = vec![
        Line::from(vec![Span::raw("Current mode: "), mode]),
        Line::from(""),
        Line::from(Span::styled(
            "High resolution uses braille patterns (8 dots per cell)",
            Style::default().fg(Color::Gray),
        )),
        Line::from(Span::styled(
            "Press Space/Enter/H to toggle • Values animate automatically",
            Style::default().fg(Color::Gray),
        )),
    ];

    let paragraph = Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(paragraph.block(block), area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .border_style(Style::default().fg(Color::DarkGray))
        .padding(Padding::horizontal(1));

    let text = Line::from(vec![
        Span::styled("Space/Enter/H", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Toggle Resolution  "),
        Span::styled("q", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Quit"),
    ]);

    let paragraph = Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(paragraph.block(block), area);
}

fn render_content(frame: &mut Frame, area: Rect, app: &App) {
    let rows =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);

    let top_row =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[0]);

    let bottom_row =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[1]);

    // Top-left: Programming Languages
    render_chart_languages(frame, top_row[0], app.high_res, app.animation_time);

    // Top-right: Market Share
    render_chart_market(frame, top_row[1], app.high_res, app.animation_time);

    // Bottom-left: Time Allocation
    render_chart_time(frame, bottom_row[0], app.high_res, app.animation_time);

    // Bottom-right: Budget Distribution
    render_chart_budget(frame, bottom_row[1], app.high_res, app.animation_time);
}

fn render_chart_languages(frame: &mut Frame, area: Rect, high_res: bool, time: f64) {
    // Animate values with sine waves at different frequencies
    let rust_val = 35.0 + 15.0 * (time * 0.5).sin();
    let go_val = 25.0 + 10.0 * (time * 0.7 + 1.0).sin();
    let python_val = 100.0 - rust_val - go_val;

    let slices = vec![
        PieSlice::new("Rust", rust_val, Color::Red),
        PieSlice::new("Go", go_val, Color::Blue),
        PieSlice::new("Python", python_val, Color::Green),
    ];

    let title = if high_res {
        " Languages (High-Res) "
    } else {
        " Languages (Standard) "
    };

    let block = Block::bordered()
        .title(title)
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::DarkGray))
        .padding(Padding::new(1, 1, 0, 0));

    let piechart = PieChart::new(slices)
        .block(block)
        .show_legend(true)
        .show_percentages(true)
        .high_resolution(high_res);

    frame.render_widget(piechart, area);
}

fn render_chart_market(frame: &mut Frame, area: Rect, high_res: bool, time: f64) {
    // Animate with different phase offsets
    let prod_a = 30.0 + 15.0 * (time * 0.6 + 2.0).sin();
    let prod_b = 30.0 + 15.0 * (time * 0.8 + 4.0).sin();
    let prod_c = 100.0 - prod_a - prod_b;

    let slices = vec![
        PieSlice::new("Product A", prod_a, Color::Magenta),
        PieSlice::new("Product B", prod_b, Color::Yellow),
        PieSlice::new("Product C", prod_c, Color::Cyan),
    ];

    let title = if high_res {
        " Market Share (High-Res) "
    } else {
        " Market Share (Standard) "
    };

    let block = Block::bordered()
        .title(title)
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::DarkGray))
        .padding(Padding::new(1, 1, 0, 0));

    let piechart = PieChart::new(slices)
        .block(block)
        .show_legend(true)
        .show_percentages(true)
        .high_resolution(high_res);

    frame.render_widget(piechart, area);
}

fn render_chart_time(frame: &mut Frame, area: Rect, high_res: bool, time: f64) {
    // Slower animation for time allocation
    let work = 45.0 + 15.0 * (time * 0.4).sin();
    let sleep = 25.0 + 10.0 * (time * 0.5 + 3.0).sin();
    let leisure = 100.0 - work - sleep;

    let slices = vec![
        PieSlice::new("Work", work, Color::LightBlue),
        PieSlice::new("Sleep", sleep, Color::LightMagenta),
        PieSlice::new("Leisure", leisure, Color::LightGreen),
    ];

    let title = if high_res {
        " Time Allocation (High-Res) "
    } else {
        " Time Allocation (Standard) "
    };

    let block = Block::bordered()
        .title(title)
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::DarkGray))
        .padding(Padding::new(1, 1, 0, 0));

    let piechart = PieChart::new(slices)
        .block(block)
        .show_legend(true)
        .show_percentages(true)
        .high_resolution(high_res);

    frame.render_widget(piechart, area);
}

fn render_chart_budget(frame: &mut Frame, area: Rect, high_res: bool, time: f64) {
    // Four values with different animation speeds
    let housing = 30.0 + 10.0 * (time * 0.55 + 1.5).sin();
    let food = 20.0 + 8.0 * (time * 0.65 + 2.5).sin();
    let transport = 18.0 + 7.0 * (time * 0.45 + 3.5).sin();
    let other = 100.0 - housing - food - transport;

    let slices = vec![
        PieSlice::new("Housing", housing, Color::LightRed),
        PieSlice::new("Food", food, Color::LightYellow),
        PieSlice::new("Transport", transport, Color::LightCyan),
        PieSlice::new("Other", other, Color::Gray),
    ];

    let title = if high_res {
        " Budget (High-Res) "
    } else {
        " Budget (Standard) "
    };

    let block = Block::bordered()
        .title(title)
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::DarkGray))
        .padding(Padding::new(1, 1, 0, 0));

    let piechart = PieChart::new(slices)
        .block(block)
        .show_legend(true)
        .show_percentages(true)
        .high_resolution(high_res);

    frame.render_widget(piechart, area);
}
