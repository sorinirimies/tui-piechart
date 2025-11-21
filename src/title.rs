//! Title positioning, alignment, and styling configuration for block wrappers.
//!
//! This module provides types and functionality for controlling where and how
//! block titles are positioned, aligned, and styled with different Unicode fonts.
//!
//! # Examples
//!
//! ```
//! use tui_piechart::title::{TitleAlignment, TitlePosition, TitleStyle, BlockExt};
//! use tui_piechart::border_style::BorderStyle;
//!
//! // Create a block with centered bold title at the bottom
//! let title = TitleStyle::Bold.apply("My Chart");
//! let block = BorderStyle::Rounded.block()
//!     .title(title)
//!     .title_alignment_horizontal(TitleAlignment::Center)
//!     .title_vertical_position(TitlePosition::Bottom);
//! ```

use ratatui::layout::Alignment;
use ratatui::widgets::Block;

/// Horizontal alignment for block titles.
///
/// Controls how the title text is aligned horizontally within the block's top
/// or bottom border. Supports start (left), center, and end (right) alignment.
///
/// # Examples
///
/// ```
/// use tui_piechart::title::{TitleAlignment, BlockExt};
/// use tui_piechart::border_style::BorderStyle;
///
/// let block = BorderStyle::Rounded.block()
///     .title("Centered Title")
///     .title_alignment_horizontal(TitleAlignment::Center);
/// ```
///
/// # Text Direction
///
/// The alignment is logical rather than physical:
/// - **Start**: Left in LTR languages, right in RTL languages
/// - **Center**: Always centered
/// - **End**: Right in LTR languages, left in RTL languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TitleAlignment {
    /// Start-aligned title (left in LTR, right in RTL)
    ///
    /// The title appears at the start of the text direction. For left-to-right
    /// languages (like English), this means left-aligned.
    Start,

    /// Center-aligned title (default)
    ///
    /// The title appears centered horizontally within the block border.
    /// This is the default alignment.
    #[default]
    Center,

    /// End-aligned title (right in LTR, left in RTL)
    ///
    /// The title appears at the end of the text direction. For left-to-right
    /// languages (like English), this means right-aligned.
    End,
}

impl From<TitleAlignment> for Alignment {
    fn from(alignment: TitleAlignment) -> Self {
        match alignment {
            TitleAlignment::Start => Alignment::Left,
            TitleAlignment::Center => Alignment::Center,
            TitleAlignment::End => Alignment::Right,
        }
    }
}

/// Vertical position for block titles.
///
/// Controls whether the title appears at the top or bottom of the block border.
///
/// # Examples
///
/// ```
/// use tui_piechart::title::{TitlePosition, BlockExt};
/// use tui_piechart::border_style::BorderStyle;
///
/// let block = BorderStyle::Rounded.block()
///     .title("Bottom Title")
///     .title_vertical_position(TitlePosition::Bottom);
/// ```
///
/// # Combinations
///
/// Title position can be combined with horizontal alignment to create
/// 6 different title placements:
/// - Top-Start, Top-Center, Top-End
/// - Bottom-Start, Bottom-Center, Bottom-End
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TitlePosition {
    /// Title at the top (default)
    ///
    /// The title appears in the top border of the block. This is the default
    /// position and is the most common placement for block titles.
    #[default]
    Top,

    /// Title at the bottom
    ///
    /// The title appears in the bottom border of the block. Useful when you
    /// want to place other content at the top or when the title serves as
    /// a caption rather than a header.
    Bottom,
}

/// Font style for block titles using Unicode character variants.
///
/// Converts regular ASCII text to different Unicode character sets to achieve
/// visual font styles in terminal user interfaces. Each style uses specific
/// Unicode code points that represent the same letters in different typographic styles.
///
/// # Examples
///
/// ```
/// use tui_piechart::title::TitleStyle;
///
/// let bold = TitleStyle::Bold.apply("Statistics");
/// let italic = TitleStyle::Italic.apply("Results");
/// let script = TitleStyle::Script.apply("Elegant");
/// ```
///
/// # Limitations
///
/// - Only supports ASCII letters (a-z, A-Z), numbers (0-9), and spaces
/// - Other characters (punctuation, special symbols) are passed through unchanged
/// - Terminal font must support the Unicode characters (most modern terminals do)
/// - Some styles may not render identically across different fonts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TitleStyle {
    /// Normal/regular text (default) - no transformation applied
    #[default]
    Normal,

    /// Bold text using Unicode Mathematical Bold characters
    ///
    /// Converts text to bold Unicode variants. Example: "Hello" → "𝐇𝐞𝐥𝐥𝐨"
    Bold,

    /// Italic text using Unicode Mathematical Italic characters
    ///
    /// Converts text to italic Unicode variants. Example: "Hello" → "𝐻𝑒𝑙𝑙𝑜"
    Italic,

    /// Bold Italic text using Unicode Mathematical Bold Italic characters
    ///
    /// Combines bold and italic styling. Example: "Hello" → "𝑯𝒆𝒍𝒍𝒐"
    BoldItalic,

    /// Script/cursive text using Unicode Mathematical Script characters
    ///
    /// Converts text to flowing script style. Example: "Hello" → "𝐻ℯ𝓁𝓁ℴ"
    Script,

    /// Bold Script text using Unicode Mathematical Bold Script characters
    ///
    /// Script style with bold weight. Example: "Hello" → "𝓗𝓮𝓵𝓵𝓸"
    BoldScript,

    /// Sans-serif text using Unicode Mathematical Sans-Serif characters
    ///
    /// Clean sans-serif style. Example: "Hello" → "𝖧𝖾𝗅𝗅𝗈"
    SansSerif,

    /// Bold Sans-serif text using Unicode Mathematical Sans-Serif Bold characters
    ///
    /// Bold sans-serif style. Example: "Hello" → "𝗛𝗲𝗹𝗹𝗼"
    BoldSansSerif,

    /// Italic Sans-serif text using Unicode Mathematical Sans-Serif Italic characters
    ///
    /// Italic sans-serif style. Example: "Hello" → "𝘏𝘦𝘭𝘭𝘰"
    ItalicSansSerif,

    /// Monospace text using Unicode Monospace characters
    ///
    /// Fixed-width monospace style. Example: "Hello" → "𝙷𝚎𝚕𝚕𝚘"
    Monospace,
}

impl TitleStyle {
    /// Apply this style to the given text.
    ///
    /// Converts ASCII letters and numbers to their Unicode equivalents in the
    /// selected style. Non-ASCII characters and unsupported characters are
    /// passed through unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_piechart::title::TitleStyle;
    ///
    /// let bold = TitleStyle::Bold.apply("Chart 2024");
    /// let italic = TitleStyle::Italic.apply("Statistics");
    /// let script = TitleStyle::Script.apply("Elegant Title");
    /// ```
    ///
    /// # Character Support
    ///
    /// - **Letters**: Full support for a-z and A-Z
    /// - **Numbers**: Support varies by style (most support 0-9)
    /// - **Spaces**: Preserved as-is
    /// - **Punctuation**: Passed through unchanged
    #[must_use]
    pub fn apply(&self, text: &str) -> String {
        match self {
            Self::Normal => text.to_string(),
            Self::Bold => convert_to_bold(text),
            Self::Italic => convert_to_italic(text),
            Self::BoldItalic => convert_to_bold_italic(text),
            Self::Script => convert_to_script(text),
            Self::BoldScript => convert_to_bold_script(text),
            Self::SansSerif => convert_to_sans_serif(text),
            Self::BoldSansSerif => convert_to_bold_sans_serif(text),
            Self::ItalicSansSerif => convert_to_italic_sans_serif(text),
            Self::Monospace => convert_to_monospace(text),
        }
    }
}

// Unicode conversion functions - using macro to reduce code duplication

/// Macro to generate Unicode conversion functions.
///
/// This macro generates functions that convert ASCII text to Unicode character variants.
/// It reduces code duplication by handling the repetitive pattern of mapping character
/// ranges to Unicode code points.
///
/// # Parameters
/// - `$name`: Function name
/// - `$upper`: Unicode base for uppercase letters (A-Z)
/// - `$lower`: Unicode base for lowercase letters (a-z)
/// - `$digit`: Optional Unicode base for digits (0-9)
macro_rules! unicode_converter {
    // Version with digit support
    ($name:ident, $upper:expr, $lower:expr, $digit:expr) => {
        fn $name(text: &str) -> String {
            text.chars()
                .map(|c| match c {
                    'A'..='Z' => char::from_u32($upper + (c as u32 - 'A' as u32)).unwrap(),
                    'a'..='z' => char::from_u32($lower + (c as u32 - 'a' as u32)).unwrap(),
                    '0'..='9' => char::from_u32($digit + (c as u32 - '0' as u32)).unwrap(),
                    _ => c,
                })
                .collect()
        }
    };
    // Version without digit support
    ($name:ident, $upper:expr, $lower:expr) => {
        fn $name(text: &str) -> String {
            text.chars()
                .map(|c| match c {
                    'A'..='Z' => char::from_u32($upper + (c as u32 - 'A' as u32)).unwrap(),
                    'a'..='z' => char::from_u32($lower + (c as u32 - 'a' as u32)).unwrap(),
                    _ => c,
                })
                .collect()
        }
    };
}

// Generate all Unicode conversion functions using the macro
unicode_converter!(convert_to_bold, 0x1D400, 0x1D41A, 0x1D7CE);
unicode_converter!(convert_to_italic, 0x1D434, 0x1D44E);
unicode_converter!(convert_to_bold_italic, 0x1D468, 0x1D482);
unicode_converter!(convert_to_script, 0x1D49C, 0x1D4B6);
unicode_converter!(convert_to_bold_script, 0x1D4D0, 0x1D4EA);
unicode_converter!(convert_to_sans_serif, 0x1D5A0, 0x1D5BA, 0x1D7E2);
unicode_converter!(convert_to_bold_sans_serif, 0x1D5D4, 0x1D5EE, 0x1D7EC);
unicode_converter!(convert_to_italic_sans_serif, 0x1D608, 0x1D622);
unicode_converter!(convert_to_monospace, 0x1D670, 0x1D68A, 0x1D7F6);

/// Extension trait for adding title positioning helpers to Block.
///
/// This trait provides ergonomic methods for setting title alignment and position
/// on Ratatui's `Block` type. It allows for method chaining and uses semantic
/// types instead of raw alignment values.
///
/// # Examples
///
/// ```
/// use tui_piechart::title::{TitleAlignment, TitlePosition, BlockExt};
/// use ratatui::widgets::Block;
///
/// let block = Block::bordered()
///     .title("My Chart")
///     .title_alignment_horizontal(TitleAlignment::Center)
///     .title_vertical_position(TitlePosition::Bottom);
/// ```
///
/// # Method Chaining
///
/// All methods return `Self`, allowing for fluent method chaining:
///
/// ```
/// use tui_piechart::title::{TitleAlignment, TitlePosition, BlockExt};
/// use tui_piechart::border_style::BorderStyle;
///
/// let block = BorderStyle::Rounded.block()
///     .title("Statistics")
///     .title_alignment_horizontal(TitleAlignment::End)
///     .title_vertical_position(TitlePosition::Bottom);
/// ```
pub trait BlockExt<'a> {
    /// Sets the horizontal alignment of the title.
    ///
    /// Controls whether the title appears at the start (left), center, or end (right)
    /// of the block border.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_piechart::title::{TitleAlignment, BlockExt};
    /// use ratatui::widgets::Block;
    ///
    /// let block = Block::bordered()
    ///     .title("My Chart")
    ///     .title_alignment_horizontal(TitleAlignment::Center);
    /// ```
    #[must_use]
    fn title_alignment_horizontal(self, alignment: TitleAlignment) -> Self;

    /// Sets the vertical position of the title.
    ///
    /// Controls whether the title appears at the top or bottom of the block border.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_piechart::title::{TitlePosition, BlockExt};
    /// use ratatui::widgets::Block;
    ///
    /// let block = Block::bordered()
    ///     .title("My Chart")
    ///     .title_vertical_position(TitlePosition::Bottom);
    /// ```
    #[must_use]
    fn title_vertical_position(self, position: TitlePosition) -> Self;
}

impl<'a> BlockExt<'a> for Block<'a> {
    fn title_alignment_horizontal(self, alignment: TitleAlignment) -> Self {
        self.title_alignment(alignment.into())
    }

    fn title_vertical_position(self, position: TitlePosition) -> Self {
        use ratatui::widgets::block::Position as RatatuiPosition;
        match position {
            TitlePosition::Top => self.title_position(RatatuiPosition::Top),
            TitlePosition::Bottom => self.title_position(RatatuiPosition::Bottom),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_alignment_default() {
        assert_eq!(TitleAlignment::default(), TitleAlignment::Center);
    }

    #[test]
    fn title_position_default() {
        assert_eq!(TitlePosition::default(), TitlePosition::Top);
    }

    #[test]
    fn title_style_default() {
        assert_eq!(TitleStyle::default(), TitleStyle::Normal);
    }

    #[test]
    fn title_alignment_to_ratatui_alignment() {
        assert_eq!(Alignment::from(TitleAlignment::Start), Alignment::Left);
        assert_eq!(Alignment::from(TitleAlignment::Center), Alignment::Center);
        assert_eq!(Alignment::from(TitleAlignment::End), Alignment::Right);
    }

    #[test]
    fn title_alignment_clone() {
        let align = TitleAlignment::End;
        let cloned = align;
        assert_eq!(align, cloned);
    }

    #[test]
    fn title_position_clone() {
        let pos = TitlePosition::Bottom;
        let cloned = pos;
        assert_eq!(pos, cloned);
    }

    #[test]
    fn title_style_clone() {
        let style = TitleStyle::Bold;
        let cloned = style;
        assert_eq!(style, cloned);
    }

    #[test]
    fn title_alignment_debug() {
        let align = TitleAlignment::Start;
        let debug = format!("{:?}", align);
        assert_eq!(debug, "Start");
    }

    #[test]
    fn title_position_debug() {
        let pos = TitlePosition::Bottom;
        let debug = format!("{:?}", pos);
        assert_eq!(debug, "Bottom");
    }

    #[test]
    fn title_style_debug() {
        let style = TitleStyle::Bold;
        let debug = format!("{:?}", style);
        assert_eq!(debug, "Bold");
    }

    #[test]
    fn block_ext_title_alignment() {
        let block = Block::bordered()
            .title("Test")
            .title_alignment_horizontal(TitleAlignment::Center);
        // If this compiles and doesn't panic, the trait is working
        assert!(format!("{:?}", block).contains("Test"));
    }

    #[test]
    fn block_ext_title_position() {
        let block = Block::bordered()
            .title("Test")
            .title_vertical_position(TitlePosition::Bottom);
        // If this compiles and doesn't panic, the trait is working
        assert!(format!("{:?}", block).contains("Test"));
    }

    #[test]
    fn block_ext_method_chaining() {
        let block = Block::bordered()
            .title("Test")
            .title_alignment_horizontal(TitleAlignment::End)
            .title_vertical_position(TitlePosition::Bottom);
        // If this compiles and doesn't panic, method chaining works
        assert!(format!("{:?}", block).contains("Test"));
    }

    #[test]
    fn title_style_normal() {
        let text = "Hello World";
        assert_eq!(TitleStyle::Normal.apply(text), "Hello World");
    }

    #[test]
    fn title_style_bold_letters() {
        let result = TitleStyle::Bold.apply("Hello");
        assert_ne!(result, "Hello");
        assert_eq!(result.chars().count(), 5); // Same length
    }

    #[test]
    fn title_style_bold_with_numbers() {
        let result = TitleStyle::Bold.apply("Chart 2024");
        assert!(result.chars().count() >= 10); // At least same length
    }

    #[test]
    fn title_style_italic_letters() {
        let result = TitleStyle::Italic.apply("Statistics");
        assert_ne!(result, "Statistics");
    }

    #[test]
    fn title_style_preserves_spaces() {
        let result = TitleStyle::Bold.apply("Hello World");
        assert!(result.contains(' '));
    }

    #[test]
    fn title_style_preserves_punctuation() {
        let result = TitleStyle::Bold.apply("Hello!");
        assert!(result.ends_with('!'));
    }

    #[test]
    fn title_style_script() {
        let result = TitleStyle::Script.apply("Test");
        assert_ne!(result, "Test");
    }

    #[test]
    fn title_style_monospace() {
        let result = TitleStyle::Monospace.apply("Code");
        assert_ne!(result, "Code");
    }

    #[test]
    fn title_style_sans_serif() {
        let result = TitleStyle::SansSerif.apply("Modern");
        assert_ne!(result, "Modern");
    }

    #[test]
    fn title_style_empty_string() {
        assert_eq!(TitleStyle::Bold.apply(""), "");
        assert_eq!(TitleStyle::Italic.apply(""), "");
    }

    #[test]
    fn title_style_mixed_case() {
        let result = TitleStyle::Bold.apply("TeSt");
        assert_ne!(result, "TeSt");
        assert_eq!(result.chars().count(), 4);
    }
}
