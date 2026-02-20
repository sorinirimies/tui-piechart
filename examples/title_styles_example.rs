//! # Title Styles Example
//!
//! This example demonstrates all available title font styles using Unicode character variants.
//!
//! **Controls:**
//! - ↑/↓ or k/j - Navigate between different title styles
//! - q/Esc - Quit
//!
//! Run with: cargo run --example title_styles_example

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Padding,
    DefaultTerminal, Frame,
};
use tui_piechart::{
    border_style::BorderStyle,
    title::{BlockExt, TitleAlignment, TitleStyle},
    PieChart, PieSlice,
};

struct App {
    selected: usize,
    styles: Vec<(TitleStyle, &'static str, &'static str)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            selected: 0,
            styles: vec![
                (
                    TitleStyle::Normal,
                    "Normal",
                    "Regular text, no transformation",
                ),
                (
                    TitleStyle::Bold,
                    "Bold",
                    "Bold Mathematical Unicode characters",
                ),
                (
                    TitleStyle::Italic,
                    "Italic",
                    "Italic Mathematical Unicode characters",
                ),
                (
                    TitleStyle::BoldItalic,
                    "Bold Italic",
                    "Combined bold and italic styling",
                ),
                (TitleStyle::Script, "Script", "Flowing cursive script style"),
                (
                    TitleStyle::BoldScript,
                    "Bold Script",
                    "Bold weight script style",
                ),
                (
                    TitleStyle::SansSerif,
                    "Sans Serif",
                    "Clean sans-serif style",
                ),
                (
                    TitleStyle::BoldSansSerif,
                    "Bold Sans",
                    "Bold sans-serif style",
                ),
                (
                    TitleStyle::ItalicSansSerif,
                    "Italic Sans",
                    "Italic sans-serif style",
                ),
                (
                    TitleStyle::Monospace,
                    "Monospace",
                    "Fixed-width monospace style",
                ),
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
                    if app.selected > 0 {
                        app.selected -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if app.selected < app.styles.len() - 1 {
                        app.selected += 1;
                    }
                }
                KeyCode::Home => {
                    app.selected = 0;
                }
                KeyCode::End => {
                    app.selected = app.styles.len() - 1;
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

    // Header
    render_header(frame, main_layout[0]);

    // Main content area
    render_main_chart(frame, main_layout[1], app);

    // Footer
    render_footer(frame, main_layout[2]);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let block = BorderStyle::Rounded
        .block()
        .title(" Title Font Styles Demo ")
        .title_alignment_horizontal(TitleAlignment::Center)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::horizontal(1));

    let text = vec![
        Line::from("Demonstrates Unicode font styles for chart titles"),
        Line::from(vec![
            Span::raw("Shows how different "),
            Span::styled(
                "Unicode character sets",
                Style::default().fg(Color::Yellow).bold(),
            ),
            Span::raw(" can create visual font styles"),
        ]),
        Line::from(vec![
            Span::styled("Note: ", Style::default().fg(Color::Red).bold()),
            Span::raw("Font rendering depends on your terminal and font support"),
        ]),
    ];

    frame.render_widget(ratatui::widgets::Paragraph::new(text).block(block), area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let block = BorderStyle::Rounded
        .block()
        .border_style(Style::default().fg(Color::DarkGray))
        .padding(Padding::horizontal(1));

    let text = Line::from(vec![
        Span::styled("↑/↓", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" or "),
        Span::styled("k/j", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Navigate  "),
        Span::styled("Home/End", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" First/Last  "),
        Span::styled("q/Esc", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Quit"),
    ]);

    frame.render_widget(
        ratatui::widgets::Paragraph::new(text)
            .block(block)
            .centered(),
        area,
    );
}

fn render_main_chart(frame: &mut Frame, area: Rect, app: &App) {
    let (style, name, description) = app.styles[app.selected];

    // Create styled title text
    let title_text = "Sales Report 2024".to_string();
    let styled_title = style.apply(&title_text);

    // Sample data
    let slices = vec![
        PieSlice::new("Electronics", 35.0, Color::Cyan),
        PieSlice::new("Clothing", 25.0, Color::Magenta),
        PieSlice::new("Food", 20.0, Color::Green),
        PieSlice::new("Books", 12.0, Color::Yellow),
        PieSlice::new("Other", 8.0, Color::Blue),
    ];

    let block = BorderStyle::Rounded
        .block()
        .title(styled_title)
        .title_alignment_horizontal(TitleAlignment::Center)
        .border_style(
            if app.selected == 0 {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Cyan)
            }
            .add_modifier(Modifier::BOLD),
        );

    let chart = PieChart::new(slices)
        .block(block)
        .legend_position(tui_piechart::LegendPosition::Right);

    // Info panel below chart
    let content_layout = Layout::vertical([
        Constraint::Min(0),    // Chart
        Constraint::Length(8), // Info panel
    ])
    .split(area);

    frame.render_widget(chart, content_layout[0]);

    // Info panel
    render_info_panel(
        frame,
        content_layout[1],
        app.selected + 1,
        app.styles.len(),
        name,
        description,
        style,
    );
}

fn render_info_panel(
    frame: &mut Frame,
    area: Rect,
    current: usize,
    total: usize,
    name: &str,
    description: &str,
    style: TitleStyle,
) {
    let block = BorderStyle::Rounded
        .block()
        .title(format!(" Style {}/{} ", current, total))
        .title_alignment_horizontal(TitleAlignment::Center)
        .border_style(Style::default().fg(Color::Yellow))
        .padding(Padding::horizontal(1));

    // Show examples with the current style
    let example = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let styled_example = style.apply(example);

    let text = vec![
        Line::from(vec![
            Span::styled("Style: ", Style::default().fg(Color::Cyan).bold()),
            Span::styled(name, Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("Description: ", Style::default().fg(Color::Cyan).bold()),
            Span::raw(description),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Character Preview:",
            Style::default().fg(Color::Cyan).bold(),
        )),
        Line::from(styled_example),
    ];

    frame.render_widget(ratatui::widgets::Paragraph::new(text).block(block), area);
}
