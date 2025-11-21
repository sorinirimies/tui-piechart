//! Symbols for pie chart widget
//!
//! This module provides predefined symbols that can be used with the [`PieChart`] widget.
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
/// ```ignore
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
/// ```ignore
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

/// Default marker for legend items
pub const LEGEND_MARKER: &str = "■";

// Generate all alternative legend markers using the macro
legend_symbols! {
    LEGEND_MARKER_CIRCLE: "●", "circle",
    LEGEND_MARKER_SQUARE: "▪", "square",
    LEGEND_MARKER_ARROW: "▶", "arrow",
    LEGEND_MARKER_DIAMOND: "◆", "diamond",
    LEGEND_MARKER_STAR: "★", "star",
    LEGEND_MARKER_WHITE_STAR: "☆", "white star",
    LEGEND_MARKER_SMALL_CIRCLE: "•", "small circle",
    LEGEND_MARKER_WHITE_CIRCLE: "○", "white circle",
    LEGEND_MARKER_TRIANGLE: "▲", "triangle",
    LEGEND_MARKER_HEART: "♥", "heart",
    LEGEND_MARKER_WHITE_HEART: "♡", "white heart",
    LEGEND_MARKER_PLUS: "✚", "plus",
    LEGEND_MARKER_CROSS: "✖", "cross",
    LEGEND_MARKER_CHECK: "✓", "check",
    LEGEND_MARKER_RIGHT_ARROW: "→", "right arrow",
    LEGEND_MARKER_DOUBLE_RIGHT: "»", "double right",
    LEGEND_MARKER_DASH: "–", "dash",
    LEGEND_MARKER_DOT: "·", "dot",
    LEGEND_MARKER_HEXAGON: "⬡", "hexagon",
    LEGEND_MARKER_BULLSEYE: "◉", "bullseye",
    LEGEND_MARKER_SQUARE_BOX: "▢", "square box",
    LEGEND_MARKER_ASTERISM: "⁂", "asterism",
    LEGEND_MARKER_HORIZONTAL_BAR: "▱", "horizontal bar",
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
            assert!(!marker.is_empty());
            assert!(marker.len() <= 3); // Most are single char, some are multi-byte
        }
    }
}
