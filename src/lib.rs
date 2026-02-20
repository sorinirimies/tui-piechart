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
//!
//! With custom border styles:
//!
//! ```no_run
//! use ratatui::style::Color;
//! use tui_piechart::{PieChart, PieSlice, border_style::BorderStyle};
//! // Or use backwards-compatible path: use tui_piechart::symbols::BorderStyle;
//!
//! let slices = vec![
//!     PieSlice::new("Rust", 45.0, Color::Red),
//!     PieSlice::new("Go", 30.0, Color::Blue),
//! ];
//!
//! // Use predefined border styles
//! let piechart = PieChart::new(slices)
//!     .block(BorderStyle::Rounded.block().title("My Chart"));
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

pub mod border_style;
pub mod legend;
#[macro_use]
pub mod macros;
pub mod symbols;
pub mod title;

// Re-export commonly used types from submodules for convenience
pub use legend::{LegendAlignment, LegendLayout, LegendPosition};
pub use title::{BlockExt, TitleAlignment, TitlePosition, TitleStyle};

/// Rendering resolution mode for pie charts.
///
/// Different resolution modes provide varying levels of detail by using
/// different Unicode block drawing characters with different dot densities.
///
/// # Examples
///
/// ```
/// use tui_piechart::{PieChart, PieSlice, Resolution};
/// use ratatui::style::Color;
///
/// let slices = vec![PieSlice::new("Rust", 45.0, Color::Red)];
///
/// // Standard resolution (1 dot per character)
/// let standard = PieChart::new(slices.clone())
///     .resolution(Resolution::Standard);
///
/// // High resolution with braille patterns (8 dots per character)
/// let braille = PieChart::new(slices)
///     .resolution(Resolution::Braille);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Resolution {
    /// Standard resolution using full characters (1 dot per cell).
    ///
    /// Uses regular Unicode characters like `●`. This is the default mode.
    #[default]
    Standard,

    /// Braille resolution using 2×4 dot patterns (8 dots per cell).
    ///
    /// Uses Unicode braille patterns (U+2800-U+28FF) providing 8x resolution.
    /// This provides the highest resolution available for terminal rendering.
    Braille,
}

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
    /// Resolution mode for rendering
    resolution: Resolution,
    /// Position of the legend
    legend_position: LegendPosition,
    /// Layout of the legend
    legend_layout: LegendLayout,
    /// Alignment of legend items
    legend_alignment: LegendAlignment,
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
            resolution: Resolution::default(),
            legend_position: LegendPosition::default(),
            legend_layout: LegendLayout::default(),
            legend_alignment: LegendAlignment::default(),
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

    /// Sets the rendering resolution mode.
    ///
    /// Different resolution modes provide varying levels of detail:
    /// - `Standard`: Regular characters (1 dot per cell)
    /// - `Braille`: 2×4 patterns (8 dots per cell, 8x resolution)
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_piechart::{PieChart, Resolution};
    ///
    /// let standard = PieChart::default().resolution(Resolution::Standard);
    /// let braille = PieChart::default().resolution(Resolution::Braille);
    /// ```
    #[must_use]
    pub const fn resolution(mut self, resolution: Resolution) -> Self {
        self.resolution = resolution;
        self
    }

    /// Sets whether to use high resolution rendering with braille patterns.
    ///
    /// This is a convenience method that sets the resolution to `Braille` when enabled,
    /// or `Standard` when disabled. For more control, use [`resolution`](Self::resolution).
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_piechart::PieChart;
    ///
    /// let piechart = PieChart::default().high_resolution(true);
    /// ```
    #[must_use]
    pub const fn high_resolution(mut self, enabled: bool) -> Self {
        self.resolution = if enabled {
            Resolution::Braille
        } else {
            Resolution::Standard
        };
        self
    }

    /// Sets the position of the legend relative to the pie chart.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_piechart::{PieChart, LegendPosition};
    ///
    /// let piechart = PieChart::default()
    ///     .legend_position(LegendPosition::Right);
    /// ```
    #[must_use]
    pub const fn legend_position(mut self, position: LegendPosition) -> Self {
        self.legend_position = position;
        self
    }

    /// Sets the layout mode for the legend.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_piechart::{PieChart, LegendLayout};
    ///
    /// // Single horizontal row
    /// let piechart = PieChart::default()
    ///     .legend_layout(LegendLayout::Horizontal);
    ///
    /// // Vertical stacking (default)
    /// let piechart = PieChart::default()
    ///     .legend_layout(LegendLayout::Vertical);
    /// ```
    #[must_use]
    pub const fn legend_layout(mut self, layout: LegendLayout) -> Self {
        self.legend_layout = layout;
        self
    }

    /// Sets the alignment of legend items within the legend area.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_piechart::{PieChart, LegendAlignment};
    ///
    /// // Center-align legend items
    /// let piechart = PieChart::default()
    ///     .legend_alignment(LegendAlignment::Center);
    ///
    /// // Right-align legend items
    /// let piechart = PieChart::default()
    ///     .legend_alignment(LegendAlignment::Right);
    /// ```
    #[must_use]
    pub const fn legend_alignment(mut self, alignment: LegendAlignment) -> Self {
        self.legend_alignment = alignment;
        self
    }

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
    /// Maximum ratio for vertical legend width (1/3 of available width).
    const LEGEND_VERTICAL_MAX_RATIO: u16 = 3;

    /// Minimum width for vertical legend to ensure readability.
    const LEGEND_VERTICAL_MIN_WIDTH: u16 = 20;

    /// Maximum ratio for horizontal legend width (2/5 = 40% of available width).
    /// This keeps the pie chart proportional and prevents legend from dominating.
    const LEGEND_HORIZONTAL_MAX_RATIO: u16 = 5;

    /// Absolute maximum width for horizontal legends to prevent excessive space usage.
    const LEGEND_HORIZONTAL_MAX_WIDTH: u16 = 60;

    /// Absolute maximum height for vertical legends to prevent pie chart from being too small.
    /// This allows 4 items with spacing (4 items * 2 lines = 8 lines, +1 for padding = 9).
    const LEGEND_VERTICAL_MAX_HEIGHT: u16 = 9;

    /// Height required for horizontal legend layout (single row with padding).
    const LEGEND_HORIZONTAL_HEIGHT: u16 = 3;

    /// Space between pie chart and legend areas.
    const LEGEND_SPACING: u16 = 1;

    /// Inner padding for legend area.
    const LEGEND_PADDING: u16 = 1;

    fn render_piechart(&self, area: Rect, buf: &mut Buffer) {
        if area.is_empty() || self.slices.is_empty() {
            return;
        }

        let total = self.total_value();
        if total <= 0.0 {
            return;
        }

        match self.resolution {
            Resolution::Standard => {
                // Continue with standard rendering below
            }
            Resolution::Braille => {
                self.render_piechart_braille(area, buf);
                return;
            }
        }

        // Calculate layout with legend positioning
        let (pie_area, legend_area_opt) = self.calculate_layout(area);

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
        if let Some(legend_area) = legend_area_opt {
            self.render_legend(buf, legend_area);
        }
    }

    #[allow(clippy::too_many_arguments, clippy::similar_names)]
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
        let scan_width = i32::from(radius + 1);
        let scan_height = i32::from((radius / 2) + 1); // Account for aspect ratio

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
                let adjusted_dx = f64::from(dx);
                #[allow(clippy::cast_precision_loss)]
                let adjusted_dy = f64::from(dy * 2);

                // Calculate distance from center
                let distance = (adjusted_dx * adjusted_dx + adjusted_dy * adjusted_dy).sqrt();

                // Check if point is within radius
                #[allow(clippy::cast_precision_loss)]
                if distance <= f64::from(radius) {
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

    fn format_legend_text(&self, slice: &PieSlice, total: f64, spacing: &str) -> String {
        if self.show_percentages {
            let percent = if total > 0.0 {
                (slice.value / total) * 100.0
            } else {
                0.0
            };
            format!(
                "{} {} {:.1}%{}",
                self.legend_marker, slice.label, percent, spacing
            )
        } else {
            format!("{} {}{}", self.legend_marker, slice.label, spacing)
        }
    }

    fn calculate_aligned_x(&self, legend_area: Rect, content_width: u16) -> u16 {
        match self.legend_alignment {
            LegendAlignment::Left => legend_area.x,
            LegendAlignment::Center => {
                legend_area.x + (legend_area.width.saturating_sub(content_width)) / 2
            }
            LegendAlignment::Right => {
                legend_area.x + legend_area.width.saturating_sub(content_width)
            }
        }
    }

    fn render_legend(&self, buf: &mut Buffer, legend_area: Rect) {
        let total = self.total_value();

        match self.legend_layout {
            LegendLayout::Vertical => {
                self.render_vertical_legend(buf, legend_area, total);
            }
            LegendLayout::Horizontal => {
                self.render_horizontal_legend(buf, legend_area, total);
            }
        }
    }

    fn render_vertical_legend(&self, buf: &mut Buffer, legend_area: Rect, total: f64) {
        for (idx, slice) in self.slices.iter().enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            let y_offset = (idx as u16) * 2;

            if y_offset >= legend_area.height {
                break;
            }

            let legend_text = self.format_legend_text(slice, total, "");
            #[allow(clippy::cast_possible_truncation)]
            let text_width = legend_text.len() as u16;
            let x_pos = self.calculate_aligned_x(legend_area, text_width);

            let line = Line::from(vec![Span::styled(
                legend_text,
                Style::default().fg(slice.color),
            )]);
            let item_area = Rect {
                x: x_pos,
                y: legend_area.y + y_offset,
                width: text_width.min(legend_area.width),
                height: 1,
            };

            line.render(item_area, buf);
        }
    }

    fn render_horizontal_legend(&self, buf: &mut Buffer, legend_area: Rect, total: f64) {
        let mut total_width = 0u16;
        let mut item_widths = Vec::new();

        for slice in &self.slices {
            let legend_text = self.format_legend_text(slice, total, "  ");
            #[allow(clippy::cast_possible_truncation)]
            let text_width = legend_text.len() as u16;
            item_widths.push(text_width);
            total_width = total_width.saturating_add(text_width);
        }

        let start_x = self.calculate_aligned_x(legend_area, total_width.min(legend_area.width));
        let mut x_offset = 0u16;

        for (idx, slice) in self.slices.iter().enumerate() {
            if x_offset >= legend_area.width {
                break;
            }

            let legend_text = self.format_legend_text(slice, total, "  ");
            let text_width = item_widths[idx];

            let line = Line::from(vec![Span::styled(
                legend_text,
                Style::default().fg(slice.color),
            )]);
            let item_area = Rect {
                x: start_x + x_offset,
                y: legend_area.y,
                width: text_width.min(legend_area.width.saturating_sub(x_offset)),
                height: 1,
            };

            line.render(item_area, buf);
            x_offset = x_offset.saturating_add(text_width);
        }
    }

    #[allow(clippy::too_many_lines)]
    fn calculate_layout(&self, area: Rect) -> (Rect, Option<Rect>) {
        if !self.show_legend || area.width < 20 || area.height < 10 {
            return (area, None);
        }

        // Vertical layout uses Left/Right positions, Horizontal layout uses Top/Bottom
        match (self.legend_position, self.legend_layout) {
            // Left/Right with Vertical layout - proper vertical stacking on sides
            (LegendPosition::Left | LegendPosition::Right, LegendLayout::Vertical) => {
                let legend_width = self
                    .calculate_legend_width()
                    .min(area.width / Self::LEGEND_VERTICAL_MAX_RATIO)
                    .max(Self::LEGEND_VERTICAL_MIN_WIDTH);
                let is_left = matches!(self.legend_position, LegendPosition::Left);
                Self::layout_horizontal_split(area, legend_width, is_left)
            }
            // Top/Bottom with Horizontal layout - single row at top/bottom
            (LegendPosition::Top | LegendPosition::Bottom, LegendLayout::Horizontal) => {
                let is_top = matches!(self.legend_position, LegendPosition::Top);
                Self::layout_vertical_split(area, Self::LEGEND_HORIZONTAL_HEIGHT, is_top)
            }
            // Fallback: use horizontal layout for incompatible combinations
            (LegendPosition::Left | LegendPosition::Right, LegendLayout::Horizontal) => {
                // Horizontal layout on sides - allocate limited width
                let legend_width = self
                    .calculate_legend_horizontal_width()
                    .min(
                        (area.width * (Self::LEGEND_HORIZONTAL_MAX_RATIO - 1))
                            / Self::LEGEND_HORIZONTAL_MAX_RATIO,
                    )
                    .min(Self::LEGEND_HORIZONTAL_MAX_WIDTH);
                let is_left = matches!(self.legend_position, LegendPosition::Left);
                Self::layout_horizontal_split(area, legend_width, is_left)
            }
            (LegendPosition::Top | LegendPosition::Bottom, LegendLayout::Vertical) => {
                // Vertical layout at top/bottom - use 2-column grid with minimal height
                let legend_height = self.calculate_vertical_grid_height(area.width);
                let is_top = matches!(self.legend_position, LegendPosition::Top);
                Self::layout_vertical_split(area, legend_height, is_top)
            }
        }
    }

    fn calculate_vertical_grid_height(&self, available_width: u16) -> u16 {
        // For vertical layout at top/bottom, use 2-column grid
        let max_item_width = self.calculate_legend_width();
        let columns = (available_width.saturating_sub(Self::LEGEND_PADDING * 2)
            / max_item_width.max(1))
        .clamp(1, 2);

        #[allow(clippy::cast_possible_truncation)]
        let num_items = self.slices.len() as u16;

        // Calculate rows: ceil(items / columns)
        let rows = num_items.div_ceil(columns);
        // Each row needs 2 lines (item + spacing), plus account for padding that will be subtracted
        (rows * 2 + Self::LEGEND_PADDING).clamp(4, Self::LEGEND_VERTICAL_MAX_HEIGHT)
    }

    fn layout_horizontal_split(
        area: Rect,
        legend_width: u16,
        legend_on_left: bool,
    ) -> (Rect, Option<Rect>) {
        if area.width <= legend_width {
            return (area, None);
        }

        let pie_width = area
            .width
            .saturating_sub(legend_width + Self::LEGEND_SPACING);

        if legend_on_left {
            (
                Rect {
                    x: area.x + legend_width + Self::LEGEND_SPACING,
                    y: area.y,
                    width: pie_width,
                    height: area.height,
                },
                Some(Rect {
                    x: area.x,
                    y: area.y + Self::LEGEND_PADDING,
                    width: legend_width,
                    height: area.height.saturating_sub(Self::LEGEND_PADDING * 2),
                }),
            )
        } else {
            (
                Rect {
                    x: area.x,
                    y: area.y,
                    width: pie_width,
                    height: area.height,
                },
                Some(Rect {
                    x: area.x + pie_width + Self::LEGEND_SPACING,
                    y: area.y + Self::LEGEND_PADDING,
                    width: legend_width,
                    height: area.height.saturating_sub(Self::LEGEND_PADDING * 2),
                }),
            )
        }
    }

    fn layout_vertical_split(
        area: Rect,
        legend_height: u16,
        legend_on_top: bool,
    ) -> (Rect, Option<Rect>) {
        if area.height <= legend_height {
            return (area, None);
        }

        let pie_height = area
            .height
            .saturating_sub(legend_height + Self::LEGEND_SPACING);

        if legend_on_top {
            (
                Rect {
                    x: area.x,
                    y: area.y + legend_height + Self::LEGEND_SPACING,
                    width: area.width,
                    height: pie_height,
                },
                Some(Rect {
                    x: area.x + Self::LEGEND_PADDING,
                    y: area.y + Self::LEGEND_PADDING,
                    width: area.width.saturating_sub(Self::LEGEND_PADDING * 2),
                    height: legend_height.saturating_sub(Self::LEGEND_PADDING),
                }),
            )
        } else {
            (
                Rect {
                    x: area.x,
                    y: area.y,
                    width: area.width,
                    height: pie_height,
                },
                Some(Rect {
                    x: area.x + Self::LEGEND_PADDING,
                    y: area.y + pie_height + Self::LEGEND_SPACING,
                    width: area.width.saturating_sub(Self::LEGEND_PADDING * 2),
                    height: legend_height.saturating_sub(Self::LEGEND_PADDING),
                }),
            )
        }
    }

    fn calculate_legend_width(&self) -> u16 {
        let total = self.total_value();

        match self.legend_layout {
            LegendLayout::Vertical => {
                // For vertical layout, find the maximum width of a single item
                let mut max_width = 0u16;

                for slice in &self.slices {
                    let text = if self.show_percentages {
                        let percent = if total > 0.0 {
                            (slice.value / total) * 100.0
                        } else {
                            0.0
                        };
                        format!("{} {} {:.1}%  ", self.legend_marker, slice.label, percent)
                    } else {
                        format!("{} {}  ", self.legend_marker, slice.label)
                    };

                    #[allow(clippy::cast_possible_truncation)]
                    let text_width = text.len() as u16;
                    max_width = max_width.max(text_width);
                }

                max_width.saturating_add(2)
            }
            LegendLayout::Horizontal => {
                // For horizontal layout, sum the width of all items
                let mut total_width = 0u16;

                for slice in &self.slices {
                    let text = if self.show_percentages {
                        let percent = if total > 0.0 {
                            (slice.value / total) * 100.0
                        } else {
                            0.0
                        };
                        format!("{} {} {:.1}%  ", self.legend_marker, slice.label, percent)
                    } else {
                        format!("{} {}  ", self.legend_marker, slice.label)
                    };

                    #[allow(clippy::cast_possible_truncation)]
                    let text_width = text.len() as u16;
                    total_width = total_width.saturating_add(text_width);
                }

                total_width.saturating_add(2)
            }
        }
    }

    fn calculate_legend_horizontal_width(&self) -> u16 {
        let total = self.total_value();
        let mut total_width = 0u16;

        for slice in &self.slices {
            let text = if self.show_percentages {
                let percent = if total > 0.0 {
                    (slice.value / total) * 100.0
                } else {
                    0.0
                };
                format!("{} {} {:.1}%  ", self.legend_marker, slice.label, percent)
            } else {
                format!("{} {}  ", self.legend_marker, slice.label)
            };

            #[allow(clippy::cast_possible_truncation)]
            let text_width = text.len() as u16;
            total_width = total_width.saturating_add(text_width);
        }

        total_width.saturating_add(2)
    }

    #[allow(clippy::similar_names)]
    fn render_piechart_braille(&self, area: Rect, buf: &mut Buffer) {
        // Calculate layout with legend positioning
        let (pie_area, legend_area_opt) = self.calculate_layout(area);

        // Calculate the center and radius of the pie chart
        let center_x_chars = pie_area.width / 2;
        let center_y_chars = pie_area.height / 2;

        // Each character cell has 2x4 braille dots
        let center_x_dots = center_x_chars * 2;
        let center_y_dots = center_y_chars * 4;

        // Calculate radius in dots
        // Braille dots are equally spaced in physical screen space because:
        // - Character cells are ~2:1 (height:width)
        // - But braille has 2 horizontal dots and 4 vertical dots per character
        // - So: horizontal spacing = W/2, vertical spacing = 2W/4 = W/2 (equal!)
        let radius = (center_x_dots).min(center_y_dots).saturating_sub(2);

        // Create a 2D array to store which slice each braille dot belongs to
        let width_dots = pie_area.width * 2;
        let height_dots = pie_area.height * 4;

        let mut dot_slices: Vec<Vec<Option<usize>>> =
            vec![vec![None; width_dots as usize]; height_dots as usize];

        // Calculate slice assignments for each dot
        let mut cumulative_percent = 0.0;
        for (slice_idx, slice) in self.slices.iter().enumerate() {
            let percent = self.percentage(slice);
            let start_angle = (cumulative_percent / 100.0) * 2.0 * PI - PI / 2.0;
            let end_angle = ((cumulative_percent + percent) / 100.0) * 2.0 * PI - PI / 2.0;

            for dy in 0..height_dots {
                for dx in 0..width_dots {
                    let rel_x = f64::from(dx) - f64::from(center_x_dots);
                    let rel_y = f64::from(dy) - f64::from(center_y_dots);

                    // No aspect ratio compensation needed for braille dots
                    // They're already equally spaced in physical screen space
                    let distance = (rel_x * rel_x + rel_y * rel_y).sqrt();

                    if distance <= f64::from(radius) {
                        let angle = rel_y.atan2(rel_x);
                        if Self::is_angle_in_slice(angle, start_angle, end_angle) {
                            dot_slices[dy as usize][dx as usize] = Some(slice_idx);
                        }
                    }
                }
            }

            cumulative_percent += percent;
        }

        // Convert dot assignments to braille characters
        for char_y in 0..pie_area.height {
            for char_x in 0..pie_area.width {
                let base_dot_x = char_x * 2;
                let base_dot_y = char_y * 4;

                // Braille pattern mapping (dots are numbered 1-8)
                // Dot positions in a 2x4 grid:
                // 1 4
                // 2 5
                // 3 6
                // 7 8
                let dot_positions = [
                    (0, 0, 0x01), // dot 1
                    (0, 1, 0x02), // dot 2
                    (0, 2, 0x04), // dot 3
                    (1, 0, 0x08), // dot 4
                    (1, 1, 0x10), // dot 5
                    (1, 2, 0x20), // dot 6
                    (0, 3, 0x40), // dot 7
                    (1, 3, 0x80), // dot 8
                ];

                let mut pattern = 0u32;
                let mut slice_colors: Vec<(usize, u32)> = Vec::new();

                for (dx, dy, bit) in dot_positions {
                    let dot_x = base_dot_x + dx;
                    let dot_y = base_dot_y + dy;

                    if dot_y < height_dots && dot_x < width_dots {
                        if let Some(slice_idx) = dot_slices[dot_y as usize][dot_x as usize] {
                            pattern |= bit;
                            // Track which slice and how many dots
                            if let Some(entry) =
                                slice_colors.iter_mut().find(|(idx, _)| *idx == slice_idx)
                            {
                                entry.1 += 1;
                            } else {
                                slice_colors.push((slice_idx, 1));
                            }
                        }
                    }
                }

                if pattern > 0 {
                    // Use the color of the slice with the most dots in this character
                    if let Some((slice_idx, _)) = slice_colors.iter().max_by_key(|(_, count)| count)
                    {
                        let braille_char = char::from_u32(0x2800 + pattern).unwrap_or('⠀');
                        let color = self.slices[*slice_idx].color;

                        let cell = &mut buf[(pie_area.x + char_x, pie_area.y + char_y)];
                        cell.set_char(braille_char).set_fg(color);
                    }
                }
            }
        }

        // Draw legend if enabled
        if let Some(legend_area) = legend_area_opt {
            self.render_legend(buf, legend_area);
        }
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
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

    // Render tests - using macros for common patterns
    render_empty_test!(piechart_render_empty_area, PieChart::default());

    render_with_size_test!(
        piechart_render_with_block,
        {
            let slices = vec![PieSlice::new("Test", 100.0, Color::Red)];
            PieChart::new(slices).block(Block::bordered())
        },
        width: 20,
        height: 10
    );

    render_test!(
        piechart_render_basic,
        {
            let slices = vec![
                PieSlice::new("Rust", 45.0, Color::Red),
                PieSlice::new("Go", 30.0, Color::Blue),
                PieSlice::new("Python", 25.0, Color::Green),
            ];
            PieChart::new(slices)
        },
        Rect::new(0, 0, 40, 20)
    );

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
    }

    // Using render macro for the visual test
    render_with_size_test!(
        piechart_multi_slice_render,
        {
            let slices = vec![
                PieSlice::new("A", 25.0, Color::Red),
                PieSlice::new("B", 25.0, Color::Blue),
                PieSlice::new("C", 25.0, Color::Green),
                PieSlice::new("D", 25.0, Color::Yellow),
            ];
            PieChart::new(slices)
        },
        width: 50,
        height: 30
    );

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

    // --- Resolution ---

    #[test]
    fn piechart_resolution_standard() {
        let piechart = PieChart::default().resolution(Resolution::Standard);
        assert!(matches!(piechart.resolution, Resolution::Standard));
    }

    #[test]
    fn piechart_resolution_braille() {
        let piechart = PieChart::default().resolution(Resolution::Braille);
        assert!(matches!(piechart.resolution, Resolution::Braille));
    }

    #[test]
    fn piechart_high_resolution_true() {
        let piechart = PieChart::default().high_resolution(true);
        assert!(matches!(piechart.resolution, Resolution::Braille));
    }

    #[test]
    fn piechart_high_resolution_false() {
        let piechart = PieChart::default().high_resolution(false);
        assert!(matches!(piechart.resolution, Resolution::Standard));
    }

    // --- Legend position / layout / alignment setters ---

    #[test]
    fn piechart_legend_position_left() {
        let piechart = PieChart::default().legend_position(LegendPosition::Left);
        assert!(matches!(piechart.legend_position, LegendPosition::Left));
    }

    #[test]
    fn piechart_legend_position_right() {
        let piechart = PieChart::default().legend_position(LegendPosition::Right);
        assert!(matches!(piechart.legend_position, LegendPosition::Right));
    }

    #[test]
    fn piechart_legend_position_top() {
        let piechart = PieChart::default().legend_position(LegendPosition::Top);
        assert!(matches!(piechart.legend_position, LegendPosition::Top));
    }

    #[test]
    fn piechart_legend_position_bottom() {
        let piechart = PieChart::default().legend_position(LegendPosition::Bottom);
        assert!(matches!(piechart.legend_position, LegendPosition::Bottom));
    }

    #[test]
    fn piechart_legend_layout_horizontal() {
        let piechart = PieChart::default().legend_layout(LegendLayout::Horizontal);
        assert!(matches!(piechart.legend_layout, LegendLayout::Horizontal));
    }

    #[test]
    fn piechart_legend_layout_vertical() {
        let piechart = PieChart::default().legend_layout(LegendLayout::Vertical);
        assert!(matches!(piechart.legend_layout, LegendLayout::Vertical));
    }

    #[test]
    fn piechart_legend_alignment_left() {
        let piechart = PieChart::default().legend_alignment(LegendAlignment::Left);
        assert!(matches!(piechart.legend_alignment, LegendAlignment::Left));
    }

    #[test]
    fn piechart_legend_alignment_center() {
        let piechart = PieChart::default().legend_alignment(LegendAlignment::Center);
        assert!(matches!(piechart.legend_alignment, LegendAlignment::Center));
    }

    #[test]
    fn piechart_legend_alignment_right() {
        let piechart = PieChart::default().legend_alignment(LegendAlignment::Right);
        assert!(matches!(piechart.legend_alignment, LegendAlignment::Right));
    }

    // --- legend_marker setter ---

    #[test]
    fn piechart_legend_marker_custom() {
        use crate::symbols::LEGEND_MARKER;
        let piechart = PieChart::default().legend_marker(LEGEND_MARKER);
        assert_eq!(piechart.legend_marker, LEGEND_MARKER);
    }

    // --- format_legend_text ---

    #[test]
    fn piechart_format_legend_text_with_percentage() {
        let slices = vec![
            PieSlice::new("Rust", 50.0, Color::Red),
            PieSlice::new("Go", 50.0, Color::Blue),
        ];
        let piechart = PieChart::new(slices.clone()).show_percentages(true);
        let text = piechart.format_legend_text(&slices[0], 100.0, "");
        assert!(text.contains("Rust"));
        assert!(text.contains("50.0%"));
    }

    #[test]
    fn piechart_format_legend_text_without_percentage() {
        let slices = vec![PieSlice::new("Rust", 50.0, Color::Red)];
        let piechart = PieChart::new(slices.clone()).show_percentages(false);
        let text = piechart.format_legend_text(&slices[0], 100.0, "");
        assert!(text.contains("Rust"));
        assert!(!text.contains('%'));
    }

    #[test]
    fn piechart_format_legend_text_zero_total() {
        let slices = vec![PieSlice::new("X", 0.0, Color::Red)];
        let piechart = PieChart::new(slices.clone()).show_percentages(true);
        let text = piechart.format_legend_text(&slices[0], 0.0, "");
        assert!(text.contains("0.0%"));
    }

    // --- calculate_aligned_x ---

    #[test]
    fn piechart_calculate_aligned_x_left() {
        let slices = vec![PieSlice::new("A", 100.0, Color::Red)];
        let piechart = PieChart::new(slices).legend_alignment(LegendAlignment::Left);
        let area = Rect::new(5, 0, 20, 10);
        assert_eq!(piechart.calculate_aligned_x(area, 10), 5);
    }

    #[test]
    fn piechart_calculate_aligned_x_center() {
        let slices = vec![PieSlice::new("A", 100.0, Color::Red)];
        let piechart = PieChart::new(slices).legend_alignment(LegendAlignment::Center);
        let area = Rect::new(0, 0, 20, 10);
        // (20 - 10) / 2 = 5
        assert_eq!(piechart.calculate_aligned_x(area, 10), 5);
    }

    #[test]
    fn piechart_calculate_aligned_x_right() {
        let slices = vec![PieSlice::new("A", 100.0, Color::Red)];
        let piechart = PieChart::new(slices).legend_alignment(LegendAlignment::Right);
        let area = Rect::new(0, 0, 20, 10);
        // 20 - 10 = 10
        assert_eq!(piechart.calculate_aligned_x(area, 10), 10);
    }

    // --- Braille render ---

    render_test!(
        piechart_render_braille,
        {
            let slices = vec![
                PieSlice::new("Rust", 60.0, Color::Red),
                PieSlice::new("Go", 40.0, Color::Blue),
            ];
            PieChart::new(slices).resolution(Resolution::Braille)
        },
        Rect::new(0, 0, 40, 20)
    );

    render_test!(
        piechart_render_braille_with_legend,
        {
            let slices = vec![
                PieSlice::new("Rust", 60.0, Color::Red),
                PieSlice::new("Go", 40.0, Color::Blue),
            ];
            PieChart::new(slices)
                .resolution(Resolution::Braille)
                .show_legend(true)
        },
        Rect::new(0, 0, 40, 20)
    );

    // --- Legend layout render paths ---

    render_test!(
        piechart_render_legend_left,
        {
            let slices = vec![
                PieSlice::new("Alpha", 50.0, Color::Red),
                PieSlice::new("Beta", 50.0, Color::Blue),
            ];
            PieChart::new(slices)
                .show_legend(true)
                .legend_position(LegendPosition::Left)
                .legend_layout(LegendLayout::Vertical)
        },
        Rect::new(0, 0, 60, 20)
    );

    render_test!(
        piechart_render_legend_right,
        {
            let slices = vec![
                PieSlice::new("Alpha", 50.0, Color::Red),
                PieSlice::new("Beta", 50.0, Color::Blue),
            ];
            PieChart::new(slices)
                .show_legend(true)
                .legend_position(LegendPosition::Right)
                .legend_layout(LegendLayout::Vertical)
        },
        Rect::new(0, 0, 60, 20)
    );

    render_test!(
        piechart_render_legend_top_horizontal,
        {
            let slices = vec![
                PieSlice::new("Alpha", 50.0, Color::Red),
                PieSlice::new("Beta", 50.0, Color::Blue),
            ];
            PieChart::new(slices)
                .show_legend(true)
                .legend_position(LegendPosition::Top)
                .legend_layout(LegendLayout::Horizontal)
        },
        Rect::new(0, 0, 60, 20)
    );

    render_test!(
        piechart_render_legend_bottom_horizontal,
        {
            let slices = vec![
                PieSlice::new("Alpha", 50.0, Color::Red),
                PieSlice::new("Beta", 50.0, Color::Blue),
            ];
            PieChart::new(slices)
                .show_legend(true)
                .legend_position(LegendPosition::Bottom)
                .legend_layout(LegendLayout::Horizontal)
        },
        Rect::new(0, 0, 60, 20)
    );

    render_test!(
        piechart_render_legend_top_vertical,
        {
            let slices = vec![
                PieSlice::new("Alpha", 50.0, Color::Red),
                PieSlice::new("Beta", 50.0, Color::Blue),
            ];
            PieChart::new(slices)
                .show_legend(true)
                .legend_position(LegendPosition::Top)
                .legend_layout(LegendLayout::Vertical)
        },
        Rect::new(0, 0, 60, 20)
    );

    render_test!(
        piechart_render_legend_bottom_vertical,
        {
            let slices = vec![
                PieSlice::new("Alpha", 50.0, Color::Red),
                PieSlice::new("Beta", 50.0, Color::Blue),
            ];
            PieChart::new(slices)
                .show_legend(true)
                .legend_position(LegendPosition::Bottom)
                .legend_layout(LegendLayout::Vertical)
        },
        Rect::new(0, 0, 60, 20)
    );

    render_test!(
        piechart_render_legend_left_horizontal,
        {
            let slices = vec![
                PieSlice::new("Alpha", 50.0, Color::Red),
                PieSlice::new("Beta", 50.0, Color::Blue),
            ];
            PieChart::new(slices)
                .show_legend(true)
                .legend_position(LegendPosition::Left)
                .legend_layout(LegendLayout::Horizontal)
        },
        Rect::new(0, 0, 60, 20)
    );

    render_test!(
        piechart_render_legend_right_horizontal,
        {
            let slices = vec![
                PieSlice::new("Alpha", 50.0, Color::Red),
                PieSlice::new("Beta", 50.0, Color::Blue),
            ];
            PieChart::new(slices)
                .show_legend(true)
                .legend_position(LegendPosition::Right)
                .legend_layout(LegendLayout::Horizontal)
        },
        Rect::new(0, 0, 60, 20)
    );

    // --- Legend with percentages ---

    render_test!(
        piechart_render_legend_with_percentages,
        {
            let slices = vec![
                PieSlice::new("Rust", 45.0, Color::Red),
                PieSlice::new("Go", 30.0, Color::Blue),
                PieSlice::new("Python", 25.0, Color::Green),
            ];
            PieChart::new(slices)
                .show_legend(true)
                .show_percentages(true)
        },
        Rect::new(0, 0, 60, 20)
    );

    // Legend alignment render paths
    render_test!(
        piechart_render_legend_alignment_center,
        {
            let slices = vec![
                PieSlice::new("A", 50.0, Color::Red),
                PieSlice::new("B", 50.0, Color::Blue),
            ];
            PieChart::new(slices)
                .show_legend(true)
                .legend_alignment(LegendAlignment::Center)
        },
        Rect::new(0, 0, 60, 20)
    );

    render_test!(
        piechart_render_legend_alignment_right,
        {
            let slices = vec![
                PieSlice::new("A", 50.0, Color::Red),
                PieSlice::new("B", 50.0, Color::Blue),
            ];
            PieChart::new(slices)
                .show_legend(true)
                .legend_alignment(LegendAlignment::Right)
        },
        Rect::new(0, 0, 60, 20)
    );

    // --- calculate_layout: small area returns no legend ---

    #[test]
    fn piechart_layout_too_small_no_legend() {
        let slices = vec![PieSlice::new("A", 100.0, Color::Red)];
        let piechart = PieChart::new(slices).show_legend(true);
        // area too narrow (< 20)
        let area = Rect::new(0, 0, 10, 5);
        let (pie_area, legend_opt) = piechart.calculate_layout(area);
        assert_eq!(pie_area, area);
        assert!(legend_opt.is_none());
    }

    #[test]
    fn piechart_layout_show_legend_false_no_legend() {
        let slices = vec![PieSlice::new("A", 100.0, Color::Red)];
        let piechart = PieChart::new(slices).show_legend(false);
        let area = Rect::new(0, 0, 60, 20);
        let (pie_area, legend_opt) = piechart.calculate_layout(area);
        assert_eq!(pie_area, area);
        assert!(legend_opt.is_none());
    }

    // --- Render with no slices (empty) ---

    render_empty_test!(piechart_render_empty_slices, PieChart::default());

    // --- Render single slice ---

    render_test!(
        piechart_render_single_slice,
        PieChart::new(vec![PieSlice::new("Only", 100.0, Color::Cyan)]),
        Rect::new(0, 0, 30, 15)
    );

    // --- Percentage zero total ---

    #[test]
    fn piechart_percentage_zero_total() {
        let slices = vec![PieSlice::new("A", 0.0, Color::Red)];
        let piechart = PieChart::new(slices.clone());
        assert_eq!(piechart.percentage(&slices[0]), 0.0);
    }
}
