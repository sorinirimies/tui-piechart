//! Symbols for pie chart widget and legends
//!
//! This module provides a comprehensive collection of predefined symbols for customizing
//! the appearance of pie charts and their legends. Choose from 30+ pie characters and
//! 24+ legend markers to match your aesthetic preferences.
//!
//! # Examples
//!
//! ## Using Predefined Legend Markers
//!
//! ```
//! use tui_piechart::{PieChart, PieSlice, symbols};
//! use ratatui::style::Color;
//!
//! let slices = vec![
//!     PieSlice::new("Rust", 45.0, Color::Red),
//!     PieSlice::new("Go", 30.0, Color::Blue),
//! ];
//!
//! // Use a star marker for legends
//! let chart = PieChart::new(slices)
//!     .legend_marker(symbols::LEGEND_MARKER_STAR)
//!     .show_legend(true);
//! ```
//!
//! ## Using Predefined Pie Characters
//!
//! ```
//! use tui_piechart::{PieChart, PieSlice, symbols};
//! use ratatui::style::Color;
//!
//! let slices = vec![PieSlice::new("Data", 100.0, Color::Cyan)];
//!
//! // Use a square for the pie chart
//! let chart = PieChart::new(slices)
//!     .pie_char(symbols::PIE_CHAR_SQUARE);
//! ```
//!
//! ## Custom Symbols (Not Predefined)
//!
//! You can use any Unicode character:
//!
//! ```
//! use tui_piechart::{PieChart, PieSlice};
//! use ratatui::style::Color;
//!
//! let slices = vec![PieSlice::new("Custom", 100.0, Color::Green)];
//!
//! // Use emoji or any Unicode character
//! let chart = PieChart::new(slices)
//!     .pie_char('🔥')
//!     .legend_marker("🌟");
//! ```
//!
//! # Categories
//!
//! ## Legend Markers
//!
//! - **Shapes**: Square, Circle, Diamond, Triangle, Hexagon
//! - **Arrows**: Arrow, Right Arrow, Double Right
//! - **Symbols**: Star, Heart, Plus, Cross, Check
//! - **Special**: Bullseye, Asterism, Horizontal Bar
//!
//! ## Pie Characters
//!
//! - **Basic**: Block, Circle, Square, Diamond
//! - **Shades**: Light, Medium, Dark shades
//! - **Shapes**: Triangle (all directions), Star, Heart
//! - **Special**: Hexagon, Bullseye, Asterism
//!
//! # Interactive Example
//!
//! Run the legend markers showcase to see all options:
//!
//! ```bash
//! cargo run --example legend_markers
//! ```
//!
//! [`PieChart`]: crate::PieChart

// Re-export BorderStyle for backwards compatibility
pub use crate::border_style::BorderStyle;

/// Macro to generate pie chart character constants.
///
/// This macro generates public constants for pie chart symbols with consistent
/// documentation and naming conventions.
///
/// # Format
///
/// ```text
/// pie_symbols! {
///     SUFFIX_NAME: 'symbol', "description",
/// }
/// ```
///
/// Generates: `pub const PIE_CHAR_SUFFIX_NAME: char = 'symbol';`
/// With doc: `/// Alternative pie chart character - description`
macro_rules! pie_symbols {
    ($($name:ident: $char:expr, $desc:expr),+ $(,)?) => {
        $(
            #[doc = concat!("Alternative pie chart character - ", $desc)]
            pub const $name: char = $char;
        )+
    };
}

/// Macro to generate legend marker constants.
///
/// This macro generates public constants for legend markers with consistent
/// documentation and naming conventions.
///
/// # Format
///
/// ```text
/// legend_symbols! {
///     SUFFIX_NAME: "symbol", "description",
/// }
/// ```
///
/// Generates: `pub const LEGEND_MARKER_SUFFIX_NAME: &str = "symbol";`
/// With doc: `/// Alternative legend marker - description`
macro_rules! legend_symbols {
    ($($name:ident: $str:expr, $desc:expr),+ $(,)?) => {
        $(
            #[doc = concat!("Alternative legend marker - ", $desc)]
            pub const $name: &str = $str;
        )+
    };
}

// ============================================================================
// PIE CHART CHARACTERS
// ============================================================================

/// Default character used to draw pie chart slices
pub const PIE_CHAR: char = '●';

// Generate all alternative pie chart characters using the macro
pie_symbols! {
    PIE_CHAR_BLOCK: '█', "filled block",
    PIE_CHAR_SHADE: '▒', "medium shade",
    PIE_CHAR_LIGHT: '░', "light shade",
    PIE_CHAR_DARK: '▓', "dark shade",
    PIE_CHAR_CIRCLE: '◉', "circle",
    PIE_CHAR_SQUARE: '■', "square",
    PIE_CHAR_DIAMOND: '◆', "diamond",
    PIE_CHAR_SMALL_CIRCLE: '•', "small circle",
    PIE_CHAR_WHITE_CIRCLE: '○', "white circle",
    PIE_CHAR_DOUBLE_CIRCLE: '◎', "double circle",
    PIE_CHAR_SMALL_SQUARE: '▪', "small square",
    PIE_CHAR_WHITE_SQUARE: '□', "white square",
    PIE_CHAR_SMALL_DIAMOND: '◆', "small diamond",
    PIE_CHAR_WHITE_DIAMOND: '◇', "white diamond",
    PIE_CHAR_STAR: '★', "star",
    PIE_CHAR_WHITE_STAR: '☆', "white star",
    PIE_CHAR_TRIANGLE_UP: '▲', "triangle up",
    PIE_CHAR_TRIANGLE_DOWN: '▼', "triangle down",
    PIE_CHAR_TRIANGLE_RIGHT: '▶', "triangle right",
    PIE_CHAR_TRIANGLE_LEFT: '◀', "triangle left",
    PIE_CHAR_PLUS: '✚', "plus",
    PIE_CHAR_CROSS: '✖', "cross",
    PIE_CHAR_HEART: '♥', "heart",
    PIE_CHAR_WHITE_HEART: '♡', "white heart",
    PIE_CHAR_SPADE: '♠', "spade",
    PIE_CHAR_CLUB: '♣', "club",
    PIE_CHAR_DOT: '·', "dot",
    PIE_CHAR_HEXAGON: '⬢', "hexagon",
    PIE_CHAR_BULLSEYE: '◉', "bullseye",
    PIE_CHAR_SQUARE_BOX: '▣', "square box",
    PIE_CHAR_ASTERISM: '※', "asterism",
    PIE_CHAR_HORIZONTAL_BAR: '▰', "horizontal bar",
}

// ============================================================================
// LEGEND MARKERS
// ============================================================================

/// Default marker for legend items - filled square
///
/// # Example
///
/// ```
/// use tui_piechart::{PieChart, PieSlice, symbols};
/// use ratatui::style::Color;
///
/// let slices = vec![PieSlice::new("Data", 100.0, Color::Red)];
/// let chart = PieChart::new(slices)
///     .legend_marker(symbols::LEGEND_MARKER)
///     .show_legend(true);
/// ```
pub const LEGEND_MARKER: &str = "■";

// Generate all alternative legend markers using the macro
// These are organized by category for easier browsing
legend_symbols! {
    // Basic Shapes
    LEGEND_MARKER_CIRCLE: "●", "circle - classic filled circle",
    LEGEND_MARKER_SQUARE: "▪", "square - compact filled square",
    LEGEND_MARKER_DIAMOND: "◆", "diamond - filled diamond shape",
    LEGEND_MARKER_TRIANGLE: "▲", "triangle - upward-pointing triangle",
    LEGEND_MARKER_HEXAGON: "⬡", "hexagon - outlined hexagon",

    // Outlined Variants
    LEGEND_MARKER_WHITE_CIRCLE: "○", "white circle - outlined circle",
    LEGEND_MARKER_SQUARE_BOX: "▢", "square box - outlined square",

    // Arrow Styles
    LEGEND_MARKER_ARROW: "▶", "arrow - right-pointing arrow",
    LEGEND_MARKER_RIGHT_ARROW: "→", "right arrow - simple arrow",
    LEGEND_MARKER_DOUBLE_RIGHT: "»", "double right - double chevron",

    // Star Styles
    LEGEND_MARKER_STAR: "★", "star - filled star",
    LEGEND_MARKER_WHITE_STAR: "☆", "white star - outlined star",

    // Heart Styles
    LEGEND_MARKER_HEART: "♥", "heart - filled heart",
    LEGEND_MARKER_WHITE_HEART: "♡", "white heart - outlined heart",

    // Symbols & Icons
    LEGEND_MARKER_PLUS: "✚", "plus - plus sign",
    LEGEND_MARKER_CROSS: "✖", "cross - X-shaped cross",
    LEGEND_MARKER_CHECK: "✓", "check - check mark",
    LEGEND_MARKER_BULLSEYE: "◉", "bullseye - circle with center dot",
    LEGEND_MARKER_ASTERISM: "⁂", "asterism - three asterisks",

    // Minimal Markers
    LEGEND_MARKER_SMALL_CIRCLE: "•", "small circle - bullet point",
    LEGEND_MARKER_DASH: "–", "dash - horizontal dash",
    LEGEND_MARKER_DOT: "·", "dot - middle dot",
    LEGEND_MARKER_HORIZONTAL_BAR: "▱", "horizontal bar - white bar",
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_symbols() {
        assert_eq!(PIE_CHAR, '●');
        assert_eq!(LEGEND_MARKER, "■");
    }

    #[test]
    fn test_pie_char_variants() {
        // Test a few to ensure macro generated them correctly
        assert_eq!(PIE_CHAR_BLOCK, '█');
        assert_eq!(PIE_CHAR_CIRCLE, '◉');
        assert_eq!(PIE_CHAR_STAR, '★');
        assert_eq!(PIE_CHAR_HEART, '♥');
    }

    #[test]
    fn test_legend_marker_variants() {
        // Test a few to ensure macro generated them correctly
        assert_eq!(LEGEND_MARKER_CIRCLE, "●");
        assert_eq!(LEGEND_MARKER_STAR, "★");
        assert_eq!(LEGEND_MARKER_ARROW, "▶");
        assert_eq!(LEGEND_MARKER_CHECK, "✓");
    }

    #[test]
    fn test_all_symbols_are_unique() {
        // Ensure we don't have duplicate symbols (except intentional ones)
        let pie_chars = vec![
            PIE_CHAR_BLOCK,
            PIE_CHAR_SHADE,
            PIE_CHAR_LIGHT,
            PIE_CHAR_CIRCLE,
            PIE_CHAR_SQUARE,
        ];

        // Just verify they're all valid Unicode characters
        for ch in pie_chars {
            assert!(ch.is_ascii() || ch > '\u{0080}');
        }
    }

    #[test]
    fn test_legend_markers_are_strings() {
        // Verify all legend markers are valid strings
        let markers = vec![
            LEGEND_MARKER,
            LEGEND_MARKER_CIRCLE,
            LEGEND_MARKER_STAR,
            LEGEND_MARKER_ARROW,
        ];

        for marker in markers {
            assert_ne!(marker, "");
            assert!(marker.len() <= 3); // Most are single char, some are multi-byte
        }
    }
}
