//! Border styles for pie chart block wrappers.
//!
//! This module provides predefined border styles that can be used to customize
//! the appearance of the block that wraps a pie chart.
//!
//! # Examples
//!
//! ```
//! use tui_piechart::{PieChart, PieSlice, border_style::BorderStyle};
//! use ratatui::style::Color;
//!
//! let slices = vec![PieSlice::new("Rust", 45.0, Color::Red)];
//! let piechart = PieChart::new(slices)
//!     .block(BorderStyle::Rounded.block().title("My Chart"));
//! ```

use ratatui::symbols::border;
use ratatui::widgets::Block;

// Re-export title positioning types for backward compatibility
// The actual implementations are in the `title` module
pub use crate::title::{BlockExt, TitleAlignment, TitlePosition};

// ============================================================================
// BORDER STYLE ENUM
// ============================================================================

/// Predefined border styles for the pie chart block wrapper.
///
/// These styles provide convenient ways to customize the appearance of the
/// block that wraps the pie chart.
///
/// # Unicode Limitations
///
/// Note that `DoubleLineRounded` and `ThickRounded` use mixed styles because
/// Unicode doesn't have true rounded double-line or thick-line box-drawing
/// characters. These styles use single-line rounded corners with double/thick
/// edges for a softer appearance.
///
/// # Examples
///
/// ```
/// use tui_piechart::{PieChart, PieSlice, border_style::BorderStyle};
/// use ratatui::style::Color;
///
/// let slices = vec![PieSlice::new("Rust", 45.0, Color::Red)];
///
/// // Dashed borders for a subtle look
/// let chart = PieChart::new(slices.clone())
///     .block(BorderStyle::Dashed.block().title("Dashed"));
///
/// // Thick rounded for bold emphasis
/// let chart = PieChart::new(slices)
///     .block(BorderStyle::ThickRounded.block().title("Bold"));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BorderStyle {
    /// Standard single-line borders (default)
    #[default]
    Standard,
    /// Rounded corners with single-line borders
    Rounded,
    /// Dashed lines throughout (┄┄┄)
    Dashed,
    /// Rounded corners with dashed borders
    RoundedDashed,
    /// Standard borders with gaps only at corners
    CornerGapped,
    /// Rounded borders with gaps only at corners
    RoundedCornerGapped,
    /// Double-line borders for standard style
    DoubleLineStandard,
    /// Double-line borders with rounded corners (mixed: rounded corners + double edges)
    DoubleLineRounded,
    /// Thick borders (uses heavy line drawing characters)
    Thick,
    /// Thick borders with rounded corners (mixed: rounded corners + thick edges)
    ThickRounded,
    /// Thick borders with dashed lines
    ThickDashed,
    /// Thick borders with gaps only at corners
    ThickCornerGapped,
}

impl BorderStyle {
    /// Creates a new `Block` with the specified border style.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_piechart::border_style::BorderStyle;
    ///
    /// let block = BorderStyle::Rounded.block().title("My Chart");
    /// ```
    #[must_use]
    pub fn block(self) -> Block<'static> {
        match self {
            Self::Standard => Block::bordered(),
            Self::Rounded => Block::bordered().border_set(border::ROUNDED),
            Self::Dashed => Block::bordered().border_set(BORDER_DASHED),
            Self::RoundedDashed => Block::bordered().border_set(BORDER_ROUNDED_DASHED),
            Self::CornerGapped => Block::bordered().border_set(BORDER_CORNER_GAPPED),
            Self::RoundedCornerGapped => Block::bordered().border_set(BORDER_ROUNDED_CORNER_GAPPED),
            Self::DoubleLineStandard => Block::bordered().border_set(border::DOUBLE),
            Self::DoubleLineRounded => Block::bordered().border_set(BORDER_DOUBLE_ROUNDED),
            Self::Thick => Block::bordered().border_set(border::THICK),
            Self::ThickRounded => Block::bordered().border_set(BORDER_THICK_ROUNDED),
            Self::ThickDashed => Block::bordered().border_set(BORDER_THICK_DASHED),
            Self::ThickCornerGapped => Block::bordered().border_set(BORDER_THICK_CORNER_GAPPED),
        }
    }
}

// ============================================================================
// CUSTOM BORDER SETS
// ============================================================================

/// Border set with dashed lines throughout
///
/// Uses Unicode dashed line characters (┄ and ┊) for a subtle, non-intrusive
/// border appearance.
pub const BORDER_DASHED: border::Set = border::Set {
    top_left: "┌",
    top_right: "┐",
    bottom_left: "└",
    bottom_right: "┘",
    vertical_left: "┊",
    vertical_right: "┊",
    horizontal_top: "┄",
    horizontal_bottom: "┄",
};

/// Border set with rounded corners and dashed lines
///
/// Combines rounded corners with dashed lines for a modern, subtle look.
pub const BORDER_ROUNDED_DASHED: border::Set = border::Set {
    top_left: "╭",
    top_right: "╮",
    bottom_left: "╰",
    bottom_right: "╯",
    vertical_left: "┊",
    vertical_right: "┊",
    horizontal_top: "┄",
    horizontal_bottom: "┄",
};

/// Border set with gaps only at corners (standard lines)
///
/// Provides a minimalist look with continuous lines and gaps only at corners.
pub const BORDER_CORNER_GAPPED: border::Set = border::Set {
    top_left: " ",
    top_right: " ",
    bottom_left: " ",
    bottom_right: " ",
    vertical_left: "│",
    vertical_right: "│",
    horizontal_top: "─",
    horizontal_bottom: "─",
};

/// Border set with rounded corners and gaps only at corners
///
/// Provides a minimalist look with continuous lines, rounded appearance, and
/// gaps only at corners.
pub const BORDER_ROUNDED_CORNER_GAPPED: border::Set = border::Set {
    top_left: " ",
    top_right: " ",
    bottom_left: " ",
    bottom_right: " ",
    vertical_left: "│",
    vertical_right: "│",
    horizontal_top: "─",
    horizontal_bottom: "─",
};

/// Border set with double lines and rounded corners (mixed style)
///
/// # Unicode Limitation
///
/// Unicode doesn't have true rounded double-line corners, so this uses
/// rounded single-line corners with double-line edges for a softer appearance.
///
/// Visual: `╭═══╮` instead of the non-existent `╔═══╗` with rounded corners.
pub const BORDER_DOUBLE_ROUNDED: border::Set = border::Set {
    top_left: "╭",
    top_right: "╮",
    bottom_left: "╰",
    bottom_right: "╯",
    vertical_left: "║",
    vertical_right: "║",
    horizontal_top: "═",
    horizontal_bottom: "═",
};

/// Border set with thick lines and rounded corners (mixed style)
///
/// # Unicode Limitation
///
/// Unicode doesn't have true rounded thick-line corners, so this uses
/// rounded single-line corners with thick-line edges for a softer appearance.
///
/// Visual: `╭━━━╮` instead of the non-existent `┏━━━┓` with rounded corners.
pub const BORDER_THICK_ROUNDED: border::Set = border::Set {
    top_left: "╭",
    top_right: "╮",
    bottom_left: "╰",
    bottom_right: "╯",
    vertical_left: "┃",
    vertical_right: "┃",
    horizontal_top: "━",
    horizontal_bottom: "━",
};

/// Border set with thick dashed lines
///
/// Uses Unicode heavy dashed line characters (┅ and ┇) for bold, non-intrusive
/// emphasis.
pub const BORDER_THICK_DASHED: border::Set = border::Set {
    top_left: "┏",
    top_right: "┓",
    bottom_left: "┗",
    bottom_right: "┛",
    vertical_left: "┇",
    vertical_right: "┇",
    horizontal_top: "┅",
    horizontal_bottom: "┅",
};

/// Border set with thick lines and gaps only at corners
///
/// Provides a bold minimalist look with thick continuous lines and gaps only
/// at corners.
pub const BORDER_THICK_CORNER_GAPPED: border::Set = border::Set {
    top_left: " ",
    top_right: " ",
    bottom_left: " ",
    bottom_right: " ",
    vertical_left: "┃",
    vertical_right: "┃",
    horizontal_top: "━",
    horizontal_bottom: "━",
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn border_style_default() {
        assert_eq!(BorderStyle::default(), BorderStyle::Standard);
    }

    #[test]
    fn border_style_block_creation() {
        // Test that all variants can create blocks without panicking
        let styles = [
            BorderStyle::Standard,
            BorderStyle::Rounded,
            BorderStyle::Dashed,
            BorderStyle::RoundedDashed,
            BorderStyle::CornerGapped,
            BorderStyle::RoundedCornerGapped,
            BorderStyle::DoubleLineStandard,
            BorderStyle::DoubleLineRounded,
            BorderStyle::Thick,
            BorderStyle::ThickRounded,
            BorderStyle::ThickDashed,
            BorderStyle::ThickCornerGapped,
        ];

        for style in &styles {
            let _block = style.block();
        }
    }

    #[test]
    fn border_style_equality() {
        assert_eq!(BorderStyle::Standard, BorderStyle::Standard);
        assert_ne!(BorderStyle::Standard, BorderStyle::Rounded);
    }

    #[test]
    fn border_sets_have_valid_characters() {
        // Ensure all border sets have non-empty strings
        let sets = [
            BORDER_DASHED,
            BORDER_ROUNDED_DASHED,
            BORDER_CORNER_GAPPED,
            BORDER_ROUNDED_CORNER_GAPPED,
            BORDER_DOUBLE_ROUNDED,
            BORDER_THICK_ROUNDED,
            BORDER_THICK_DASHED,
            BORDER_THICK_CORNER_GAPPED,
        ];

        for set in &sets {
            assert!(!set.top_left.is_empty());
            assert!(!set.top_right.is_empty());
            assert!(!set.bottom_left.is_empty());
            assert!(!set.bottom_right.is_empty());
            assert!(!set.vertical_left.is_empty());
            assert!(!set.vertical_right.is_empty());
            assert!(!set.horizontal_top.is_empty());
            assert!(!set.horizontal_bottom.is_empty());
        }
    }

    // Note: Title alignment and position tests are in the `title` module
}
