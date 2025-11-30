//! # Legend Alignment Example
//!
//! This example demonstrates the legend alignment feature for the PieChart widget.
//! You can align legend items to the left, center, or right within the legend area.
//!
//! **Controls:**
//! - ↑/↓ or k/j - Navigate between different alignments
//! - ←/→ or h/l - Toggle between positions (Right/Left/Top/Bottom)
//! - Space - Toggle between Vertical/Horizontal layouts
//! - q/Esc - Quit
//!
//! Run with: cargo run --example legend_alignment

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
    legend::{LegendAlignment, LegendLayout, LegendPosition},
    title::{BlockExt, TitleAlignment},
    PieChart, PieSlice,
};

struct App {
    alignment_index: usize,
    position_index: usize,
    layout: LegendLayout,
    alignments: Vec<(LegendAlignment, &'static str)>,
    positions: Vec<(LegendPosition, &'static str)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            alignment_index: 0,
            position_index: 0,
            layout: LegendLayout::Vertical,
            alignments: vec![
                (LegendAlignment::Left, "Left"),
                (LegendAlignment::Center, "Center"),
                (LegendAlignment::Right, "Right"),
            ],
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
                    if app.alignment_index > 0 {
                        app.alignment_index -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if app.alignment_index < app.alignments.len() - 1 {
                        app.alignment_index += 1;
                    }
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    if app.position_index > 0 {
                        app.position_index -= 1;
                    }
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    if app.position_index < app.positions.len() - 1 {
                        app.position_index += 1;
                    }
                }
                KeyCode::Char(' ') => {
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
        Constraint::Length(6), // Header
        Constraint::Min(0),    // Content
        Constraint::Length(3), // Footer
    ])
    .split(frame.area());

    render_header(frame, main_layout[0]);
    render_main_chart(frame, main_layout[1], app);
    render_footer(frame, main_layout[2]);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let block = BorderStyle::Rounded
        .block()
        .title(" Legend Alignment Demo ")
        .title_alignment_horizontal(TitleAlignment::Center)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::horizontal(1));

    let text = vec![
        Line::from("Demonstrates legend alignment options for better visual balance"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Alignment: ", Style::default().fg(Color::Gray)),
            Span::styled("↑/↓", Style::default().fg(Color::Cyan).bold()),
            Span::styled("  Position: ", Style::default().fg(Color::Gray)),
            Span::styled("←/→", Style::default().fg(Color::Cyan).bold()),
            Span::styled("  Layout: ", Style::default().fg(Color::Gray)),
            Span::styled("Space", Style::default().fg(Color::Cyan).bold()),
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
        Span::raw(" Alignment  "),
        Span::styled("←/→", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Position  "),
        Span::styled("Space", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Layout  "),
        Span::styled("q", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Quit"),
    ]);

    let paragraph = ratatui::widgets::Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(paragraph.block(block), area);
}

fn render_main_chart(frame: &mut Frame, area: Rect, app: &App) {
    let (alignment, alignment_name) = app.alignments[app.alignment_index];
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

    let title = format!(" {} - {} - {} ", alignment_name, position_name, layout_name);

    let block = BorderStyle::Rounded
        .block()
        .title(title)
        .title_alignment_horizontal(TitleAlignment::Center)
        .border_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .padding(Padding::new(2, 2, 1, 1));

    let block = block
        .title_bottom(Line::from(vec![
            Span::styled("Alignment: ", Style::default().fg(Color::Gray)),
            Span::styled(alignment_name, Style::default().fg(Color::Yellow).bold()),
            Span::raw("  "),
            Span::styled("Position: ", Style::default().fg(Color::Gray)),
            Span::styled(position_name, Style::default().fg(Color::Cyan).bold()),
            Span::raw("  "),
            Span::styled("Layout: ", Style::default().fg(Color::Gray)),
            Span::styled(layout_name, Style::default().fg(Color::Green).bold()),
        ]))
        .title_alignment(Alignment::Center);

    let piechart = PieChart::new(slices)
        .block(block)
        .show_legend(true)
        .show_percentages(true)
        .legend_position(position)
        .legend_layout(app.layout)
        .legend_alignment(alignment);

    frame.render_widget(piechart, area);
}
