//! # PieChart Widget Example (Animated)
//!
//! This example demonstrates the `tui-piechart` widget with four different
//! animated pie charts showcasing various use cases.
//!
//! **Controls:**
//! - ↑/↓ or k/j - Navigate between different charts
//! - q/Esc - Quit
//! - All charts animate smoothly at ~60 FPS
//!
//! Run with: cargo run --example piechart

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph},
    DefaultTerminal, Frame,
};
use std::time::{Duration, Instant};
use tui_piechart::{symbols, PieChart, PieSlice};

struct App {
    selected: usize,
    animation_time: f64,
    last_tick: Instant,
}

impl Default for App {
    fn default() -> Self {
        Self {
            selected: 0,
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
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.selected > 0 {
                            app.selected -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.selected < 3 {
                            app.selected += 1;
                        }
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
        Constraint::Length(3), // Header
        Constraint::Min(0),    // Content
        Constraint::Length(3), // Footer
    ])
    .split(frame.area());

    // Header
    render_header(frame, main_layout[0]);

    // Content area - 2x2 grid of pie charts
    let rows = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[1]);

    let top_row =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[0]);

    let bottom_row =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[1]);

    // Render 4 different pie chart examples with animation
    render_programming_languages(frame, top_row[0], app.selected == 0, app.animation_time);
    render_market_share(frame, top_row[1], app.selected == 1, app.animation_time);
    render_time_allocation(frame, bottom_row[0], app.selected == 2, app.animation_time);
    render_budget_distribution(frame, bottom_row[1], app.selected == 3, app.animation_time);

    // Footer
    render_footer(frame, main_layout[2]);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .title(" tui-piechart Demo - Animated ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::horizontal(1));

    let text = Paragraph::new("A customizable pie chart widget for Ratatui")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Gray));

    frame.render_widget(text.block(block), area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .border_style(Style::default().fg(Color::DarkGray))
        .padding(Padding::horizontal(1));

    let text = Line::from(vec![
        Span::styled("↑/↓", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Navigate  "),
        Span::styled("q", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Quit"),
    ]);

    let paragraph = Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(paragraph.block(block), area);
}

fn render_programming_languages(frame: &mut Frame, area: Rect, is_selected: bool, time: f64) {
    // Animate values with sine waves at different frequencies
    let rust_val = 35.0 + 15.0 * (time * 0.5).sin();
    let go_val = 25.0 + 10.0 * (time * 0.7 + 1.0).sin();
    let python_val = 100.0 - rust_val - go_val;

    let slices = vec![
        PieSlice::new("Rust", rust_val, Color::Red),
        PieSlice::new("Go", go_val, Color::Blue),
        PieSlice::new("Python", python_val, Color::Green),
    ];

    let block = Block::bordered()
        .title(" Programming Languages ")
        .title_alignment(Alignment::Center)
        .border_style(if is_selected {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        })
        .padding(Padding::new(1, 1, 0, 0));

    let piechart = PieChart::new(slices)
        .block(block)
        .show_legend(true)
        .show_percentages(true);

    frame.render_widget(piechart, area);
}

fn render_market_share(frame: &mut Frame, area: Rect, is_selected: bool, time: f64) {
    // Animate with different phase offsets
    let prod_a = 30.0 + 15.0 * (time * 0.6 + 2.0).sin();
    let prod_b = 30.0 + 15.0 * (time * 0.8 + 4.0).sin();
    let prod_c = 100.0 - prod_a - prod_b;

    let slices = vec![
        PieSlice::new("Product A", prod_a, Color::Magenta),
        PieSlice::new("Product B", prod_b, Color::Yellow),
        PieSlice::new("Product C", prod_c, Color::Cyan),
    ];

    let block = Block::bordered()
        .title(" Market Share ")
        .title_alignment(Alignment::Center)
        .border_style(if is_selected {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        })
        .padding(Padding::new(1, 1, 0, 0));

    let piechart = PieChart::new(slices)
        .block(block)
        .show_legend(true)
        .show_percentages(true)
        .pie_char(symbols::PIE_CHAR_BLOCK);

    frame.render_widget(piechart, area);
}

fn render_time_allocation(frame: &mut Frame, area: Rect, is_selected: bool, time: f64) {
    // Slower animation for time allocation
    let work = 45.0 + 15.0 * (time * 0.4).sin();
    let sleep = 25.0 + 10.0 * (time * 0.5 + 3.0).sin();
    let leisure = 100.0 - work - sleep;

    let slices = vec![
        PieSlice::new("Work", work, Color::LightBlue),
        PieSlice::new("Sleep", sleep, Color::LightMagenta),
        PieSlice::new("Leisure", leisure, Color::LightGreen),
    ];

    let block = Block::bordered()
        .title(" Daily Time Allocation ")
        .title_alignment(Alignment::Center)
        .border_style(if is_selected {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        })
        .padding(Padding::new(1, 1, 0, 0));

    let piechart = PieChart::new(slices)
        .block(block)
        .show_legend(true)
        .show_percentages(true)
        .pie_char(symbols::PIE_CHAR_CIRCLE);

    frame.render_widget(piechart, area);
}

fn render_budget_distribution(frame: &mut Frame, area: Rect, is_selected: bool, time: f64) {
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

    let block = Block::bordered()
        .title(" Budget Distribution ")
        .title_alignment(Alignment::Center)
        .border_style(if is_selected {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        })
        .padding(Padding::new(1, 1, 0, 0));

    let piechart = PieChart::new(slices)
        .block(block)
        .show_legend(true)
        .show_percentages(true)
        .pie_char(symbols::PIE_CHAR_DIAMOND);

    frame.render_widget(piechart, area);
}
