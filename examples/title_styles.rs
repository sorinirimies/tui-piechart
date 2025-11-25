//! # Title Styles & Positioning Example
//!
//! This example demonstrates all available title font styles using Unicode character variants
//! and title positioning options (alignment and vertical position).
//!
//! **Controls:**
//! - ↑/↓ or k/j - Navigate between different title styles
//! - ←/→ or h/l - Change horizontal alignment (Start/Center/End)
//! - Space or Tab - Toggle vertical position (Top/Bottom)
//! - m - Toggle mode (Styles/Positioning)
//! - q/Esc - Quit
//!
//! Run with: cargo run --example title_styles

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::Padding,
    DefaultTerminal, Frame,
};
use tui_piechart::{
    border_style::BorderStyle,
    title::{BlockExt, TitleAlignment, TitlePosition, TitleStyle},
    PieChart, PieSlice,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DemoMode {
    Styles,
    Positioning,
}

struct App {
    mode: DemoMode,
    selected_style: usize,
    selected_alignment: usize,
    selected_position: usize,
    styles: Vec<(TitleStyle, &'static str, &'static str)>,
    alignments: Vec<(TitleAlignment, &'static str)>,
    positions: Vec<(TitlePosition, &'static str)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            mode: DemoMode::Styles,
            selected_style: 0,
            selected_alignment: 1, // Start with Center
            selected_position: 0,  // Start with Top
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
            alignments: vec![
                (TitleAlignment::Start, "Start (Left)"),
                (TitleAlignment::Center, "Center"),
                (TitleAlignment::End, "End (Right)"),
            ],
            positions: vec![
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
                KeyCode::Char('m') => {
                    app.mode = match app.mode {
                        DemoMode::Styles => DemoMode::Positioning,
                        DemoMode::Positioning => DemoMode::Styles,
                    };
                }
                KeyCode::Char(' ') | KeyCode::Tab => {
                    app.selected_position = (app.selected_position + 1) % app.positions.len();
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if app.mode == DemoMode::Styles
                        && app.selected_style > 0 {
                            app.selected_style -= 1;
                        }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if app.mode == DemoMode::Styles
                        && app.selected_style < app.styles.len() - 1 {
                            app.selected_style += 1;
                        }
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    if app.selected_alignment > 0 {
                        app.selected_alignment -= 1;
                    }
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    if app.selected_alignment < app.alignments.len() - 1 {
                        app.selected_alignment += 1;
                    }
                }
                KeyCode::Home => {
                    if app.mode == DemoMode::Styles {
                        app.selected_style = 0;
                    }
                }
                KeyCode::End => {
                    if app.mode == DemoMode::Styles {
                        app.selected_style = app.styles.len() - 1;
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
        Constraint::Length(6), // Header
        Constraint::Min(0),    // Content
        Constraint::Length(3), // Footer
    ])
    .split(frame.area());

    // Header
    render_header(frame, main_layout[0], app);

    // Main content area
    render_content(frame, main_layout[1], app);

    // Footer
    render_footer(frame, main_layout[2], app);
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let mode_text = match app.mode {
        DemoMode::Styles => "Font Styles",
        DemoMode::Positioning => "Positioning",
    };

    let block = BorderStyle::Rounded
        .block()
        .title(format!(" Title Demo - {} Mode ", mode_text))
        .title_alignment_horizontal(TitleAlignment::Center)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::horizontal(1));

    let text = match app.mode {
        DemoMode::Styles => vec![
            Line::from("Demonstrates Unicode font styles for chart titles"),
            Line::from(vec![
                Span::styled("Mode: ", Style::default().fg(Color::Gray)),
                Span::styled("Font Styles", Style::default().fg(Color::Yellow).bold()),
                Span::raw("  Press "),
                Span::styled("m", Style::default().fg(Color::Cyan).bold()),
                Span::raw(" to switch to Positioning"),
            ]),
        ],
        DemoMode::Positioning => vec![
            Line::from("Demonstrates title alignment and position options"),
            Line::from(vec![
                Span::styled("Mode: ", Style::default().fg(Color::Gray)),
                Span::styled("Positioning", Style::default().fg(Color::Green).bold()),
                Span::raw("  Press "),
                Span::styled("m", Style::default().fg(Color::Cyan).bold()),
                Span::raw(" to switch to Styles"),
            ]),
        ],
    };

    frame.render_widget(ratatui::widgets::Paragraph::new(text).block(block), area);
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let block = BorderStyle::Rounded
        .block()
        .border_style(Style::default().fg(Color::DarkGray))
        .padding(Padding::horizontal(1));

    let text = match app.mode {
        DemoMode::Styles => Line::from(vec![
            Span::styled("↑/↓", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" Styles  "),
            Span::styled("←/→", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" Align  "),
            Span::styled("Space", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" Position  "),
            Span::styled("m", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" Mode  "),
            Span::styled("q", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" Quit"),
        ]),
        DemoMode::Positioning => Line::from(vec![
            Span::styled("←/→", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" Alignment  "),
            Span::styled("Space", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" Position  "),
            Span::styled("m", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" Mode  "),
            Span::styled("q", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" Quit"),
        ]),
    };

    frame.render_widget(
        ratatui::widgets::Paragraph::new(text)
            .block(block)
            .centered(),
        area,
    );
}

fn render_content(frame: &mut Frame, area: Rect, app: &App) {
    let (style, style_name, description) = app.styles[app.selected_style];
    let (alignment, alignment_name) = app.alignments[app.selected_alignment];
    let (position, position_name) = app.positions[app.selected_position];

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

    let info_line = match app.mode {
        DemoMode::Styles => Line::from(vec![
            Span::styled("Style: ", Style::default().fg(Color::Gray)),
            Span::styled(style_name, Style::default().fg(Color::Yellow).bold()),
            Span::raw("  "),
            Span::styled(description, Style::default().fg(Color::Gray).italic()),
        ]),
        DemoMode::Positioning => Line::from(vec![
            Span::styled("Alignment: ", Style::default().fg(Color::Gray)),
            Span::styled(alignment_name, Style::default().fg(Color::Cyan).bold()),
            Span::raw("  "),
            Span::styled("Position: ", Style::default().fg(Color::Gray)),
            Span::styled(position_name, Style::default().fg(Color::Green).bold()),
        ]),
    };

    let block = BorderStyle::Rounded
        .block()
        .title(styled_title)
        .title_alignment_horizontal(alignment)
        .title_vertical_position(position)
        .title_bottom(info_line)
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    let chart = PieChart::new(slices)
        .block(block)
        .legend_position(tui_piechart::LegendPosition::Right);

    frame.render_widget(chart, area);
}
