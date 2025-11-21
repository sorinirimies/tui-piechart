//! Test utility macros for reducing boilerplate code.
//!
//! This module contains macros specifically designed for testing patterns,
//! helping eliminate repetitive test code across the crate.
//!
//! # Overview
//!
//! The macros in this module help with:
//! - Enum testing (default, clone, debug)
//! - Assertion tests
//! - Debug format verification
//! - Type conversions
//! - String transformations
//! - Render/visual tests (widget rendering without panics)
//!
//! # Organization
//!
//! This is part of the `macros` module family:
//! - **`macros::test`** - Test utilities (this module)
//! - Module-specific macros live in their respective files (e.g., `unicode_converter!` in `title.rs`)
//!
//! # Usage Examples
//!
//! ## Testing Enums
//!
//! ```
//! # use tui_piechart::enum_tests;
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
//! enum MyPosition {
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
//!         enum_type: MyPosition,
//!         default_test: (test_default, MyPosition::Top),
//!         clone_test: (test_clone, MyPosition::Bottom),
//!         debug_test: (test_debug, MyPosition::Bottom, "Bottom"),
//!     }
//! }
//! ```
//!
//! ## Testing Multiple Debug Formats
//!
//! ```
//! # use tui_piechart::debug_format_tests;
//! # #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
//! # enum MyAlignment {
//! #     #[default]
//! #     Start,
//! #     Center,
//! #     End,
//! # }
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!
//!     debug_format_tests! {
//!         enum_type: MyAlignment,
//!         tests: [
//!             (test_start_debug, MyAlignment::Start, "Start"),
//!             (test_center_debug, MyAlignment::Center, "Center"),
//!             (test_end_debug, MyAlignment::End, "End"),
//!         ]
//!     }
//! }
//! ```
//!
//! ## Simple Assertions
//!
//! ```
//! # use tui_piechart::assert_test;
//! # use tui_piechart::assert_eq_test;
//! #[cfg(test)]
//! mod tests {
//!     assert_test!(test_is_empty, "".is_empty());
//!     assert_eq_test!(test_addition, 2 + 2, 4);
//! }
//! ```
//!
//! ## Render Tests
//!
//! ```ignore
//! use tui_piechart::{render_test, render_with_size_test, render_empty_test};
//! use tui_piechart::PieChart;
//! use ratatui::layout::Rect;
//!
//! #[cfg(test)]
//! mod tests {
//!     render_test!(test_basic_render, PieChart::default(), Rect::new(0, 0, 40, 20));
//!     render_with_size_test!(test_small_size, PieChart::default(), width: 20, height: 10);
//!     render_empty_test!(test_empty_area, PieChart::default());
//! }
//! ```
//!
//! # Benefits
//!
//! - **Reduces boilerplate**: Write less repetitive test code
//! - **Consistency**: All tests follow the same pattern
//! - **Maintainability**: Change patterns in one place
//! - **Readability**: Declarative test definitions
//! - **Visual testing**: Ensure widgets render without panics

/// Generate standard enum tests (default, clone, debug).
///
/// This macro generates common test cases for enums that implement Default, Clone,
/// Copy, Debug, and `PartialEq`. It reduces boilerplate in test modules.
///
/// # Examples
///
/// ```ignore
/// #[cfg(test)]
/// mod tests {
///     use super::*;
///
///     enum_tests! {
///         enum_type: MyEnum,
///         default_test: (test_default, MyEnum::DefaultVariant),
///         clone_test: (test_clone, MyEnum::OtherVariant),
///         debug_test: (test_debug, MyEnum::OtherVariant, "OtherVariant"),
///     }
/// }
/// ```
#[macro_export]
macro_rules! enum_tests {
    (
        enum_type: $enum_name:ty,
        default_test: ($default_test_name:ident, $default_variant:expr),
        clone_test: ($clone_test_name:ident, $test_variant:expr),
        debug_test: ($debug_test_name:ident, $debug_variant:expr, $debug_str:expr $(,)?),
    ) => {
        #[test]
        fn $default_test_name() {
            assert_eq!(<$enum_name>::default(), $default_variant);
        }

        #[test]
        fn $clone_test_name() {
            let value = $test_variant;
            let cloned = value;
            assert_eq!(value, cloned);
        }

        #[test]
        fn $debug_test_name() {
            let value = $debug_variant;
            let debug = format!("{:?}", value);
            assert_eq!(debug, $debug_str);
        }
    };
}

/// Generate a simple assertion test.
///
/// Creates a test function with a given name and assertion.
///
/// # Examples
///
/// ```ignore
/// assert_test!(test_addition, 2 + 2 == 4);
/// assert_test!(test_string, "hello".len() == 5);
/// ```
#[macro_export]
macro_rules! assert_test {
    ($test_name:ident, $assertion:expr) => {
        #[test]
        fn $test_name() {
            assert!($assertion);
        }
    };
}

/// Generate an equality assertion test.
///
/// Creates a test that checks if two expressions are equal.
///
/// # Examples
///
/// ```ignore
/// assert_eq_test!(test_math, 2 + 2, 4);
/// assert_eq_test!(test_default, MyType::default().value(), 0);
/// ```
#[macro_export]
macro_rules! assert_eq_test {
    ($test_name:ident, $left:expr, $right:expr) => {
        #[test]
        fn $test_name() {
            assert_eq!($left, $right);
        }
    };
}

/// Generate debug format tests for multiple enum variants.
///
/// This macro creates a test for each variant that checks its Debug output.
///
/// # Examples
///
/// ```ignore
/// debug_format_tests! {
///     enum_type: MyEnum,
///     tests: [
///         (test_first_debug, MyEnum::First, "First"),
///         (test_second_debug, MyEnum::Second, "Second"),
///         (test_third_debug, MyEnum::Third, "Third"),
///     ]
/// }
/// ```
#[macro_export]
macro_rules! debug_format_tests {
    (
        enum_type: $enum_name:ty,
        tests: [
            $(($test_name:ident, $variant:expr, $expected:expr)),+ $(,)?
        ]
    ) => {
        $(
            #[test]
            fn $test_name() {
                let value = $variant;
                let debug = format!("{:?}", value);
                assert_eq!(debug, $expected);
            }
        )+
    };
}

/// Test that a conversion (From/Into) works correctly.
///
/// # Examples
///
/// ```ignore
/// conversion_test!(
///     test_alignment_to_ratatui,
///     TitleAlignment::Start,
///     Alignment,
///     Alignment::Left
/// );
/// ```
#[macro_export]
macro_rules! conversion_test {
    ($test_name:ident, $from:expr, $to_type:ty, $expected:expr) => {
        #[test]
        fn $test_name() {
            let result: $to_type = $from.into();
            assert_eq!(result, $expected);
        }
    };
}

/// Test that multiple enum variants can be instantiated.
///
/// Useful for compile-time verification that all variants are accessible.
///
/// # Examples
///
/// ```ignore
/// instantiate_variants_test!(
///     test_all_border_styles,
///     BorderStyle,
///     [Standard, Rounded, Dashed, CornerGapped]
/// );
/// ```
#[macro_export]
macro_rules! instantiate_variants_test {
    ($test_name:ident, $enum_name:ident, [$($variant:ident),+ $(,)?]) => {
        #[test]
        fn $test_name() {
            $(
                let _ = $enum_name::$variant;
            )+
        }
    };
}

/// Test that a method doesn't panic with a given input.
///
/// # Examples
///
/// ```ignore
/// no_panic_test!(test_apply_bold, {
///     let result = TitleStyle::Bold.apply("Test");
///     assert!(!result.is_empty());
/// });
/// ```
#[macro_export]
macro_rules! no_panic_test {
    ($test_name:ident, $body:block) => {
        #[test]
        fn $test_name() {
            $body
        }
    };
}

/// Test that a string transformation preserves certain properties.
///
/// # Examples
///
/// ```ignore
/// string_transform_test!(
///     test_bold_preserves_length,
///     TitleStyle::Bold.apply("Test"),
///     original: "Test",
///     length_preserved: true
/// );
/// ```
#[macro_export]
macro_rules! string_transform_test {
    (
        $test_name:ident,
        $transform:expr,
        original: $original:expr,
        length_preserved: $should_preserve:expr
    ) => {
        #[test]
        fn $test_name() {
            let original = $original;
            let result = $transform;
            if $should_preserve {
                assert_eq!(result.chars().count(), original.chars().count());
            }
        }
    };
}

/// Test that rendering to a buffer doesn't panic.
///
/// This macro creates a test that ensures a widget can be rendered without
/// panicking, which is useful for visual regression testing.
///
/// # Examples
///
/// ```ignore
/// use ratatui::buffer::Buffer;
/// use ratatui::layout::Rect;
///
/// render_test!(
///     test_piechart_renders,
///     PieChart::default(),
///     Rect::new(0, 0, 40, 20)
/// );
/// ```
#[macro_export]
macro_rules! render_test {
    ($test_name:ident, $widget:expr, $area:expr) => {
        #[test]
        fn $test_name() {
            use ratatui::buffer::Buffer;
            let mut buffer = Buffer::empty($area);
            ratatui::widgets::Widget::render($widget, buffer.area, &mut buffer);
        }
    };
}

/// Test that rendering with specific dimensions doesn't panic.
///
/// This is a convenience wrapper around `render_test` that creates the Rect for you.
///
/// # Examples
///
/// ```ignore
/// render_with_size_test!(
///     test_chart_small,
///     PieChart::default(),
///     width: 20,
///     height: 10
/// );
/// ```
#[macro_export]
macro_rules! render_with_size_test {
    (
        $test_name:ident,
        $widget:expr,
        width: $width:expr,
        height: $height:expr
    ) => {
        #[test]
        fn $test_name() {
            use ratatui::buffer::Buffer;
            use ratatui::layout::Rect;
            let area = Rect::new(0, 0, $width, $height);
            let mut buffer = Buffer::empty(area);
            ratatui::widgets::Widget::render($widget, buffer.area, &mut buffer);
        }
    };
}

/// Test rendering with multiple widget configurations.
///
/// Useful for testing that various configurations all render without panicking.
///
/// # Examples
///
/// ```ignore
/// multi_render_test!(test_pie_configurations, [
///     (PieChart::default(), Rect::new(0, 0, 20, 10)),
///     (PieChart::default().show_legend(false), Rect::new(0, 0, 30, 15)),
/// ]);
/// ```
#[macro_export]
macro_rules! multi_render_test {
    ($test_name:ident, [$(($widget:expr, $area:expr)),+ $(,)?]) => {
        #[test]
        fn $test_name() {
            use ratatui::buffer::Buffer;
            $(
                let mut buffer = Buffer::empty($area);
                ratatui::widgets::Widget::render($widget, buffer.area, &mut buffer);
            )+
        }
    };
}

/// Test that rendering to an empty area doesn't panic.
///
/// # Examples
///
/// ```ignore
/// render_empty_test!(test_chart_empty, PieChart::default());
/// ```
#[macro_export]
macro_rules! render_empty_test {
    ($test_name:ident, $widget:expr) => {
        #[test]
        fn $test_name() {
            use ratatui::buffer::Buffer;
            use ratatui::layout::Rect;
            let mut buffer = Buffer::empty(Rect::new(0, 0, 0, 0));
            ratatui::widgets::Widget::render($widget, buffer.area, &mut buffer);
        }
    };
}

#[cfg(test)]
#[allow(unnameable_test_items)]
mod tests {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    enum TestEnum {
        #[default]
        First,
        Second,
        Third,
    }

    // Test the macros themselves
    assert_test!(macro_assert_test_works, true);
    assert_eq_test!(macro_assert_eq_test_works, 2 + 2, 4);

    instantiate_variants_test!(test_enum_variants, TestEnum, [First, Second, Third]);

    debug_format_tests! {
        enum_type: TestEnum,
        tests: [
            (test_first_fmt, TestEnum::First, "First"),
            (test_second_fmt, TestEnum::Second, "Second"),
        ]
    }

    enum_tests! {
        enum_type: TestEnum,
        default_test: (test_enum_default, TestEnum::First),
        clone_test: (test_enum_clone, TestEnum::Second),
        debug_test: (test_enum_debug, TestEnum::Third, "Third"),
    }
}
