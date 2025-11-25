//! # Legend Markers Example
//!
//! This example showcases all the built-in legend marker symbols available
//! in the library. You can cycle through different marker styles to see how
//! they look with your pie charts.
//!
//! **Controls:**
//! - ↑/↓ or k/j - Navigate between different marker styles
//! - q/Esc - Quit
//!
//! Run with: cargo run --example legend_markers

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
    symbols,
    title::{BlockExt, TitleAlignment},
    PieChart, PieSlice,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MarkerStyle {
    marker: &'static str,
    name: &'static str,
    description: &'static str,
}

const MARKER_STYLES: &[MarkerStyle] = &[
    MarkerStyle {
        marker: symbols::LEGEND_MARKER,
        name: "Default Square",
        description: "■ The default filled square marker",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_CIRCLE,
        name: "Circle",
        description: "● Classic filled circle marker",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_SQUARE,
        name: "Small Square",
        description: "▪ Compact square marker",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_ARROW,
        name: "Arrow",
        description: "▶ Right-pointing arrow marker",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_DIAMOND,
        name: "Diamond",
        description: "◆ Filled diamond marker",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_STAR,
        name: "Star",
        description: "★ Filled star marker",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_WHITE_STAR,
        name: "White Star",
        description: "☆ Outlined star marker",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_SMALL_CIRCLE,
        name: "Small Circle",
        description: "• Compact bullet point",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_WHITE_CIRCLE,
        name: "White Circle",
        description: "○ Outlined circle marker",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_TRIANGLE,
        name: "Triangle",
        description: "▲ Upward-pointing triangle",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_HEART,
        name: "Heart",
        description: "♥ Filled heart marker",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_WHITE_HEART,
        name: "White Heart",
        description: "♡ Outlined heart marker",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_PLUS,
        name: "Plus",
        description: "✚ Plus sign marker",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_CROSS,
        name: "Cross",
        description: "✖ X-shaped cross marker",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_CHECK,
        name: "Check",
        description: "✓ Check mark marker",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_RIGHT_ARROW,
        name: "Right Arrow",
        description: "→ Simple right arrow",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_DOUBLE_RIGHT,
        name: "Double Right",
        description: "» Double chevron marker",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_DASH,
        name: "Dash",
        description: "– Horizontal dash marker",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_DOT,
        name: "Dot",
        description: "· Middle dot marker",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_HEXAGON,
        name: "Hexagon",
        description: "⬡ Outlined hexagon marker",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_BULLSEYE,
        name: "Bullseye",
        description: "◉ Circle with center dot",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_SQUARE_BOX,
        name: "Square Box",
        description: "▢ Outlined square marker",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_ASTERISM,
        name: "Asterism",
        description: "⁂ Three asterisks marker",
    },
    MarkerStyle {
        marker: symbols::LEGEND_MARKER_HORIZONTAL_BAR,
        name: "Horizontal Bar",
        description: "▱ Horizontal bar marker",
    },
];

#[derive(Default)]
struct App {
    marker_index: usize,
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
                    if app.marker_index > 0 {
                        app.marker_index -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if app.marker_index < MARKER_STYLES.len() - 1 {
                        app.marker_index += 1;
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
        Constraint::Min(0),    // Content
        Constraint::Length(3), // Footer
    ])
    .split(frame.area());

    render_header(frame, main_layout[0]);
    render_main_content(frame, main_layout[1], app);
    render_footer(frame, main_layout[2]);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let block = BorderStyle::Rounded
        .block()
        .title(" Legend Markers Demo ")
        .title_alignment_horizontal(TitleAlignment::Center)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::horizontal(1));

    let text = vec![
        Line::from("Explore all built-in legend marker symbols"),
        Line::from(vec![
            Span::styled("Navigate: ", Style::default().fg(Color::Gray)),
            Span::styled("↑/↓", Style::default().fg(Color::Cyan).bold()),
            Span::styled(" or ", Style::default().fg(Color::Gray)),
            Span::styled("k/j", Style::default().fg(Color::Cyan).bold()),
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
        Span::raw(" Navigate  "),
        Span::styled("q", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Quit"),
    ]);

    let paragraph = ratatui::widgets::Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(paragraph.block(block), area);
}

fn render_main_content(frame: &mut Frame, area: Rect, app: &App) {
    let current_style = &MARKER_STYLES[app.marker_index];

    // Create slices with sample data
    let slices = vec![
        PieSlice::new("Rust", 45.0, Color::Red),
        PieSlice::new("Go", 30.0, Color::Blue),
        PieSlice::new("Python", 15.0, Color::Green),
        PieSlice::new("JavaScript", 10.0, Color::Yellow),
    ];

    let title = format!(" {} ", current_style.name);
    let bottom_title = Line::from(vec![
        Span::styled("Marker: ", Style::default().fg(Color::Gray)),
        Span::styled(
            current_style.marker,
            Style::default().fg(Color::Cyan).bold(),
        ),
        Span::raw("  "),
        Span::styled(
            current_style.description,
            Style::default().fg(Color::Gray).italic(),
        ),
    ]);

    let block = BorderStyle::Rounded
        .block()
        .title(title.clone())
        .title_alignment_horizontal(TitleAlignment::Center)
        .title_bottom(bottom_title)
        .title_alignment(Alignment::Center)
        .border_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .padding(Padding::new(2, 2, 1, 1));

    // Add navigation hint
    let nav_hint = if app.marker_index < MARKER_STYLES.len() - 1 {
        format!("({}/{}) ▼ Next", app.marker_index + 1, MARKER_STYLES.len())
    } else {
        format!("({}/{}) (Last)", app.marker_index + 1, MARKER_STYLES.len())
    };

    let block = if app.marker_index > 0 {
        block.title_top(Line::from(vec![Span::styled(
            format!(
                "▲ Previous ({}/{})",
                app.marker_index + 1,
                MARKER_STYLES.len()
            ),
            Style::default().fg(Color::DarkGray),
        )]))
    } else {
        block.title_top(Line::from(vec![Span::styled(
            nav_hint,
            Style::default().fg(Color::DarkGray),
        )]))
    };

    let piechart = PieChart::new(slices)
        .block(block)
        .show_legend(true)
        .show_percentages(true)
        .legend_marker(current_style.marker);

    frame.render_widget(piechart, area);
}
