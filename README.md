# tui-piechart

[![Crates.io](https://img.shields.io/crates/v/tui-piechart)](https://crates.io/crates/tui-piechart)
[![Documentation](https://docs.rs/tui-piechart/badge.svg)](https://docs.rs/tui-piechart)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Release](https://github.com/sorinirimies/tui-piechart/actions/workflows/release.yml/badge.svg)](https://github.com/sorinirimies/tui-piechart/actions/workflows/release.yml)
[![CI](https://github.com/sorinirimies/tui-piechart/actions/workflows/ci.yml/badge.svg)](https://github.com/sorinirimies/tui-piechart/actions/workflows/ci.yml)

A customizable pie chart widget for [Ratatui](https://github.com/ratatui/ratatui) TUI applications.

## Preview Standard and High Resolution
![tui-piechart](https://github.com/user-attachments/assets/1e142d27-67c6-4452-8562-4273776851e9)


## Features

- 🥧 Simple pie chart with customizable slices
- 🎨 Customizable colors for each slice
- 🔤 Labels and percentages
- 📊 Legend support
- 📦 Optional block wrapper
- ✨ Custom symbols for pie chart and legend
- 🔍 High resolution mode using braille patterns (8x resolution)
- ⚡ Zero-cost abstractions

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tui-piechart = "0.1.0"
ratatui = "0.29"
```

Or install with cargo:

```bash
cargo add tui-piechart
```

## Quick Start

```rust
use ratatui::style::Color;
use tui_piechart::{PieChart, PieSlice};

// Create slices
let slices = vec![
    PieSlice::new("Rust", 45.0, Color::Red),
    PieSlice::new("Go", 30.0, Color::Blue),
    PieSlice::new("Python", 25.0, Color::Green),
];

// Create pie chart
let piechart = PieChart::new(slices);

// With customization
let piechart = PieChart::new(slices)
    .show_legend(true)
    .show_percentages(true)
    .pie_char('●');

// With a block
use ratatui::widgets::Block;

let piechart = PieChart::new(slices)
    .block(Block::bordered().title("Statistics"));

// With high resolution mode (braille patterns for 8x resolution)
let piechart = PieChart::new(slices)
    .high_resolution(true);

// Or use the Resolution enum
use tui_piechart::Resolution;

let standard = PieChart::new(slices).resolution(Resolution::Standard);
let braille = PieChart::new(slices).resolution(Resolution::Braille);
```

## Examples

Run the included examples:

```bash
# Main interactive example (4 charts)
cargo run --example piechart

# Predefined symbols examples (4 charts each):
cargo run --example symbols_circles_squares      # Default, Block, Circle, Square
cargo run --example symbols_stars_hearts         # Diamond, Star, White Star, Heart
cargo run --example symbols_triangles_hexagons   # Triangle, Hexagon, Bullseye, Square Box
cargo run --example symbols_shades_bars          # Asterism, Horizontal Bar, Shade, Light

# Custom (non-predefined) symbols showcase (12 charts)
cargo run --example custom_symbols

# High resolution mode demo (animated, toggle with Space)
cargo run --example high_resolution
```

### Interactive Mode (Default)

Navigate through different chart types with your keyboard:
- **↑/↓** or **k/j** - Navigate between charts
- **Tab** - Switch to API Showcase mode
- **q** or **Esc** - Quit

The interactive mode demonstrates:
- Programming language distribution chart
- Market share visualization
- Time allocation pie chart
- Budget distribution chart

### API Showcase Mode

Press **Tab** to switch to the API showcase view, which displays:
- Basic usage (`new()`, `default()`, `slices()`)
- Styling options (`show_legend()`, `show_percentages()`, `pie_char()`)
- Custom symbols (circle, square, diamond, block)
- Multiple slices handling

## Customization

### Colors

Each slice can have its own color:

```rust
let slices = vec![
    PieSlice::new("Category A", 40.0, Color::Red),
    PieSlice::new("Category B", 35.0, Color::Blue),
    PieSlice::new("Category C", 25.0, Color::Green),
];
```

### Display Options

Control what information is shown:

```rust
let piechart = PieChart::new(slices)
    .show_legend(true)       // Show/hide legend
    .show_percentages(true); // Show/hide percentages in legend
```

### Custom Symbols

The pie chart widget allows full customization of symbols used for rendering. You can use **any Unicode character** for the pie chart and **any string** for legend markers.

#### Basic Usage

```rust
use tui_piechart::symbols;

// Using predefined symbols
let piechart = PieChart::new(slices)
    .pie_char(symbols::PIE_CHAR_BLOCK)              // █
    .legend_marker(symbols::LEGEND_MARKER_CIRCLE);  // ●

// Using custom characters
let piechart = PieChart::new(slices)
    .pie_char('★')       // Any Unicode character
    .legend_marker("→"); // Any string
```

## Predefined Symbols

The `symbols` module provides carefully selected characters that work well in most terminals:

### Pie Chart Characters

**Basic Shapes:**
- `symbols::PIE_CHAR` - ● (default, filled circle)
- `symbols::PIE_CHAR_BLOCK` - █ (solid block, high density)
- `symbols::PIE_CHAR_CIRCLE` - ◉ (circle with center dot)
- `symbols::PIE_CHAR_SQUARE` - ■ (solid square)
- `symbols::PIE_CHAR_DIAMOND` - ◆ (solid diamond)
- `symbols::PIE_CHAR_HEXAGON` - ⬢ (filled hexagon)
- `symbols::PIE_CHAR_BULLSEYE` - ◉ (bullseye circle)
- `symbols::PIE_CHAR_SQUARE_BOX` - ▣ (squared box)

**Shading Patterns:**
- `symbols::PIE_CHAR_SHADE` - ▒ (medium shade pattern)
- `symbols::PIE_CHAR_LIGHT` - ░ (light shade pattern)
- `symbols::PIE_CHAR_DARK` - ▓ (dark shade pattern)

**Circle Variations:**
- `symbols::PIE_CHAR_SMALL_CIRCLE` - • (small filled circle)
- `symbols::PIE_CHAR_WHITE_CIRCLE` - ○ (hollow circle)
- `symbols::PIE_CHAR_DOUBLE_CIRCLE` - ◎ (circle with ring)

**Square Variations:**
- `symbols::PIE_CHAR_SMALL_SQUARE` - ▪ (small filled square)
- `symbols::PIE_CHAR_WHITE_SQUARE` - □ (hollow square)

**Diamond Variations:**
- `symbols::PIE_CHAR_SMALL_DIAMOND` - ◆ (small filled diamond)
- `symbols::PIE_CHAR_WHITE_DIAMOND` - ◇ (hollow diamond)

**Stars:**
- `symbols::PIE_CHAR_STAR` - ★ (filled star)
- `symbols::PIE_CHAR_WHITE_STAR` - ☆ (hollow star)

**Triangles:**
- `symbols::PIE_CHAR_TRIANGLE_UP` - ▲ (triangle pointing up)
- `symbols::PIE_CHAR_TRIANGLE_DOWN` - ▼ (triangle pointing down)
- `symbols::PIE_CHAR_TRIANGLE_RIGHT` - ▶ (triangle pointing right)
- `symbols::PIE_CHAR_TRIANGLE_LEFT` - ◀ (triangle pointing left)

**Card Suits:**
- `symbols::PIE_CHAR_HEART` - ♥ (filled heart)
- `symbols::PIE_CHAR_WHITE_HEART` - ♡ (hollow heart)
- `symbols::PIE_CHAR_SPADE` - ♠ (spade)
- `symbols::PIE_CHAR_CLUB` - ♣ (club)

**Other:**
- `symbols::PIE_CHAR_PLUS` - ✚ (plus sign)
- `symbols::PIE_CHAR_CROSS` - ✖ (cross/multiply)
- `symbols::PIE_CHAR_DOT` - · (middle dot)
- `symbols::PIE_CHAR_ASTERISM` - ※ (reference mark)
- `symbols::PIE_CHAR_HORIZONTAL_BAR` - ▰ (horizontal bar)

### Legend Marker Symbols

**Basic Markers:**
- `symbols::LEGEND_MARKER` - ■ (default, solid square)
- `symbols::LEGEND_MARKER_CIRCLE` - ● (filled circle)
- `symbols::LEGEND_MARKER_SQUARE` - ▪ (small square)
- `symbols::LEGEND_MARKER_DIAMOND` - ◆ (solid diamond)
- `symbols::LEGEND_MARKER_ARROW` - ▶ (right-pointing triangle)

**Stars:**
- `symbols::LEGEND_MARKER_STAR` - ★ (filled star)
- `symbols::LEGEND_MARKER_WHITE_STAR` - ☆ (hollow star)

**Circles:**
- `symbols::LEGEND_MARKER_SMALL_CIRCLE` - • (small circle)
- `symbols::LEGEND_MARKER_WHITE_CIRCLE` - ○ (hollow circle)

**Shapes:**
- `symbols::LEGEND_MARKER_TRIANGLE` - ▲ (triangle)
- `symbols::LEGEND_MARKER_HEART` - ♥ (filled heart)
- `symbols::LEGEND_MARKER_WHITE_HEART` - ♡ (hollow heart)

**Symbols:**
- `symbols::LEGEND_MARKER_PLUS` - ✚ (plus sign)
- `symbols::LEGEND_MARKER_CROSS` - ✖ (cross)
- `symbols::LEGEND_MARKER_CHECK` - ✓ (checkmark)

**Arrows & Lines:**
- `symbols::LEGEND_MARKER_RIGHT_ARROW` - → (right arrow)
- `symbols::LEGEND_MARKER_DOUBLE_RIGHT` - » (double right)
- `symbols::LEGEND_MARKER_DASH` - – (dash)
- `symbols::LEGEND_MARKER_DOT` - · (dot)

**Special Shapes:**
- `symbols::LEGEND_MARKER_HEXAGON` - ⬡ (hollow hexagon)
- `symbols::LEGEND_MARKER_BULLSEYE` - ◉ (bullseye)
- `symbols::LEGEND_MARKER_SQUARE_BOX` - ▢ (hollow square box)
- `symbols::LEGEND_MARKER_ASTERISM` - ⁂ (asterism)
- `symbols::LEGEND_MARKER_HORIZONTAL_BAR` - ▱ (hollow horizontal bar)

## Custom Symbol Examples

### Theme-Based Combinations

```rust
// Professional/Corporate Theme
let piechart = PieChart::new(slices)
    .pie_char(symbols::PIE_CHAR_BLOCK)
    .legend_marker(symbols::LEGEND_MARKER);

// Minimal/Clean Theme
let piechart = PieChart::new(slices)
    .pie_char('·')
    .legend_marker("•");

// Geometric Theme
let piechart = PieChart::new(slices)
    .pie_char('◆')
    .legend_marker("◇");

// Playful Theme
let piechart = PieChart::new(slices)
    .pie_char('★')
    .legend_marker("☆");
```

### Custom Unicode Characters

You have access to the full Unicode character set:

```rust
// Arrows and triangles
let piechart = PieChart::new(slices)
    .pie_char('▲')
    .legend_marker("▶");

// Playing card suits
let piechart = PieChart::new(slices)
    .pie_char('♠')
    .legend_marker("♣");

// Decorative symbols
let piechart = PieChart::new(slices)
    .pie_char('◎')
    .legend_marker("✦");

// Emoji (if your terminal supports it)
let piechart = PieChart::new(slices)
    .pie_char('🔥')
    .legend_marker("🌟");
```

### Multi-Character Legend Markers

Legend markers can be multiple characters for unique styles:

```rust
// ASCII arrows
let piechart = PieChart::new(slices)
    .legend_marker("-->");

// Brackets
let piechart = PieChart::new(slices)
    .legend_marker("[ ]");

// Custom prefix
let piechart = PieChart::new(slices)
    .legend_marker("=> ");
```

### Interactive Examples

**Predefined Symbols Examples (4 charts each in 2x2 grid):**

```bash
# Circles and squares
cargo run --example symbols_circles_squares

# Stars and hearts
cargo run --example symbols_stars_hearts

# Triangles and hexagons
cargo run --example symbols_triangles_hexagons

# Shades and bars
cargo run --example symbols_shades_bars
```

Each example showcases 4 predefined symbol combinations:
- **symbols_circles_squares**: Default, Block, Circle, Square
- **symbols_stars_hearts**: Diamond, Star, White Star, Heart
- **symbols_triangles_hexagons**: Triangle, Hexagon, Bullseye, Square Box
- **symbols_shades_bars**: Asterism, Horizontal Bar, Shade, Light
- Navigate with arrow keys or hjkl

**Custom Symbols Example:**

```bash
cargo run --example custom_symbols
```

This example showcases truly custom Unicode characters NOT in the predefined list:
- 12 unique custom symbol combinations
- Hexagons, emoji, asterisms, and more
- Demonstrates the full flexibility of custom Unicode
- Navigate with ↑/↓ to compare styles
- Note: Some emoji may require specific terminal font support

### Best Practices

1. **Terminal Compatibility**: Not all terminals support all Unicode characters. Test your symbols in your target environment.

2. **Solid Characters**: Use solid, dense characters (●, █, ■) for best visibility in pie charts.

3. **Consistent Themes**: Match your pie character with your legend marker for visual consistency.

4. **Size Considerations**: For small charts (< 40 chars wide), use simpler characters like `●` or `█`.

5. **Testing**: Always test your custom symbols in the actual terminal where your application will run.

### High Resolution Mode

Enable high resolution rendering using Unicode braille patterns for **dramatically smoother** pie charts.

**Live animated demo:** The `high_resolution` example includes smooth animations that showcase the quality difference between standard and braille rendering in real-time.

```rust
let piechart = PieChart::new(slices)
    .high_resolution(true);

// Or use the Resolution enum
use tui_piechart::Resolution;

let standard = PieChart::new(slices).resolution(Resolution::Standard);
let braille = PieChart::new(slices).resolution(Resolution::Braille);
```

**Visual Comparison:**

Standard mode (1 dot per cell):
```
        ●●●●●●●●●
     ●●●●●●●●●●●●●●●
   ●●●●●●●●●●●●●●●●●●●
  ●●●●●●●●●●●●●●●●●●●●●
```

Braille mode (8 dots per cell):
```
      ⣀⣀⣀⣀⣀⣄⣀⣀⣀⣀⡀
   ⢀⣠⣴⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣤⣀
 ⣠⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣦⡀
⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷
```

**How it works:**
- Uses Unicode braille characters (U+2800-U+28FF)
- Each character cell contains **2×4 dots** (8 dots total)
- Provides **8x the resolution** compared to standard mode
- Results in **noticeably smoother circles** and crisp edges

**Example usage:**
```rust
// Standard resolution (blocky)
let standard = PieChart::new(slices);

// High resolution (smooth) - just add one method!
let high_res = PieChart::new(slices).high_resolution(true);
```

**Interactive animated demo:**
```bash
cargo run --example high_resolution
# Press Space/Enter/H to toggle between modes
# Values animate smoothly to showcase rendering quality
```

**Best for:**
- When visual quality matters most
- Presentations and demos where smoothness impresses
- Large terminal displays (>80 columns)
- Terminals with excellent Unicode support

**Note:** The difference is immediately visible - high-res creates smooth circles instead of blocky shapes!

## Generating Demo GIFs

If you have [VHS](https://github.com/charmbracelet/vhs) installed, you can generate demo GIFs for all examples:

```bash
# Main interactive demo
vhs examples/piechart.tape

# Predefined symbols examples
vhs examples/symbols_circles_squares.tape
vhs examples/symbols_stars_hearts.tape
vhs examples/symbols_triangles_hexagons.tape
vhs examples/symbols_shades_bars.tape

# Custom symbols demo
vhs examples/custom_symbols.tape

# Note: high_resolution example is interactive and best experienced live
```

## Development

### Prerequisites

- Rust 1.74.0 or later
- [just](https://github.com/casey/just) - command runner (optional)
- [git-cliff](https://github.com/orhun/git-cliff) - changelog generator (optional)

Install tools:

```bash
just install-tools
```

### Common Tasks

```bash
# Run example
just run

# Run tests
just test

# Format and lint
just fmt
just clippy

# Check all
just check-all

# Generate demo GIF (requires VHS)
just vhs

# Bump version
just bump 0.1.0
```

See all available commands:

```bash
just --list
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Acknowledgments

This widget was created for the [Ratatui](https://github.com/ratatui/ratatui) ecosystem.

Special thanks to the Ratatui team for creating such an amazing TUI framework.
