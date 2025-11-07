//! # tui-piechart
//!
//! A customizable pie chart widget for [Ratatui](https://github.com/ratatui/ratatui) TUI applications.
//!
//! ## Features
//!
//! - 🥧 Simple pie chart with customizable slices
//! - 🎨 Customizable colors for each slice
//! - 🔤 Labels and percentages
//! - 📊 Legend support
//! - 📦 Optional block wrapper
//! - ✨ Custom symbols for pie chart and legend
//! - ⚡ Zero-cost abstractions
//!
//! ## Examples
//!
//! Basic usage:
//!
//! ```no_run
//! use ratatui::style::Color;
//! use tui_piechart::{PieChart, PieSlice};
//!
//! let slices = vec![
//!     PieSlice::new("Rust", 45.0, Color::Red),
//!     PieSlice::new("Go", 30.0, Color::Blue),
//!     PieSlice::new("Python", 25.0, Color::Green),
//! ];
//! let piechart = PieChart::new(slices);
//! ```
//!
//! With custom styling:
//!
//! ```no_run
//! use ratatui::style::{Color, Style};
//! use tui_piechart::{PieChart, PieSlice};
//!
//! let slices = vec![
//!     PieSlice::new("Rust", 45.0, Color::Red),
//!     PieSlice::new("Go", 30.0, Color::Blue),
//! ];
//! let piechart = PieChart::new(slices)
//!     .style(Style::default())
//!     .show_legend(true)
//!     .show_percentages(true);
//! ```
//!
//! With custom symbols:
//!
//! ```no_run
//! use ratatui::style::Color;
//! use tui_piechart::{PieChart, PieSlice, symbols};
//!
//! let slices = vec![
//!     PieSlice::new("Rust", 45.0, Color::Red),
//!     PieSlice::new("Go", 30.0, Color::Blue),
//! ];
//!
//! // Use predefined symbols
//! let piechart = PieChart::new(slices.clone())
//!     .pie_char(symbols::PIE_CHAR_BLOCK)
//!     .legend_marker(symbols::LEGEND_MARKER_CIRCLE);
//!
//! // Or use any custom characters
//! let piechart = PieChart::new(slices)
//!     .pie_char('█')
//!     .legend_marker("→");
//! ```

#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::f64::consts::PI;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Styled};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Widget};

pub mod symbols;

/// A slice of the pie chart representing a portion of data.
///
/// Each slice has a label, a value, and a color.
///
/// # Examples
///
/// ```
/// use ratatui::style::Color;
/// use tui_piechart::PieSlice;
///
/// let slice = PieSlice::new("Rust", 45.0, Color::Red);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PieSlice<'a> {
    /// The label for this slice
    label: &'a str,
    /// The value of this slice (will be converted to percentage)
    value: f64,
    /// The color of this slice
    color: Color,
}

impl<'a> PieSlice<'a> {
    /// Creates a new pie slice with the given label, value, and color.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::style::Color;
    /// use tui_piechart::PieSlice;
    ///
    /// let slice = PieSlice::new("Rust", 45.0, Color::Red);
    /// ```
    #[must_use]
    pub const fn new(label: &'a str, value: f64, color: Color) -> Self {
        Self {
            label,
            value,
            color,
        }
    }

    /// Returns the label of this slice.
    #[must_use]
    pub const fn label(&self) -> &'a str {
        self.label
    }

    /// Returns the value of this slice.
    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }

    /// Returns the color of this slice.
    #[must_use]
    pub const fn color(&self) -> Color {
        self.color
    }
}

/// A widget that displays a pie chart.
///
/// A `PieChart` displays data as slices of a circle, where each slice represents
/// a proportion of the total.
///
/// # Examples
///
/// ```
/// use ratatui::style::Color;
/// use tui_piechart::{PieChart, PieSlice};
///
/// let slices = vec![
///     PieSlice::new("Rust", 45.0, Color::Red),
///     PieSlice::new("Go", 30.0, Color::Blue),
///     PieSlice::new("Python", 25.0, Color::Green),
/// ];
/// let piechart = PieChart::new(slices);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PieChart<'a> {
    /// The slices of the pie chart
    slices: Vec<PieSlice<'a>>,
    /// Optional block to wrap the pie chart
    block: Option<Block<'a>>,
    /// Base style for the entire widget
    style: Style,
    /// Whether to show the legend
    show_legend: bool,
    /// Whether to show percentages on slices
    show_percentages: bool,
    /// The character to use for drawing the pie chart
    pie_char: char,
    /// The marker to use for legend items
    legend_marker: &'a str,
}

impl Default for PieChart<'_> {
    /// Returns a default `PieChart` widget.
    ///
    /// The default widget has:
    /// - No slices
    /// - No block
    /// - Default style
    /// - Legend shown
    /// - Percentages shown
    /// - Default pie character (●)
    /// - Default legend marker (■)
    fn default() -> Self {
        Self {
            slices: Vec::new(),
            block: None,
            style: Style::default(),
            show_legend: true,
            show_percentages: true,
            pie_char: symbols::PIE_CHAR,
            legend_marker: symbols::LEGEND_MARKER,
        }
    }
}

impl<'a> PieChart<'a> {
    /// Creates a new `PieChart` with the given slices.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::style::Color;
    /// use tui_piechart::{PieChart, PieSlice};
    ///
    /// let slices = vec![
    ///     PieSlice::new("Rust", 45.0, Color::Red),
    ///     PieSlice::new("Go", 30.0, Color::Blue),
    /// ];
    /// let piechart = PieChart::new(slices);
    /// ```
    #[must_use]
    pub fn new(slices: Vec<PieSlice<'a>>) -> Self {
        Self {
            slices,
            ..Default::default()
        }
    }

    /// Sets the slices of the pie chart.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::style::Color;
    /// use tui_piechart::{PieChart, PieSlice};
    ///
    /// let slices = vec![
    ///     PieSlice::new("Rust", 45.0, Color::Red),
    /// ];
    /// let piechart = PieChart::default().slices(slices);
    /// ```
    #[must_use]
    pub fn slices(mut self, slices: Vec<PieSlice<'a>>) -> Self {
        self.slices = slices;
        self
    }

    /// Wraps the pie chart with the given block.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::style::Color;
    /// use ratatui::widgets::Block;
    /// use tui_piechart::{PieChart, PieSlice};
    ///
    /// let slices = vec![PieSlice::new("Rust", 45.0, Color::Red)];
    /// let piechart = PieChart::new(slices)
    ///     .block(Block::bordered().title("Statistics"));
    /// ```
    #[must_use]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Sets the base style of the widget.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::style::{Color, Style};
    /// use tui_piechart::PieChart;
    ///
    /// let piechart = PieChart::default()
    ///     .style(Style::default().fg(Color::White));
    /// ```
    #[must_use]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Sets whether to show the legend.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_piechart::PieChart;
    ///
    /// let piechart = PieChart::default().show_legend(true);
    /// ```
    #[must_use]
    pub const fn show_legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    /// Sets whether to show percentages on slices.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_piechart::PieChart;
    ///
    /// let piechart = PieChart::default().show_percentages(true);
    /// ```
    #[must_use]
    pub const fn show_percentages(mut self, show: bool) -> Self {
        self.show_percentages = show;
        self
    }

    /// Sets the character used to draw the pie chart.
    ///
    /// You can use any Unicode character for custom visualization.
    ///
    /// # Examples
    ///
    /// Using a predefined symbol:
    ///
    /// ```
    /// use tui_piechart::{PieChart, symbols};
    ///
    /// let piechart = PieChart::default()
    ///     .pie_char(symbols::PIE_CHAR_BLOCK);
    /// ```
    ///
    /// Using a custom character:
    ///
    /// ```
    /// use tui_piechart::PieChart;
    ///
    /// let piechart = PieChart::default().pie_char('█');
    /// ```
    #[must_use]
    pub const fn pie_char(mut self, c: char) -> Self {
        self.pie_char = c;
        self
    }

    /// Sets the marker used for legend items.
    ///
    /// You can use any string (including Unicode characters) for custom markers.
    ///
    /// # Examples
    ///
    /// Using a predefined symbol:
    ///
    /// ```
    /// use tui_piechart::{PieChart, symbols};
    ///
    /// let piechart = PieChart::default()
    ///     .legend_marker(symbols::LEGEND_MARKER_CIRCLE);
    /// ```
    ///
    /// Using custom markers:
    ///
    /// ```
    /// use tui_piechart::PieChart;
    ///
    /// // Simple arrow
    /// let piechart = PieChart::default().legend_marker("→");
    ///
    /// // Or any Unicode character
    /// let piechart = PieChart::default().legend_marker("★");
    ///
    /// // Or even multi-character strings
    /// let piechart = PieChart::default().legend_marker("-->");
    /// ```
    #[must_use]
    pub const fn legend_marker(mut self, marker: &'a str) -> Self {
        self.legend_marker = marker;
        self
    }

    /// Calculates the total value of all slices.
    fn total_value(&self) -> f64 {
        self.slices.iter().map(|s| s.value).sum()
    }

    /// Calculates the percentage for a given slice.
    fn percentage(&self, slice: &PieSlice) -> f64 {
        let total = self.total_value();
        if total > 0.0 {
            (slice.value / total) * 100.0
        } else {
            0.0
        }
    }
}

impl Styled for PieChart<'_> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(mut self, style: S) -> Self::Item {
        self.style = style.into();
        self
    }
}

impl Widget for PieChart<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self, area, buf);
    }
}

impl Widget for &PieChart<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let inner = if let Some(ref block) = self.block {
            let inner_area = block.inner(area);
            block.render(area, buf);
            inner_area
        } else {
            area
        };
        self.render_piechart(inner, buf);
    }
}

impl PieChart<'_> {
    fn render_piechart(&self, area: Rect, buf: &mut Buffer) {
        if area.is_empty() || self.slices.is_empty() {
            return;
        }

        let total = self.total_value();
        if total <= 0.0 {
            return;
        }

        // If we need to show legend, reserve space on the right
        let (pie_area, legend_x) = if self.show_legend && area.width > 35 {
            let legend_width = 20;
            let pie_width = area.width.saturating_sub(legend_width);
            (
                Rect {
                    x: area.x,
                    y: area.y,
                    width: pie_width,
                    height: area.height,
                },
                area.x + pie_width + 1, // Add 1 space padding
            )
        } else {
            (area, 0)
        };

        // Calculate the center and radius of the pie chart
        // Account for terminal character aspect ratio (typically 1:2, chars are twice as tall as wide)
        let center_x = pie_area.width / 2;
        let center_y = pie_area.height / 2;

        // Adjust radius for aspect ratio - use width as limiting factor
        let radius = center_x.min(center_y * 2).saturating_sub(1);

        // Draw the pie chart
        let mut cumulative_percent = 0.0;
        for slice in &self.slices {
            let percent = self.percentage(slice);
            self.render_slice(
                pie_area,
                buf,
                center_x,
                center_y,
                radius,
                cumulative_percent,
                percent,
                slice.color,
            );
            cumulative_percent += percent;
        }

        // Draw legend if enabled
        if self.show_legend && area.width > 35 {
            self.render_legend(area, buf, legend_x);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_slice(
        &self,
        area: Rect,
        buf: &mut Buffer,
        center_x: u16,
        center_y: u16,
        radius: u16,
        start_percent: f64,
        percent: f64,
        color: Color,
    ) {
        if radius == 0 || percent <= 0.0 {
            return;
        }

        // Start angle at top (90 degrees) and go clockwise
        let start_angle = (start_percent / 100.0) * 2.0 * PI - PI / 2.0;
        let end_angle = ((start_percent + percent) / 100.0) * 2.0 * PI - PI / 2.0;

        // Scan the entire area around the center
        let scan_width = (radius + 1) as i32;
        let scan_height = ((radius / 2) + 1) as i32; // Account for aspect ratio

        for dy in -scan_height..=scan_height {
            for dx in -scan_width..=scan_width {
                // Calculate actual position in buffer
                let x = i32::from(area.x) + i32::from(center_x) + dx;
                let y = i32::from(area.y) + i32::from(center_y) + dy;

                // Check bounds
                if x < i32::from(area.x)
                    || x >= i32::from(area.x + area.width)
                    || y < i32::from(area.y)
                    || y >= i32::from(area.y + area.height)
                {
                    continue;
                }

                // Adjust for aspect ratio: multiply y distance by 2
                #[allow(clippy::cast_precision_loss)]
                let adjusted_dx = dx as f64;
                #[allow(clippy::cast_precision_loss)]
                let adjusted_dy = (dy * 2) as f64;

                // Calculate distance from center
                let distance = (adjusted_dx * adjusted_dx + adjusted_dy * adjusted_dy).sqrt();

                // Check if point is within radius
                #[allow(clippy::cast_precision_loss)]
                if distance <= radius as f64 {
                    // Calculate angle from center (0 = right, PI/2 = up, PI = left, 3PI/2 = down)
                    let angle = adjusted_dy.atan2(adjusted_dx);

                    // Check if angle is within slice
                    if Self::is_angle_in_slice(angle, start_angle, end_angle) {
                        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                        {
                            let cell = &mut buf[(x as u16, y as u16)];
                            cell.set_char(self.pie_char).set_fg(color);
                        }
                    }
                }
            }
        }
    }

    fn is_angle_in_slice(angle: f64, start: f64, end: f64) -> bool {
        // Normalize angles to [0, 2π]
        let normalize = |a: f64| {
            let mut normalized = a % (2.0 * PI);
            if normalized < 0.0 {
                normalized += 2.0 * PI;
            }
            normalized
        };

        let norm_angle = normalize(angle);
        let norm_start = normalize(start);
        let norm_end = normalize(end);

        if norm_start <= norm_end {
            norm_angle >= norm_start && norm_angle <= norm_end
        } else {
            // Handle wrap around at 2π/0
            norm_angle >= norm_start || norm_angle <= norm_end
        }
    }

    fn render_legend(&self, area: Rect, buf: &mut Buffer, legend_x: u16) {
        let total = self.total_value();

        for (y_offset, slice) in self.slices.iter().enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            let y_offset_u16 = y_offset as u16;

            // Add spacing between legend items and start a bit lower
            let actual_y_offset = y_offset_u16 * 2 + 1;

            if actual_y_offset >= area.height {
                break;
            }

            let legend_text = if self.show_percentages {
                let percent = if total > 0.0 {
                    (slice.value / total) * 100.0
                } else {
                    0.0
                };
                format!("{} {} {:.1}%", self.legend_marker, slice.label, percent)
            } else {
                format!("{} {}", self.legend_marker, slice.label)
            };

            let spans = vec![Span::styled(legend_text, Style::default().fg(slice.color))];
            let line = Line::from(spans);

            let legend_area = Rect {
                x: legend_x,
                y: area.y + actual_y_offset,
                width: area.width.saturating_sub(legend_x - area.x),
                height: 1,
            };

            line.render(legend_area, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pie_slice_new() {
        let slice = PieSlice::new("Test", 50.0, Color::Red);
        assert_eq!(slice.label(), "Test");
        assert_eq!(slice.value(), 50.0);
        assert_eq!(slice.color(), Color::Red);
    }

    #[test]
    fn piechart_new() {
        let slices = vec![
            PieSlice::new("A", 30.0, Color::Red),
            PieSlice::new("B", 70.0, Color::Blue),
        ];
        let piechart = PieChart::new(slices.clone());
        assert_eq!(piechart.slices, slices);
    }

    #[test]
    fn piechart_default() {
        let piechart = PieChart::default();
        assert!(piechart.slices.is_empty());
        assert!(piechart.show_legend);
        assert!(piechart.show_percentages);
    }

    #[test]
    fn piechart_slices() {
        let slices = vec![PieSlice::new("Test", 100.0, Color::Green)];
        let piechart = PieChart::default().slices(slices.clone());
        assert_eq!(piechart.slices, slices);
    }

    #[test]
    fn piechart_style() {
        let style = Style::default().fg(Color::Red);
        let piechart = PieChart::default().style(style);
        assert_eq!(piechart.style, style);
    }

    #[test]
    fn piechart_show_legend() {
        let piechart = PieChart::default().show_legend(false);
        assert!(!piechart.show_legend);
    }

    #[test]
    fn piechart_show_percentages() {
        let piechart = PieChart::default().show_percentages(false);
        assert!(!piechart.show_percentages);
    }

    #[test]
    fn piechart_pie_char() {
        let piechart = PieChart::default().pie_char('█');
        assert_eq!(piechart.pie_char, '█');
    }

    #[test]
    fn piechart_total_value() {
        let slices = vec![
            PieSlice::new("A", 30.0, Color::Red),
            PieSlice::new("B", 70.0, Color::Blue),
        ];
        let piechart = PieChart::new(slices);
        assert_eq!(piechart.total_value(), 100.0);
    }

    #[test]
    fn piechart_percentage() {
        let slices = vec![
            PieSlice::new("A", 30.0, Color::Red),
            PieSlice::new("B", 70.0, Color::Blue),
        ];
        let piechart = PieChart::new(slices);
        assert_eq!(
            piechart.percentage(&PieSlice::new("A", 30.0, Color::Red)),
            30.0
        );
    }

    #[test]
    fn piechart_render_empty_area() {
        let piechart = PieChart::default();
        let mut buffer = Buffer::empty(Rect::new(0, 0, 0, 0));
        piechart.render(buffer.area, &mut buffer);
    }

    #[test]
    fn piechart_render_with_block() {
        let slices = vec![PieSlice::new("Test", 100.0, Color::Red)];
        let piechart = PieChart::new(slices).block(Block::bordered());
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 10));
        piechart.render(buffer.area, &mut buffer);
    }

    #[test]
    fn piechart_render_basic() {
        let slices = vec![
            PieSlice::new("Rust", 45.0, Color::Red),
            PieSlice::new("Go", 30.0, Color::Blue),
            PieSlice::new("Python", 25.0, Color::Green),
        ];
        let piechart = PieChart::new(slices);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 40, 20));
        piechart.render(buffer.area, &mut buffer);
    }

    #[test]
    fn piechart_styled_trait() {
        use ratatui::style::Stylize;
        let piechart = PieChart::default().red();
        assert_eq!(piechart.style.fg, Some(Color::Red));
    }

    #[test]
    fn piechart_with_multiple_slices() {
        let slices = vec![
            PieSlice::new("A", 25.0, Color::Red),
            PieSlice::new("B", 25.0, Color::Blue),
            PieSlice::new("C", 25.0, Color::Green),
            PieSlice::new("D", 25.0, Color::Yellow),
        ];
        let piechart = PieChart::new(slices);
        assert_eq!(piechart.total_value(), 100.0);

        let mut buffer = Buffer::empty(Rect::new(0, 0, 50, 30));
        piechart.render(buffer.area, &mut buffer);
    }

    #[test]
    fn piechart_zero_values() {
        let slices = vec![
            PieSlice::new("A", 0.0, Color::Red),
            PieSlice::new("B", 0.0, Color::Blue),
        ];
        let piechart = PieChart::new(slices);
        assert_eq!(piechart.total_value(), 0.0);
    }

    #[test]
    fn piechart_method_chaining() {
        use ratatui::widgets::Block;

        let slices = vec![PieSlice::new("Test", 100.0, Color::Red)];
        let piechart = PieChart::new(slices)
            .show_legend(true)
            .show_percentages(true)
            .pie_char('█')
            .block(Block::bordered().title("Test"))
            .style(Style::default().fg(Color::White));

        assert!(piechart.show_legend);
        assert!(piechart.show_percentages);
        assert_eq!(piechart.pie_char, '█');
        assert!(piechart.block.is_some());
        assert_eq!(piechart.style.fg, Some(Color::White));
    }

    #[test]
    fn piechart_custom_symbols() {
        use crate::symbols;

        let piechart = PieChart::default().pie_char(symbols::PIE_CHAR_BLOCK);
        assert_eq!(piechart.pie_char, '█');

        let piechart = PieChart::default().pie_char(symbols::PIE_CHAR_CIRCLE);
        assert_eq!(piechart.pie_char, '◉');

        let piechart = PieChart::default().pie_char(symbols::PIE_CHAR_SQUARE);
        assert_eq!(piechart.pie_char, '■');
    }

    #[test]
    fn piechart_is_angle_in_slice() {
        use std::f64::consts::PI;

        // Test angle in range
        assert!(PieChart::is_angle_in_slice(PI / 4.0, 0.0, PI / 2.0));

        // Test angle outside range
        assert!(!PieChart::is_angle_in_slice(PI, 0.0, PI / 2.0));

        // Test wrap around
        assert!(PieChart::is_angle_in_slice(0.1, 1.5 * PI, 0.5));
    }
}
