//! Legend positioning and layout configuration for pie charts.
//!
//! This module provides types and functionality for controlling where and how
//! the legend is displayed relative to the pie chart.
//!
//! # Examples
//!
//! ```
//! use tui_piechart::{PieChart, PieSlice, LegendPosition, LegendLayout, LegendAlignment};
//! use ratatui::style::Color;
//!
//! let slices = vec![
//!     PieSlice::new("Rust", 45.0, Color::Red),
//!     PieSlice::new("Go", 30.0, Color::Blue),
//!     PieSlice::new("Python", 25.0, Color::Green),
//! ];
//!
//! // Position legend on the left with horizontal layout and center alignment
//! let chart = PieChart::new(slices)
//!     .legend_position(LegendPosition::Left)
//!     .legend_layout(LegendLayout::Horizontal)
//!     .legend_alignment(LegendAlignment::Center);
//! ```

/// Position of the legend relative to the pie chart.
///
/// Controls where the legend appears in relation to the pie chart visualization.
/// The legend can be positioned on any of the four sides: right (default), left,
/// top, or bottom.
///
/// # Examples
///
/// ```
/// use tui_piechart::{PieChart, PieSlice, LegendPosition};
/// use ratatui::style::Color;
///
/// let slices = vec![PieSlice::new("Rust", 45.0, Color::Red)];
///
/// // Position legend on the left side
/// let chart = PieChart::new(slices)
///     .legend_position(LegendPosition::Left);
/// ```
///
/// # Layout Impact
///
/// The legend position affects how space is allocated:
/// - **Right/Left**: Legend takes a portion of horizontal space
/// - **Top/Bottom**: Legend takes a portion of vertical space
///
/// The chart automatically adjusts its size to accommodate the legend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LegendPosition {
    /// Legend on the right side (default)
    ///
    /// The legend appears to the right of the pie chart. This is the default
    /// position and works well for most use cases.
    #[default]
    Right,

    /// Legend on the left side
    ///
    /// The legend appears to the left of the pie chart. Useful when you want
    /// the chart to be more prominent on the right side of the display.
    Left,

    /// Legend at the top
    ///
    /// The legend appears above the pie chart. Best used with horizontal layout
    /// for a more compact display.
    Top,

    /// Legend at the bottom
    ///
    /// The legend appears below the pie chart. Works well with horizontal layout
    /// when vertical space is limited.
    Bottom,
}

/// Layout mode for the legend.
///
/// Controls how legend items are arranged: either stacked vertically in a column
/// (default) or arranged horizontally in a single row.
///
/// # Examples
///
/// ```
/// use tui_piechart::{PieChart, PieSlice, LegendLayout, LegendPosition};
/// use ratatui::style::Color;
///
/// let slices = vec![
///     PieSlice::new("Rust", 45.0, Color::Red),
///     PieSlice::new("Go", 30.0, Color::Blue),
/// ];
///
/// // Use horizontal layout with legend at top
/// let chart = PieChart::new(slices)
///     .legend_position(LegendPosition::Top)
///     .legend_layout(LegendLayout::Horizontal);
/// ```
///
/// # Layout Considerations
///
/// - **Vertical**: Each legend item takes one line. Best for detailed legends
///   with longer labels or when vertical space is available.
/// - **Horizontal**: All legend items on one line. Best for compact displays
///   or when used with Top/Bottom positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LegendLayout {
    /// Vertical layout - items stacked vertically (default)
    ///
    /// Each legend item appears on its own line, stacked vertically:
    /// ```text
    /// ● Item 1  45%
    /// ● Item 2  30%
    /// ● Item 3  25%
    /// ```
    ///
    /// This is the default layout and provides clear separation between items.
    #[default]
    Vertical,

    /// Horizontal layout - items in a single row
    ///
    /// All legend items appear in a single horizontal row:
    /// ```text
    /// ● Item 1  45%  ● Item 2  30%  ● Item 3  25%
    /// ```
    ///
    /// This layout is more compact and works well with Top/Bottom positions.
    /// The chart automatically calculates required width to prevent item cutoff.
    Horizontal,
}

/// Alignment of legend items within the legend area.
///
/// Controls how legend items are aligned horizontally within their allocated space.
/// This is particularly useful in grid layouts or when the legend area is wider
/// than the legend content.
///
/// # Examples
///
/// ```
/// use tui_piechart::{PieChart, PieSlice, LegendAlignment};
/// use ratatui::style::Color;
///
/// let slices = vec![
///     PieSlice::new("Rust", 45.0, Color::Red),
///     PieSlice::new("Go", 30.0, Color::Blue),
/// ];
///
/// // Center-align legend items
/// let chart = PieChart::new(slices)
///     .legend_alignment(LegendAlignment::Center);
/// ```
///
/// # Layout Considerations
///
/// - **Left**: Legend items start from the left edge (default)
/// - **Center**: Legend items are centered within the legend area
/// - **Right**: Legend items align to the right edge
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LegendAlignment {
    /// Left alignment (default)
    ///
    /// Legend items start from the left edge of the legend area.
    /// This is the default alignment.
    #[default]
    Left,

    /// Center alignment
    ///
    /// Legend items are centered within the legend area.
    /// Useful for creating balanced, symmetric layouts.
    Center,

    /// Right alignment
    ///
    /// Legend items align to the right edge of the legend area.
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legend_position_default() {
        assert_eq!(LegendPosition::default(), LegendPosition::Right);
    }

    #[test]
    fn legend_layout_default() {
        assert_eq!(LegendLayout::default(), LegendLayout::Vertical);
    }

    #[test]
    fn legend_position_clone() {
        let pos = LegendPosition::Left;
        let cloned = pos;
        assert_eq!(pos, cloned);
    }

    #[test]
    fn legend_layout_clone() {
        let layout = LegendLayout::Horizontal;
        let cloned = layout;
        assert_eq!(layout, cloned);
    }

    #[test]
    fn legend_position_debug() {
        let pos = LegendPosition::Top;
        let debug = format!("{pos:?}");
        assert_eq!(debug, "Top");
    }

    #[test]
    fn legend_layout_debug() {
        let layout = LegendLayout::Vertical;
        let debug = format!("{layout:?}");
        assert_eq!(debug, "Vertical");
    }

    #[test]
    fn legend_alignment_default() {
        assert_eq!(LegendAlignment::default(), LegendAlignment::Left);
    }

    #[test]
    fn legend_alignment_clone() {
        let alignment = LegendAlignment::Center;
        let cloned = alignment;
        assert_eq!(alignment, cloned);
    }

    #[test]
    fn legend_alignment_debug() {
        let alignment = LegendAlignment::Right;
        let debug = format!("{:?}", alignment);
        assert_eq!(debug, "Right");
    }
}
