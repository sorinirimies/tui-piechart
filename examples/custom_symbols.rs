//! # Custom Symbols Example
//!
//! This example demonstrates using truly custom symbols that are NOT in the predefined list.
//! Shows how you can use ANY Unicode character for complete customization.
//!
//! Run with: cargo run --example custom_symbols

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph},
    DefaultTerminal, Frame,
};
use tui_piechart::{PieChart, PieSlice};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SymbolSet {
    pie_char: char,
    legend_marker: &'static str,
    name: &'static str,
    description: &'static str,
}

const CUSTOM_SYMBOLS: &[SymbolSet] = &[
    SymbolSet {
        pie_char: '⬤',
        legend_marker: "○",
        name: "Large Circle & Ring",
        description: "Large black circle & white ring",
    },
    SymbolSet {
        pie_char: '⬟',
        legend_marker: "◯",
        name: "Pentagon & Ring",
        description: "Pentagon & large circle",
    },
    SymbolSet {
        pie_char: '⊕',
        legend_marker: "⊗",
        name: "Circle Operators",
        description: "Circled plus & circled times",
    },
    SymbolSet {
        pie_char: '☯',
        legend_marker: "☮",
        name: "Symbols",
        description: "Yin Yang & peace sign",
    },
    SymbolSet {
        pie_char: '⚛',
        legend_marker: "☢",
        name: "Science",
        description: "Atom symbol & radioactive sign",
    },
    SymbolSet {
        pie_char: '♫',
        legend_marker: "♬",
        name: "Music",
        description: "Beamed eighth notes & sixteenth notes",
    },
    SymbolSet {
        pie_char: '☘',
        legend_marker: "❀",
        name: "Nature",
        description: "Shamrock & flower",
    },
    SymbolSet {
        pie_char: '⚙',
        legend_marker: "⚒",
        name: "Tools",
        description: "Gear & hammer and pick",
    },
];

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
                    if app.selected > 0 {
                        app.selected -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if app.selected < CUSTOM_SYMBOLS.len() - 1 {
                        app.selected += 1;
                    }
                }
                KeyCode::Home => app.selected = 0,
                KeyCode::End => app.selected = CUSTOM_SYMBOLS.len() - 1,
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
    render_content(frame, main_layout[1], app);
    render_footer(frame, main_layout[2]);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .title(" Custom Symbols - Beyond Predefined ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::horizontal(1));

    let text = vec![
        Line::from(vec![
            Span::styled("All symbols here are ", Style::default()),
            Span::styled("NOT", Style::default().fg(Color::Yellow).bold()),
            Span::styled(" in the predefined symbols list", Style::default()),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "This demonstrates the full flexibility of custom Unicode characters",
            Style::default().fg(Color::Gray),
        )),
        Line::from(Span::styled(
            "Note: Some emoji may not render in all terminals",
            Style::default().fg(Color::DarkGray).italic(),
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
        Span::styled("↑/↓", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Navigate  "),
        Span::styled("Home/End", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" First/Last  "),
        Span::styled("q", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Quit"),
    ]);

    let paragraph = Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(paragraph.block(block), area);
}

fn render_content(frame: &mut Frame, area: Rect, app: &App) {
    let layout = Layout::vertical([
        Constraint::Length(4), // Selected symbol info
        Constraint::Min(0),    // Charts grid
    ])
    .split(area);

    render_symbol_info(frame, layout[0], app);

    // 4x2 grid for 8 charts
    let rows =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).split(layout[1]);

    let cols1 = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
    .split(rows[0]);

    let cols2 = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
    .split(rows[1]);

    let areas = [
        cols1[0], cols1[1], cols1[2], cols1[3], cols2[0], cols2[1], cols2[2], cols2[3],
    ];

    for (i, &symbol_set) in CUSTOM_SYMBOLS.iter().enumerate() {
        if i < areas.len() {
            render_chart(frame, areas[i], symbol_set, i == app.selected);
        }
    }
}

fn render_symbol_info(frame: &mut Frame, area: Rect, app: &App) {
    let symbol_set = CUSTOM_SYMBOLS[app.selected];

    let block = Block::bordered()
        .title(" Selected Custom Symbol ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Yellow))
        .padding(Padding::horizontal(1));

    let text = vec![
        Line::from(vec![
            Span::styled("Name: ", Style::default().fg(Color::Gray)),
            Span::styled(symbol_set.name, Style::default().fg(Color::Yellow).bold()),
        ]),
        Line::from(vec![
            Span::styled("Description: ", Style::default().fg(Color::Gray)),
            Span::styled(symbol_set.description, Style::default().fg(Color::White)),
        ]),
    ];

    let paragraph = Paragraph::new(text).alignment(Alignment::Left);
    frame.render_widget(paragraph.block(block), area);
}

fn render_chart(frame: &mut Frame, area: Rect, symbol_set: SymbolSet, is_selected: bool) {
    let slices = vec![
        PieSlice::new("A", 40.0, Color::Red),
        PieSlice::new("B", 35.0, Color::Blue),
        PieSlice::new("C", 25.0, Color::Green),
    ];

    let border_style = if is_selected {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::bordered()
        .title(format!(" {} ", symbol_set.name))
        .title_alignment(Alignment::Center)
        .border_style(border_style)
        .padding(Padding::new(1, 1, 0, 0));

    let piechart = PieChart::new(slices)
        .block(block)
        .show_legend(true)
        .show_percentages(true)
        .pie_char(symbol_set.pie_char)
        .legend_marker(symbol_set.legend_marker);

    frame.render_widget(piechart, area);
}
