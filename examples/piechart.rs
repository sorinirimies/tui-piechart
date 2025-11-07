//! # PieChart Widget Example
//!
//! This example demonstrates the `tui-piechart` widget with two modes:
//!
//! ## Interactive Mode (Default)
//! - Navigate with ↑/↓ or k/j to select different chart types
//! - Press Tab to switch to API Showcase
//! - Press q or Esc to quit
//!
//! ## API Showcase Mode
//! - View all public API methods and features
//! - Press Tab to return to Interactive mode
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
use tui_piechart::{symbols, PieChart, PieSlice};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    Interactive,
    ApiShowcase,
}

struct App {
    selected: usize,
    mode: ViewMode,
}

impl Default for App {
    fn default() -> Self {
        Self {
            selected: 0,
            mode: ViewMode::Interactive,
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
                KeyCode::Tab => {
                    app.mode = match app.mode {
                        ViewMode::Interactive => ViewMode::ApiShowcase,
                        ViewMode::ApiShowcase => ViewMode::Interactive,
                    };
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if app.mode == ViewMode::Interactive && app.selected > 0 {
                        app.selected -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if app.mode == ViewMode::Interactive && app.selected < 3 {
                        app.selected += 1;
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app: &App) {
    match app.mode {
        ViewMode::Interactive => render_interactive(frame, app),
        ViewMode::ApiShowcase => render_api_showcase(frame, app),
    }
}

fn render_interactive(frame: &mut Frame, app: &App) {
    let main_layout = Layout::vertical([
        Constraint::Length(3), // Header
        Constraint::Min(0),    // Content
        Constraint::Length(3), // Footer
    ])
    .split(frame.area());

    // Header
    render_header(frame, main_layout[0], app);

    // Content area - 2x2 grid of pie charts
    let rows = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[1]);

    let top_row =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[0]);

    let bottom_row =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[1]);

    // Render 4 different pie chart examples
    render_programming_languages(frame, top_row[0], app.selected == 0);
    render_market_share(frame, top_row[1], app.selected == 1);
    render_time_allocation(frame, bottom_row[0], app.selected == 2);
    render_budget_distribution(frame, bottom_row[1], app.selected == 3);

    // Footer
    render_footer(frame, main_layout[2], app);
}

fn render_api_showcase(frame: &mut Frame, app: &App) {
    let layout = Layout::vertical([
        Constraint::Length(3),  // Header
        Constraint::Length(15), // Basic pie chart showcase
        Constraint::Length(15), // Styled pie chart showcase
        Constraint::Min(0),     // Custom symbols showcase
        Constraint::Length(3),  // Footer
    ])
    .split(frame.area());

    render_header(frame, layout[0], app);
    render_showcase_basic(frame, layout[1]);
    render_showcase_styled(frame, layout[2]);
    render_showcase_custom(frame, layout[3]);
    render_footer(frame, layout[4], app);
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered()
        .title(match app.mode {
            ViewMode::Interactive => " tui-piechart Demo - Interactive Mode ",
            ViewMode::ApiShowcase => " tui-piechart Demo - API Showcase ",
        })
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::horizontal(1));

    let text = Paragraph::new("A customizable pie chart widget for Ratatui")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Gray));

    frame.render_widget(text.block(block), area);
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered()
        .border_style(Style::default().fg(Color::DarkGray))
        .padding(Padding::horizontal(1));

    let text = match app.mode {
        ViewMode::Interactive => Line::from(vec![
            Span::styled("↑/↓", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" Navigate  "),
            Span::styled("Tab", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" API Showcase  "),
            Span::styled("q", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" Quit"),
        ]),
        ViewMode::ApiShowcase => Line::from(vec![
            Span::styled("Tab", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" Back to Interactive  "),
            Span::styled("q", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" Quit"),
        ]),
    };

    let paragraph = Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(paragraph.block(block), area);
}

fn render_programming_languages(frame: &mut Frame, area: Rect, is_selected: bool) {
    let slices = vec![
        PieSlice::new("Rust", 45.0, Color::Red),
        PieSlice::new("Go", 30.0, Color::Blue),
        PieSlice::new("Python", 25.0, Color::Green),
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

fn render_market_share(frame: &mut Frame, area: Rect, is_selected: bool) {
    let slices = vec![
        PieSlice::new("Product A", 40.0, Color::Magenta),
        PieSlice::new("Product B", 35.0, Color::Yellow),
        PieSlice::new("Product C", 25.0, Color::Cyan),
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

fn render_time_allocation(frame: &mut Frame, area: Rect, is_selected: bool) {
    let slices = vec![
        PieSlice::new("Work", 50.0, Color::LightBlue),
        PieSlice::new("Sleep", 30.0, Color::LightMagenta),
        PieSlice::new("Leisure", 20.0, Color::LightGreen),
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

fn render_budget_distribution(frame: &mut Frame, area: Rect, is_selected: bool) {
    let slices = vec![
        PieSlice::new("Housing", 35.0, Color::LightRed),
        PieSlice::new("Food", 25.0, Color::LightYellow),
        PieSlice::new("Transport", 20.0, Color::LightCyan),
        PieSlice::new("Other", 20.0, Color::Gray),
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

// API Showcase rendering functions

fn render_showcase_basic(frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .title(" Basic Usage: new(), default(), slices() ")
        .border_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ])
    .split(inner);

    // Basic pie chart
    let slices1 = vec![
        PieSlice::new("A", 60.0, Color::Red),
        PieSlice::new("B", 40.0, Color::Blue),
    ];
    let pie1 = PieChart::new(slices1);
    frame.render_widget(pie1, layout[0]);

    // Using default
    let slices2 = vec![
        PieSlice::new("X", 50.0, Color::Green),
        PieSlice::new("Y", 50.0, Color::Yellow),
    ];
    let pie2 = PieChart::default().slices(slices2);
    frame.render_widget(pie2, layout[1]);

    // With block
    let slices3 = vec![
        PieSlice::new("One", 70.0, Color::Magenta),
        PieSlice::new("Two", 30.0, Color::Cyan),
    ];
    let pie3 = PieChart::new(slices3).block(Block::bordered().title("Chart"));
    frame.render_widget(pie3, layout[2]);
}

fn render_showcase_styled(frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .title(" Styling: show_legend(), show_percentages(), pie_char() ")
        .border_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ])
    .split(inner);

    // Without legend
    let slices1 = vec![
        PieSlice::new("A", 60.0, Color::Red),
        PieSlice::new("B", 40.0, Color::Blue),
    ];
    let pie1 = PieChart::new(slices1).show_legend(false);
    frame.render_widget(pie1, layout[0]);

    // Without percentages
    let slices2 = vec![
        PieSlice::new("X", 50.0, Color::Green),
        PieSlice::new("Y", 50.0, Color::Yellow),
    ];
    let pie2 = PieChart::new(slices2).show_percentages(false);
    frame.render_widget(pie2, layout[1]);

    // Custom character
    let slices3 = vec![
        PieSlice::new("1", 70.0, Color::Magenta),
        PieSlice::new("2", 30.0, Color::Cyan),
    ];
    let pie3 = PieChart::new(slices3).pie_char('█');
    frame.render_widget(pie3, layout[2]);
}

fn render_showcase_custom(frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .title(" Custom Symbols: PIE_CHAR_*, multiple slices ")
        .border_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
    .split(inner);

    // Circle symbol
    let slices1 = vec![
        PieSlice::new("A", 40.0, Color::Red),
        PieSlice::new("B", 30.0, Color::Blue),
        PieSlice::new("C", 30.0, Color::Green),
    ];
    let pie1 = PieChart::new(slices1).pie_char(symbols::PIE_CHAR_CIRCLE);
    frame.render_widget(pie1, layout[0]);

    // Square symbol
    let slices2 = vec![
        PieSlice::new("X", 25.0, Color::Yellow),
        PieSlice::new("Y", 25.0, Color::Magenta),
        PieSlice::new("Z", 50.0, Color::Cyan),
    ];
    let pie2 = PieChart::new(slices2).pie_char(symbols::PIE_CHAR_SQUARE);
    frame.render_widget(pie2, layout[1]);

    // Diamond symbol
    let slices3 = vec![
        PieSlice::new("1", 33.0, Color::LightRed),
        PieSlice::new("2", 33.0, Color::LightBlue),
        PieSlice::new("3", 34.0, Color::LightGreen),
    ];
    let pie3 = PieChart::new(slices3).pie_char(symbols::PIE_CHAR_DIAMOND);
    frame.render_widget(pie3, layout[2]);

    // Block symbol with many slices
    let slices4 = vec![
        PieSlice::new("A", 20.0, Color::Red),
        PieSlice::new("B", 20.0, Color::Blue),
        PieSlice::new("C", 20.0, Color::Green),
        PieSlice::new("D", 20.0, Color::Yellow),
        PieSlice::new("E", 20.0, Color::Magenta),
    ];
    let pie4 = PieChart::new(slices4).pie_char(symbols::PIE_CHAR_BLOCK);
    frame.render_widget(pie4, layout[3]);
}
