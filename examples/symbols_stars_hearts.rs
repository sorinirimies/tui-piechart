//! # Stars & Hearts Symbols Example
//!
//! This example demonstrates star and heart symbols from tui-piechart.
//! Shows Diamond, Star, White Star, and Heart symbols in a 2x2 grid.
//!
//! Run with: cargo run --example symbols_stars_hearts

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph},
    DefaultTerminal, Frame,
};
use tui_piechart::{symbols, PieChart, PieSlice};

#[derive(Default)]
struct App {
    selected: usize,
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
                    if app.selected >= 2 {
                        app.selected -= 2;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if app.selected < 2 {
                        app.selected += 2;
                    }
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    if app.selected % 2 == 1 {
                        app.selected -= 1;
                    }
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    if app.selected % 2 == 0 && app.selected < 3 {
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
    let main_layout = Layout::vertical([
        Constraint::Length(5), // Header
        Constraint::Min(0),    // Content
        Constraint::Length(3), // Footer
    ])
    .split(frame.area());

    render_header(frame, main_layout[0]);
    render_content(frame, main_layout[1], app);
    render_footer(frame, main_layout[2]);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .title(" Stars & Hearts - Predefined ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::horizontal(1));

    let text = vec![
        Line::from("Star and heart symbol combinations from the symbols module"),
        Line::from(""),
        Line::from(Span::styled(
            "Use arrow keys or hjkl to navigate between charts",
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
        Span::styled("↑↓←→", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" or "),
        Span::styled("hjkl", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Navigate  "),
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

    // Top-left: Diamond
    render_chart_diamond(frame, top_row[0], app.selected == 0);

    // Top-right: Star
    render_chart_star(frame, top_row[1], app.selected == 1);

    // Bottom-left: White Star
    render_chart_white_star(frame, bottom_row[0], app.selected == 2);

    // Bottom-right: Heart
    render_chart_heart(frame, bottom_row[1], app.selected == 3);
}

fn render_chart_diamond(frame: &mut Frame, area: Rect, is_selected: bool) {
    let slices = vec![
        PieSlice::new("Rust", 45.0, Color::Red),
        PieSlice::new("Go", 30.0, Color::Blue),
        PieSlice::new("Python", 25.0, Color::Green),
    ];

    let border_style = if is_selected {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::bordered()
        .title(" Diamond (◆ / ◆) ")
        .title_alignment(Alignment::Center)
        .border_style(border_style)
        .padding(Padding::new(1, 1, 0, 0));

    let piechart = PieChart::new(slices)
        .block(block)
        .show_legend(true)
        .show_percentages(true)
        .pie_char(symbols::PIE_CHAR_DIAMOND)
        .legend_marker(symbols::LEGEND_MARKER_DIAMOND);

    frame.render_widget(piechart, area);
}

fn render_chart_star(frame: &mut Frame, area: Rect, is_selected: bool) {
    let slices = vec![
        PieSlice::new("Product A", 40.0, Color::Magenta),
        PieSlice::new("Product B", 35.0, Color::Yellow),
        PieSlice::new("Product C", 25.0, Color::Cyan),
    ];

    let border_style = if is_selected {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::bordered()
        .title(" Star (★ / ★) ")
        .title_alignment(Alignment::Center)
        .border_style(border_style)
        .padding(Padding::new(1, 1, 0, 0));

    let piechart = PieChart::new(slices)
        .block(block)
        .show_legend(true)
        .show_percentages(true)
        .pie_char(symbols::PIE_CHAR_STAR)
        .legend_marker(symbols::LEGEND_MARKER_STAR);

    frame.render_widget(piechart, area);
}

fn render_chart_white_star(frame: &mut Frame, area: Rect, is_selected: bool) {
    let slices = vec![
        PieSlice::new("Work", 50.0, Color::LightBlue),
        PieSlice::new("Sleep", 30.0, Color::LightMagenta),
        PieSlice::new("Leisure", 20.0, Color::LightGreen),
    ];

    let border_style = if is_selected {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::bordered()
        .title(" White Star (☆ / ☆) ")
        .title_alignment(Alignment::Center)
        .border_style(border_style)
        .padding(Padding::new(1, 1, 0, 0));

    let piechart = PieChart::new(slices)
        .block(block)
        .show_legend(true)
        .show_percentages(true)
        .pie_char(symbols::PIE_CHAR_WHITE_STAR)
        .legend_marker(symbols::LEGEND_MARKER_WHITE_STAR);

    frame.render_widget(piechart, area);
}

fn render_chart_heart(frame: &mut Frame, area: Rect, is_selected: bool) {
    let slices = vec![
        PieSlice::new("Housing", 35.0, Color::LightRed),
        PieSlice::new("Food", 25.0, Color::LightYellow),
        PieSlice::new("Transport", 20.0, Color::LightCyan),
        PieSlice::new("Other", 20.0, Color::Gray),
    ];

    let border_style = if is_selected {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::bordered()
        .title(" Heart (♥ / ♥) ")
        .title_alignment(Alignment::Center)
        .border_style(border_style)
        .padding(Padding::new(1, 1, 0, 0));

    let piechart = PieChart::new(slices)
        .block(block)
        .show_legend(true)
        .show_percentages(true)
        .pie_char(symbols::PIE_CHAR_HEART)
        .legend_marker(symbols::LEGEND_MARKER_HEART);

    frame.render_widget(piechart, area);
}
