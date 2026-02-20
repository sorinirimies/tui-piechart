//! # Legend Positioning Example
//!
//! This example demonstrates all available legend positions and layouts for the PieChart widget.
//!
//! **Controls:**
//! - ↑/↓ or k/j - Navigate between different positions
//! - ←/→ or h/l - Toggle between Vertical and Horizontal layouts
//! - q/Esc - Quit
//!
//! Run with: cargo run --example legend_positioning

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Padding,
    DefaultTerminal, Frame,
};
use tui_piechart::{
    border_style::BorderStyle,
    legend::{LegendLayout, LegendPosition},
    title::{BlockExt, TitleAlignment},
    PieChart, PieSlice,
};

struct App {
    position_index: usize,
    layout: LegendLayout,
    positions: Vec<(LegendPosition, &'static str)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            position_index: 0,
            layout: LegendLayout::Vertical,
            positions: vec![
                (LegendPosition::Right, "Right"),
                (LegendPosition::Left, "Left"),
                (LegendPosition::Top, "Top"),
                (LegendPosition::Bottom, "Bottom"),
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
                    if app.position_index > 0 {
                        app.position_index -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if app.position_index < app.positions.len() - 1 {
                        app.position_index += 1;
                    }
                }
                KeyCode::Left | KeyCode::Char('h') | KeyCode::Right | KeyCode::Char('l') => {
                    app.layout = match app.layout {
                        LegendLayout::Vertical => LegendLayout::Horizontal,
                        LegendLayout::Horizontal => LegendLayout::Vertical,
                    };
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
        Constraint::Min(0),    // Content
        Constraint::Length(3), // Footer
    ])
    .split(frame.area());

    // Header
    render_header(frame, main_layout[0]);

    // Content - show current position/layout in large chart
    render_main_chart(frame, main_layout[1], app);

    // Footer
    render_footer(frame, main_layout[2]);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let block = BorderStyle::Rounded
        .block()
        .title(" Legend Positioning Demo ")
        .title_alignment_horizontal(TitleAlignment::Center)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::horizontal(1));

    let text = vec![
        Line::from("Demonstrates legend positioning and layout options"),
        Line::from(vec![
            Span::styled("Position: ", Style::default().fg(Color::Gray)),
            Span::styled("↑/↓", Style::default().fg(Color::Cyan).bold()),
            Span::styled("  Layout: ", Style::default().fg(Color::Gray)),
            Span::styled("←/→", Style::default().fg(Color::Cyan).bold()),
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
        Span::styled("↑/↓", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Position  "),
        Span::styled("←/→", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Layout  "),
        Span::styled("q", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Quit"),
    ]);

    let paragraph = ratatui::widgets::Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(paragraph.block(block), area);
}

fn render_main_chart(frame: &mut Frame, area: Rect, app: &App) {
    let (position, position_name) = app.positions[app.position_index];
    let layout_name = match app.layout {
        LegendLayout::Vertical => "Vertical",
        LegendLayout::Horizontal => "Horizontal",
    };

    let slices = vec![
        PieSlice::new("Rust", 45.0, Color::Red),
        PieSlice::new("Go", 30.0, Color::Blue),
        PieSlice::new("Python", 15.0, Color::Green),
        PieSlice::new("JavaScript", 10.0, Color::Yellow),
    ];

    let title = format!(" {} - {} ", position_name, layout_name);

    let block = BorderStyle::Rounded
        .block()
        .title(title.clone())
        .title_alignment_horizontal(TitleAlignment::Center)
        .border_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .padding(Padding::new(2, 2, 1, 1));

    // Add bottom title explaining the layout
    let block = block
        .title_bottom(Line::from(vec![
            Span::styled("Legend Position: ", Style::default().fg(Color::Gray)),
            Span::styled(position_name, Style::default().fg(Color::Cyan).bold()),
            Span::raw("  "),
            Span::styled("Layout: ", Style::default().fg(Color::Gray)),
            Span::styled(layout_name, Style::default().fg(Color::Cyan).bold()),
        ]))
        .title_alignment(Alignment::Center);

    let piechart = PieChart::new(slices)
        .block(block)
        .show_legend(true)
        .show_percentages(true)
        .legend_position(position)
        .legend_layout(app.layout);

    frame.render_widget(piechart, area);
}
