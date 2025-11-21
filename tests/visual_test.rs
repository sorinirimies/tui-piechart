use ratatui::{backend::TestBackend, buffer::Buffer, style::Color, Terminal};
use tui_piechart::{PieChart, PieSlice};

#[test]
fn test_pie_chart_rendering() {
    let backend = TestBackend::new(60, 20);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            let slices = vec![
                PieSlice::new("Rust", 45.0, Color::Red),
                PieSlice::new("Go", 30.0, Color::Blue),
                PieSlice::new("Python", 25.0, Color::Green),
            ];

            let piechart = PieChart::new(slices)
                .show_legend(true)
                .show_percentages(true);

            frame.render_widget(piechart, frame.area());
        })
        .unwrap();

    // Print the buffer for visual inspection
    let buffer = terminal.backend().buffer();
    print_buffer(buffer);

    // Basic assertions - just verify some content exists
    assert!(buffer_contains_char(buffer, '●'));
}

#[test]
fn test_pie_chart_with_different_sizes() {
    println!("\n=== Small (30x10) ===");
    test_size(30, 10);

    println!("\n=== Medium (60x20) ===");
    test_size(60, 20);

    println!("\n=== Large (80x30) ===");
    test_size(80, 30);
}

#[test]
fn test_pie_chart_grid_layout() {
    println!("\n=== 2x2 Grid Layout (80x40) ===");
    let backend = TestBackend::new(80, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            use ratatui::layout::{Constraint, Layout};

            let rows = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(frame.area());

            let top_row =
                Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(rows[0]);

            let bottom_row =
                Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(rows[1]);

            // Top-left: Programming Languages
            let slices1 = vec![
                PieSlice::new("Rust", 45.0, Color::Red),
                PieSlice::new("Go", 30.0, Color::Blue),
                PieSlice::new("Python", 25.0, Color::Green),
            ];
            let chart1 = PieChart::new(slices1)
                .show_legend(true)
                .show_percentages(true);
            frame.render_widget(chart1, top_row[0]);

            // Top-right: Market Share
            let slices2 = vec![
                PieSlice::new("Product A", 40.0, Color::Magenta),
                PieSlice::new("Product B", 35.0, Color::Yellow),
                PieSlice::new("Product C", 25.0, Color::Cyan),
            ];
            let chart2 = PieChart::new(slices2)
                .show_legend(true)
                .show_percentages(true);
            frame.render_widget(chart2, top_row[1]);

            // Bottom-left: Time Allocation
            let slices3 = vec![
                PieSlice::new("Work", 50.0, Color::LightBlue),
                PieSlice::new("Sleep", 30.0, Color::LightMagenta),
                PieSlice::new("Leisure", 20.0, Color::LightGreen),
            ];
            let chart3 = PieChart::new(slices3)
                .show_legend(true)
                .show_percentages(true);
            frame.render_widget(chart3, bottom_row[0]);

            // Bottom-right: Budget Distribution
            let slices4 = vec![
                PieSlice::new("Housing", 35.0, Color::LightRed),
                PieSlice::new("Food", 25.0, Color::LightYellow),
                PieSlice::new("Transport", 20.0, Color::LightCyan),
                PieSlice::new("Other", 20.0, Color::Gray),
            ];
            let chart4 = PieChart::new(slices4)
                .show_legend(true)
                .show_percentages(true);
            frame.render_widget(chart4, bottom_row[1]);
        })
        .unwrap();

    print_buffer(terminal.backend().buffer());
}

#[test]
fn test_custom_symbols() {
    println!("\n=== Custom Symbols Test ===");
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            use ratatui::layout::{Constraint, Layout};
            use tui_piechart::symbols;

            let cols = Layout::horizontal([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .split(frame.area());

            // Default symbols
            let slices1 = vec![
                PieSlice::new("A", 50.0, Color::Red),
                PieSlice::new("B", 50.0, Color::Blue),
            ];
            let chart1 = PieChart::new(slices1)
                .pie_char(symbols::PIE_CHAR)
                .legend_marker(symbols::LEGEND_MARKER);
            frame.render_widget(chart1, cols[0]);

            // Custom single character
            let slices2 = vec![
                PieSlice::new("X", 60.0, Color::Green),
                PieSlice::new("Y", 40.0, Color::Yellow),
            ];
            let chart2 = PieChart::new(slices2).pie_char('★').legend_marker("→");
            frame.render_widget(chart2, cols[1]);

            // Custom multi-character marker
            let slices3 = vec![
                PieSlice::new("One", 70.0, Color::Magenta),
                PieSlice::new("Two", 30.0, Color::Cyan),
            ];
            let chart3 = PieChart::new(slices3).pie_char('◆').legend_marker("-->");
            frame.render_widget(chart3, cols[2]);
        })
        .unwrap();

    print_buffer(terminal.backend().buffer());
}

#[test]
fn test_star_symbols() {
    println!("\n=== Star Symbols Test ===");
    let backend = TestBackend::new(80, 25);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            use ratatui::layout::{Constraint, Layout};
            use tui_piechart::symbols;

            let cols = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(frame.area());

            // Filled stars
            let slices1 = vec![
                PieSlice::new("Rust", 45.0, Color::Red),
                PieSlice::new("Go", 30.0, Color::Blue),
                PieSlice::new("Python", 25.0, Color::Green),
            ];
            let chart1 = PieChart::new(slices1)
                .pie_char(symbols::PIE_CHAR_STAR)
                .legend_marker(symbols::LEGEND_MARKER_STAR);
            frame.render_widget(chart1, cols[0]);

            // Hollow stars
            let slices2 = vec![
                PieSlice::new("JavaScript", 40.0, Color::Yellow),
                PieSlice::new("TypeScript", 35.0, Color::Cyan),
                PieSlice::new("Java", 25.0, Color::Magenta),
            ];
            let chart2 = PieChart::new(slices2)
                .pie_char(symbols::PIE_CHAR_WHITE_STAR)
                .legend_marker(symbols::LEGEND_MARKER_WHITE_STAR);
            frame.render_widget(chart2, cols[1]);
        })
        .unwrap();

    print_buffer(terminal.backend().buffer());
}

#[test]
fn test_high_resolution_mode() {
    println!("\n=== High Resolution Mode Test ===");
    let backend = TestBackend::new(60, 25);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            use ratatui::layout::{Constraint, Layout};

            let cols = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(frame.area());

            // Standard resolution (left)
            let slices1 = vec![
                PieSlice::new("A", 50.0, Color::Red),
                PieSlice::new("B", 50.0, Color::Blue),
            ];
            let chart1 = PieChart::new(slices1).show_legend(true);
            frame.render_widget(chart1, cols[0]);

            // High resolution (right)
            let slices2 = vec![
                PieSlice::new("A", 50.0, Color::Red),
                PieSlice::new("B", 50.0, Color::Blue),
            ];
            let chart2 = PieChart::new(slices2)
                .show_legend(true)
                .high_resolution(true);
            frame.render_widget(chart2, cols[1]);
        })
        .unwrap();

    println!("Standard (left) vs High-Res (right):");
    print_buffer(terminal.backend().buffer());

    // Verify high-res uses braille characters
    let buffer = terminal.backend().buffer();
    let mut found_braille = false;
    for y in 0..buffer.area().height {
        for x in 30..buffer.area().width {
            // Check right side for braille characters (U+2800-U+28FF)
            if let Some(cell) = buffer.cell((x, y)) {
                let ch = cell.symbol().chars().next().unwrap_or(' ');
                if ('\u{2800}'..='\u{28FF}').contains(&ch) {
                    found_braille = true;
                    break;
                }
            }
        }
        if found_braille {
            break;
        }
    }

    assert!(
        found_braille,
        "High resolution mode should use braille characters"
    );
}

fn test_size(width: u16, height: u16) {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            let slices = vec![
                PieSlice::new("A", 40.0, Color::Red),
                PieSlice::new("B", 30.0, Color::Blue),
                PieSlice::new("C", 30.0, Color::Green),
            ];

            let piechart = PieChart::new(slices)
                .show_legend(true)
                .show_percentages(true);

            frame.render_widget(piechart, frame.area());
        })
        .unwrap();

    print_buffer(terminal.backend().buffer());
}

fn print_buffer(buffer: &Buffer) {
    let area = buffer.area();
    println!("\n┌{}┐", "─".repeat(area.width as usize));

    for y in 0..area.height {
        print!("│");
        for x in 0..area.width {
            let cell = buffer.cell((x, y)).unwrap();
            print!("{}", cell.symbol());
        }
        println!("│");
    }

    println!("└{}┘", "─".repeat(area.width as usize));
}

#[test]
fn test_border_styles() {
    println!("\n=== Border Styles Test ===");
    let backend = TestBackend::new(160, 60);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            use ratatui::layout::{Constraint, Layout};
            use tui_piechart::border_style::BorderStyle;

            let rows = Layout::vertical([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(frame.area());

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

            let row4 = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(rows[3]);

            let slices = vec![
                PieSlice::new("A", 50.0, Color::Red),
                PieSlice::new("B", 30.0, Color::Blue),
                PieSlice::new("C", 20.0, Color::Green),
            ];

            // Standard
            let chart1 = PieChart::new(slices.clone())
                .block(BorderStyle::Standard.block().title(" Standard "))
                .show_legend(true);
            frame.render_widget(chart1, row1[0]);

            // Rounded
            let chart2 = PieChart::new(slices.clone())
                .block(BorderStyle::Rounded.block().title(" Rounded "))
                .show_legend(true);
            frame.render_widget(chart2, row1[1]);

            // Dashed
            let chart3 = PieChart::new(slices.clone())
                .block(BorderStyle::Dashed.block().title(" Dashed "))
                .show_legend(true);
            frame.render_widget(chart3, row1[2]);

            // RoundedDashed
            let chart4 = PieChart::new(slices.clone())
                .block(BorderStyle::RoundedDashed.block().title(" RoundedDashed "))
                .show_legend(true);
            frame.render_widget(chart4, row2[0]);

            // CornerGapped
            let chart5 = PieChart::new(slices.clone())
                .block(BorderStyle::CornerGapped.block().title(" CornerGapped "))
                .show_legend(true);
            frame.render_widget(chart5, row2[1]);

            // RoundedCornerGapped
            let chart6 = PieChart::new(slices.clone())
                .block(
                    BorderStyle::RoundedCornerGapped
                        .block()
                        .title(" RoundedCornerGapped "),
                )
                .show_legend(true);
            frame.render_widget(chart6, row2[2]);

            // DoubleLineStandard
            let chart7 = PieChart::new(slices.clone())
                .block(
                    BorderStyle::DoubleLineStandard
                        .block()
                        .title(" DoubleLineStandard "),
                )
                .show_legend(true);
            frame.render_widget(chart7, row3[0]);

            // DoubleLineRounded
            let chart8 = PieChart::new(slices.clone())
                .block(
                    BorderStyle::DoubleLineRounded
                        .block()
                        .title(" DoubleLineRounded "),
                )
                .show_legend(true);
            frame.render_widget(chart8, row3[1]);

            // Thick
            let chart9 = PieChart::new(slices.clone())
                .block(BorderStyle::Thick.block().title(" Thick "))
                .show_legend(true);
            frame.render_widget(chart9, row3[2]);

            // ThickRounded
            let chart10 = PieChart::new(slices.clone())
                .block(BorderStyle::ThickRounded.block().title(" ThickRounded "))
                .show_legend(true);
            frame.render_widget(chart10, row4[0]);

            // ThickDashed
            let chart11 = PieChart::new(slices)
                .block(BorderStyle::ThickDashed.block().title(" ThickDashed "))
                .show_legend(true);
            frame.render_widget(chart11, row4[1]);
        })
        .unwrap();

    print_buffer(terminal.backend().buffer());

    // Basic assertion - verify border characters exist
    let buffer = terminal.backend().buffer();
    assert!(buffer_contains_char(buffer, '─') || buffer_contains_char(buffer, '│'));
}

fn buffer_contains_char(buffer: &Buffer, c: char) -> bool {
    let area = buffer.area();
    for y in 0..area.height {
        for x in 0..area.width {
            if let Some(cell) = buffer.cell((x, y)) {
                if cell.symbol().contains(c) {
                    return true;
                }
            }
        }
    }
    false
}
