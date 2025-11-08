# Examples Guide

This project includes seven interactive examples demonstrating different aspects of the tui-piechart widget. Most examples follow a consistent 4-chart layout in a 2x2 grid for easy comparison.

## Example Structure

Each example displays **4 pie charts** in a **2x2 grid layout**, allowing you to compare different symbol styles side-by-side. Navigate between charts using arrow keys or hjkl vim-style navigation.

## 1. Main Interactive Demo

**File:** `examples/piechart.rs`

**Run:** `cargo run --example piechart`

**Features:**
- 4 different pie chart examples showcasing various use cases
- Interactive mode with arrow key navigation
- Press Tab to switch to API Showcase mode
- Demonstrates:
  - Programming language distribution
  - Market share visualization
  - Time allocation pie chart
  - Budget distribution

**VHS Tape:** `examples/piechart.tape`

**Generate GIF:** `vhs examples/piechart.tape`

---

## Predefined Symbols Examples

Four focused examples showcasing predefined symbols from the `symbols` module. Each example shows 4 related symbol combinations in a 2x2 grid.

### 2. Circles & Squares

**File:** `examples/symbols_circles_squares.rs`

**Run:** `cargo run --example symbols_circles_squares`

**Symbols Shown:**
- **Default** (● / ■) - Standard filled circle and square
- **Block** (█ / ■) - Solid block character
- **Circle** (◉ / ●) - Circle with center dot and filled circle
- **Square** (■ / ▪) - Large square and small square

**Description:** Circle and square symbol combinations suitable for most use cases.

**VHS Tape:** `examples/symbols_circles_squares.tape`

**Generate GIF:** `vhs examples/symbols_circles_squares.tape`

### 3. Stars & Hearts

**File:** `examples/symbols_stars_hearts.rs`

**Run:** `cargo run --example symbols_stars_hearts`

**Symbols Shown:**
- **Diamond** (◆ / ◆) - Solid diamond shapes
- **Star** (★ / ★) - Filled star symbols
- **White Star** (☆ / ☆) - Hollow star outlines
- **Heart** (♥ / ♥) - Heart symbols for emotional data

**Description:** Star and heart symbols perfect for adding personality to your charts.

**VHS Tape:** `examples/symbols_stars_hearts.tape`

**Generate GIF:** `vhs examples/symbols_stars_hearts.tape`

### 4. Triangles & Hexagons

**File:** `examples/symbols_triangles_hexagons.rs`

**Run:** `cargo run --example symbols_triangles_hexagons`

**Symbols Shown:**
- **Triangle** (▲ / ▲) - Upward pointing triangles
- **Hexagon** (⬢ / ⬡) - Filled and hollow hexagons
- **Bullseye** (◉ / ◉) - Circle with center dot (target-like)
- **Square Box** (▣ / ▢) - Squared boxes with and without fill

**Description:** Triangle and hexagon symbols for technical or scientific data.

**VHS Tape:** `examples/symbols_triangles_hexagons.tape`

**Generate GIF:** `vhs examples/symbols_triangles_hexagons.tape`

### 5. Shades & Bars

**File:** `examples/symbols_shades_bars.rs`

**Run:** `cargo run --example symbols_shades_bars`

**Symbols Shown:**
- **Asterism** (※ / ⁂) - Reference marks and asterisms
- **Horizontal Bar** (▰ / ▱) - Filled and hollow horizontal bars
- **Shade** (▒ / ■) - Medium shade pattern
- **Light** (░ / ■) - Light shade pattern

**Description:** Shading patterns and bar symbols for unique visualizations.

**VHS Tape:** `examples/symbols_shades_bars.tape`

**Generate GIF:** `vhs examples/symbols_shades_bars.tape`

---

## 6. Custom Symbols Demo

**File:** `examples/custom_symbols.rs`

**Run:** `cargo run --example custom_symbols`

**Features:**
- Showcases 12 truly custom Unicode symbols NOT in the predefined list
- 4x3 grid layout showing:
  - Medium Circles (⚫ / ⚪)
  - Emoji (🔥 / 🌟)
  - Large Circle & Ring (⬤ / ○)
  - Pentagon & Ring (⬟ / ◯)
  - Art Emoji (🎯 / 🎨)
  - Electric (⚡ / ⭐)
  - Circle Operators (⊕ / ⊗)
  - Symbols (☯ / ☮)
  - Science (⚛ / ☢)
  - Music (♫ / ♬)
  - Nature (☘ / ❀)
  - Tools (⚙ / ⚒)
- Demonstrates the full flexibility of custom Unicode characters
- Note: Some emoji may require specific terminal font support

**VHS Tape:** `examples/custom_symbols.tape`

**Generate GIF:** `vhs examples/custom_symbols.tape`

---

## 7. High Resolution Demo

**File:** `examples/high_resolution.rs`

**Run:** `cargo run --example high_resolution`

**Features:**
- Interactive toggle between standard and high resolution modes
- **Animated values** that change smoothly over time
- 4 different pie charts (2x2 grid)
- Press Space/Enter/H to toggle resolution
- Demonstrates:
  - Programming languages distribution (animated)
  - Market share visualization (animated)
  - Time allocation pie chart (animated)
  - Budget distribution (animated)
- Uses Unicode braille patterns (U+2800-U+28FF) for 8x resolution
- Runs at ~60 FPS for smooth animations

**Description:** 
High resolution mode uses braille characters which provide 8 dots per character cell (2×4 grid), giving 8x the resolution for smoother circles and better edge definition. The example includes smooth animations that continuously change the pie chart values using sine waves, making the quality difference between standard and braille rendering immediately apparent. The animations showcase how braille patterns create noticeably smoother, more fluid transitions.

**VHS Tape:** Available - demonstrates toggling between standard and high resolution modes

---

## Key Differences

### Predefined vs Custom

**Predefined Symbols Examples (symbols_circles_squares, symbols_stars_hearts, etc.):**
- Use `symbols::PIE_CHAR_*` and `symbols::LEGEND_MARKER_*` constants
- All symbols are from the library's `symbols` module
- Guaranteed to work in most terminals
- Best for standard, professional use cases
- 4 charts per example for focused comparison

**Custom Symbols Example:**
- Uses raw Unicode characters directly
- Shows symbols that are NOT in the predefined list
- Demonstrates unlimited customization potential
- May require specific terminal/font support for emoji
- 12 charts showing diverse possibilities

---

## Navigation Controls

All examples support consistent navigation:

- **↑↓←→** or **hjkl** - Navigate between charts (vim-style)
- **Home** - Jump to first chart (where available)
- **End** - Jump to last chart (where available)
- **Tab** - Switch modes (piechart.rs only)
- **q** or **Esc** - Quit

---

## Example Selection Guide

**Choose the right example for your needs:**

| Goal | Example to Run |
|------|----------------|
| Learn the basics | `piechart` |
| Circles & squares | `symbols_circles_squares` |
| Stars & hearts | `symbols_stars_hearts` |
| Triangles & hexagons | `symbols_triangles_hexagons` |
| Shades & bars | `symbols_shades_bars` |
| Maximum customization | `custom_symbols` |
| High resolution rendering | `high_resolution` |

---

## Generating Demo GIFs

If you have [VHS](https://github.com/charmbracelet/vhs) installed:

```bash
# Generate all demo GIFs
vhs examples/piechart.tape
vhs examples/symbols_circles_squares.tape
vhs examples/symbols_stars_hearts.tape
vhs examples/symbols_triangles_hexagons.tape
vhs examples/symbols_shades_bars.tape
vhs examples/custom_symbols.tape
```

All examples now have VHS tapes for generating demo GIFs!

---

## Quick Start

```bash
# See everything quickly
cargo run --example piechart                     # Main features
cargo run --example symbols_circles_squares      # Circles & squares
cargo run --example custom_symbols               # Custom possibilities
cargo run --example high_resolution              # High-res mode (toggle with Space)

# Explore specific symbol categories
cargo run --example symbols_stars_hearts         # Stars and hearts
cargo run --example symbols_triangles_hexagons   # Triangles & hexagons
cargo run --example symbols_shades_bars          # Shades & bars
```

---

## Summary

- **7 total examples**
- **6 examples** use 2x2 grid (4 charts)
- **1 example** uses 4x3 grid (12 charts)
- **16 predefined symbol combinations** across 4 examples
- **12 custom symbol examples** in 1 example
- **1 high resolution demo** with interactive toggle
- **Consistent navigation** across all examples
- **Progressive learning** from basic to advanced