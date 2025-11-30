//! # Border Styles Example
//!
//! This example demonstrates all the available border styles for the PieChart widget.
//!
//! **Controls:**
//! - ↑/↓ or k/j - Navigate between different border styles
//! - q/Esc - Quit
//!
//! Run with: cargo run --example border_styles

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::Padding,
    DefaultTerminal, Frame,
};
use tui_piechart::{border_style::BorderStyle, PieChart, PieSlice};

struct App {
    selected: usize,
    total_styles: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            selected: 0,
            total_styles: 11,
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
                    if app.selected < app.total_styles - 1 {
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
        Constraint::Length(3), // Header
        Constraint::Min(0),    // Content
        Constraint::Length(3), // Footer
    ])
    .split(frame.area());

    // Header
    render_header(frame, main_layout[0]);

    // Content area - 4 rows for 11 charts
    let rows = Layout::vertical([
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
    .split(main_layout[1]);

    let row1 = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ])
    .split(rows[0]);

    let row2 = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ])
    .split(rows[1]);

    let row3 = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ])
    .split(rows[2]);

    let row4 =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[3]);

    // Render all 11 border style examples
    render_border_example(
        frame,
        row1[0],
        BorderStyle::Standard,
        "Standard",
        app.selected == 0,
    );
    render_border_example(
        frame,
        row1[1],
        BorderStyle::Rounded,
        "Rounded",
        app.selected == 1,
    );
    render_border_example(
        frame,
        row1[2],
        BorderStyle::Dashed,
        "Dashed",
        app.selected == 2,
    );
    render_border_example(
        frame,
        row2[0],
        BorderStyle::RoundedDashed,
        "Rounded Dashed",
        app.selected == 3,
    );
    render_border_example(
        frame,
        row2[1],
        BorderStyle::CornerGapped,
        "Corner Gapped",
        app.selected == 4,
    );
    render_border_example(
        frame,
        row2[2],
        BorderStyle::RoundedCornerGapped,
        "Rounded Corner Gapped",
        app.selected == 5,
    );
    render_border_example(
        frame,
        row3[0],
        BorderStyle::DoubleLineStandard,
        "Double Line",
        app.selected == 6,
    );
    render_border_example(
        frame,
        row3[1],
        BorderStyle::DoubleLineRounded,
        "Double Rounded",
        app.selected == 7,
    );
    render_border_example(
        frame,
        row3[2],
        BorderStyle::Thick,
        "Thick",
        app.selected == 8,
    );
    render_border_example(
        frame,
        row4[0],
        BorderStyle::ThickRounded,
        "Thick Rounded",
        app.selected == 9,
    );
    render_border_example(
        frame,
        row4[1],
        BorderStyle::ThickDashed,
        "Thick Dashed",
        app.selected == 10,
    );

    // Footer
    render_footer(frame, main_layout[2]);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let block = BorderStyle::Rounded
        .block()
        .title(" Border Styles Demo ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::horizontal(1));

    let text = ratatui::widgets::Paragraph::new("11 available border styles for PieChart blocks")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Gray));

    frame.render_widget(text.block(block), area);
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

fn render_border_example(
    frame: &mut Frame,
    area: Rect,
    border_style: BorderStyle,
    title: &str,
    is_selected: bool,
) {
    let slices = vec![
        PieSlice::new("Rust", 45.0, Color::Red),
        PieSlice::new("Go", 30.0, Color::Blue),
        PieSlice::new("Python", 25.0, Color::Green),
    ];

    let block = border_style
        .block()
        .title(format!(" {} ", title))
        .title_alignment(Alignment::Center)
        .border_style(if is_selected {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        })
        .padding(Padding::new(1, 1, 0, 0));

    let piechart = PieChart::new(slices)
        .block(block)
        .show_legend(true)
        .show_percentages(false)
        .legend_alignment(tui_piechart::LegendAlignment::Center);

    frame.render_widget(piechart, area);
}
