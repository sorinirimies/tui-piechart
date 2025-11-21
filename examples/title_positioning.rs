//! # Title Positioning Example
//!
//! This example demonstrates all available title positioning options for PieChart blocks.
//!
//! **Controls:**
//! - ↑/↓ or k/j - Change vertical position (Top/Bottom)
//! - ←/→ or h/l - Change horizontal alignment (Start/Center/End)
//! - q/Esc - Quit
//!
//! Run with: cargo run --example title_positioning

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::Padding,
    DefaultTerminal, Frame,
};
use tui_piechart::{
    border_style::BorderStyle,
    title::{BlockExt, TitleAlignment, TitlePosition},
    PieChart, PieSlice,
};

struct App {
    horizontal_index: usize,
    vertical_index: usize,
    horizontal_options: Vec<(TitleAlignment, &'static str)>,
    vertical_options: Vec<(TitlePosition, &'static str)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            horizontal_index: 1, // Start with Center
            vertical_index: 0,   // Start with Top
            horizontal_options: vec![
                (TitleAlignment::Start, "Start"),
                (TitleAlignment::Center, "Center"),
                (TitleAlignment::End, "End"),
            ],
            vertical_options: vec![
                (TitlePosition::Top, "Top"),
                (TitlePosition::Bottom, "Bottom"),
            ],
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
        terminal.draw(|frame| render(frame, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Up | KeyCode::Char('k') => {
                    if app.vertical_index > 0 {
                        app.vertical_index -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if app.vertical_index < app.vertical_options.len() - 1 {
                        app.vertical_index += 1;
                    }
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    if app.horizontal_index > 0 {
                        app.horizontal_index -= 1;
                    }
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    if app.horizontal_index < app.horizontal_options.len() - 1 {
                        app.horizontal_index += 1;
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app: &App) {
    let main_layout = Layout::vertical([
        Constraint::Length(5), // Header
        Constraint::Min(0),    // Content - 3 columns
        Constraint::Length(3), // Footer
    ])
    .split(frame.area());

    // Header
    render_header(frame, main_layout[0]);

    // Content - show all 6 combinations (3 horizontal × 2 vertical)
    render_grid(frame, main_layout[1], app);

    // Footer
    render_footer(frame, main_layout[2]);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let block = BorderStyle::Rounded
        .block()
        .title(" Title Positioning Demo ")
        .title_alignment_horizontal(TitleAlignment::Center)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::horizontal(1));

    let text = vec![
        Line::from("Demonstrates title alignment (horizontal) and position (vertical)"),
        Line::from(vec![
            Span::styled("Horizontal: ", Style::default().fg(Color::Gray)),
            Span::styled("←/→", Style::default().fg(Color::Cyan).bold()),
            Span::styled("  Vertical: ", Style::default().fg(Color::Gray)),
            Span::styled("↑/↓", Style::default().fg(Color::Cyan).bold()),
        ]),
    ];

    let paragraph = ratatui::widgets::Paragraph::new(text).alignment(Alignment::Center);

    frame.render_widget(paragraph.block(block), area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let block = BorderStyle::Rounded
        .block()
        .border_style(Style::default().fg(Color::DarkGray))
        .padding(Padding::horizontal(1));

    let text = Line::from(vec![
        Span::styled("←/→", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Horizontal  "),
        Span::styled("↑/↓", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Vertical  "),
        Span::styled("q", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Quit"),
    ]);

    let paragraph = ratatui::widgets::Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(paragraph.block(block), area);
}

fn render_grid(frame: &mut Frame, area: Rect, app: &App) {
    // Create 2 rows (for Top and Bottom)
    let rows =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);

    // Create 3 columns (for Start, Center, End)
    let top_cols = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ])
    .split(rows[0]);

    let bottom_cols = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ])
    .split(rows[1]);

    // Render all 6 combinations
    // Top row
    render_chart(
        frame,
        top_cols[0],
        TitleAlignment::Start,
        TitlePosition::Top,
        "Start - Top",
        app.is_selected(0, 0),
    );
    render_chart(
        frame,
        top_cols[1],
        TitleAlignment::Center,
        TitlePosition::Top,
        "Center - Top",
        app.is_selected(1, 0),
    );
    render_chart(
        frame,
        top_cols[2],
        TitleAlignment::End,
        TitlePosition::Top,
        "End - Top",
        app.is_selected(2, 0),
    );

    // Bottom row
    render_chart(
        frame,
        bottom_cols[0],
        TitleAlignment::Start,
        TitlePosition::Bottom,
        "Start - Bottom",
        app.is_selected(0, 1),
    );
    render_chart(
        frame,
        bottom_cols[1],
        TitleAlignment::Center,
        TitlePosition::Bottom,
        "Center - Bottom",
        app.is_selected(1, 1),
    );
    render_chart(
        frame,
        bottom_cols[2],
        TitleAlignment::End,
        TitlePosition::Bottom,
        "End - Bottom",
        app.is_selected(2, 1),
    );
}

impl App {
    fn is_selected(&self, h_idx: usize, v_idx: usize) -> bool {
        self.horizontal_index == h_idx && self.vertical_index == v_idx
    }
}

fn render_chart(
    frame: &mut Frame,
    area: Rect,
    h_align: TitleAlignment,
    v_pos: TitlePosition,
    label: &str,
    is_selected: bool,
) {
    let slices = vec![
        PieSlice::new("Rust", 45.0, Color::Red),
        PieSlice::new("Go", 30.0, Color::Blue),
        PieSlice::new("Python", 25.0, Color::Green),
    ];

    let border_color = if is_selected {
        Color::Cyan
    } else {
        Color::DarkGray
    };

    let border_style = if is_selected {
        Style::default()
            .fg(border_color)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(border_color)
    };

    let block = BorderStyle::Rounded
        .block()
        .title(format!(" {} ", label))
        .title_alignment_horizontal(h_align)
        .title_vertical_position(v_pos)
        .border_style(border_style)
        .padding(Padding::new(1, 1, 0, 0));

    let piechart = PieChart::new(slices)
        .block(block)
        .show_legend(false)
        .show_percentages(false);

    frame.render_widget(piechart, area);
}
