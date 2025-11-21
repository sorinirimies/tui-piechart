//! Macro utilities for reducing boilerplate code.
//!
//! This module provides reusable macros that help eliminate repetitive patterns
//! throughout the crate. Macros are organized by category into submodules.
//!
//! # Module Organization
//!
//! - **`test`** - Testing utilities and assertion macros
//!
//! # Usage
//!
//! Macros are automatically available throughout the crate due to `#[macro_use]`.
//! External users can import them from the crate root:
//!
//! ```
//! use tui_piechart::{assert_test, enum_tests};
//! ```
//!
//! # Available Macros
//!
//! ## Test Utilities
//!
//! - [`enum_tests!`] - Generate default, clone, and debug tests for enums
//! - [`assert_test!`] - Create simple boolean assertion tests
//! - [`assert_eq_test!`] - Create equality comparison tests
//! - [`debug_format_tests!`] - Test debug formatting for multiple enum variants
//! - [`conversion_test!`] - Test From/Into trait implementations
//! - [`instantiate_variants_test!`] - Verify all enum variants are accessible
//! - [`no_panic_test!`] - Ensure code blocks don't panic
//! - [`string_transform_test!`] - Test string transformations preserve properties
//! - [`render_test!`] - Test that widget rendering doesn't panic
//! - [`render_with_size_test!`] - Test rendering with specific dimensions
//! - [`render_empty_test!`] - Test rendering to empty area
//! - [`multi_render_test!`] - Test multiple widget configurations
//!
//! # Examples
//!
//! ## Testing Enums
//!
//! ```
//! # use tui_piechart::enum_tests;
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
//! enum Position {
//!     #[default]
//!     Top,
//!     Bottom,
//! }
//!
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!
//!     enum_tests! {
//!         enum_type: Position,
//!         default_test: (test_default, Position::Top),
//!         clone_test: (test_clone, Position::Bottom),
//!         debug_test: (test_debug, Position::Bottom, "Bottom"),
//!     }
//! }
//! ```
//!
//! ## Debug Format Testing
//!
//! ```
//! # use tui_piechart::debug_format_tests;
//! # #[derive(Debug)]
//! # enum Alignment { Start, Center, End }
//! #[cfg(test)]
//! mod tests {
//!     # use super::*;
//!     debug_format_tests! {
//!         enum_type: Alignment,
//!         tests: [
//!             (test_start, Alignment::Start, "Start"),
//!             (test_center, Alignment::Center, "Center"),
//!         ]
//!     }
//! }
//! ```
//!
//! ## Render Testing
//!
//! ```ignore
//! # use tui_piechart::{render_test, render_with_size_test};
//! # use tui_piechart::PieChart;
//! # use ratatui::layout::Rect;
//! #[cfg(test)]
//! mod tests {
//!     render_test!(test_basic_render, PieChart::default(), Rect::new(0, 0, 40, 20));
//!     render_with_size_test!(test_small, PieChart::default(), width: 20, height: 10);
//! }
//! ```

#[macro_use]
pub mod test;
