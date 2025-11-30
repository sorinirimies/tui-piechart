# Examples Guide

This project includes eleven interactive examples demonstrating different aspects of the tui-piechart widget. Most examples follow a consistent layout in a grid for easy comparison.

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

**VHS Tape:** `examples/vhs/piechart.tape`

**Generate GIF:** `vhs examples/vhs/piechart.tape`

![Main Interactive Demo](examples/vhs/target/piechart.gif)

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

**VHS Tape:** `examples/vhs/symbols_circles_squares.tape`

**Generate GIF:** `vhs examples/vhs/symbols_circles_squares.tape`

![Circles & Squares Demo](examples/vhs/target/symbols_circles_squares.gif)

### 3. Stars & Hearts

**File:** `examples/symbols_stars_hearts.rs`

**Run:** `cargo run --example symbols_stars_hearts`

**Symbols Shown:**
- **Diamond** (◆ / ◆) - Solid diamond shapes
- **Star** (★ / ★) - Filled star symbols
- **White Star** (☆ / ☆) - Hollow star outlines
- **Heart** (♥ / ♥) - Heart symbols for emotional data

**Description:** Star and heart symbols perfect for adding personality to your charts.

**VHS Tape:** `examples/vhs/symbols_stars_hearts.tape`

**Generate GIF:** `vhs examples/vhs/symbols_stars_hearts.tape`

![Stars & Hearts Demo](examples/vhs/target/symbols_stars_hearts.gif)

### 4. Triangles & Hexagons

**File:** `examples/symbols_triangles_hexagons.rs`

**Run:** `cargo run --example symbols_triangles_hexagons`

**Symbols Shown:**
- **Triangle** (▲ / ▲) - Upward pointing triangles
- **Hexagon** (⬢ / ⬡) - Filled and hollow hexagons
- **Bullseye** (◉ / ◉) - Circle with center dot (target-like)
- **Square Box** (▣ / ▢) - Squared boxes with and without fill

**Description:** Triangle and hexagon symbols for technical or scientific data.

**VHS Tape:** `examples/vhs/symbols_triangles_hexagons.tape`

**Generate GIF:** `vhs examples/vhs/symbols_triangles_hexagons.tape`

![Triangles & Hexagons Demo](examples/vhs/target/symbols_triangles_hexagons.gif)

### 5. Shades & Bars

**File:** `examples/symbols_shades_bars.rs`

**Run:** `cargo run --example symbols_shades_bars`

**Symbols Shown:**
- **Asterism** (※ / ⁂) - Reference marks and asterisms
- **Horizontal Bar** (▰ / ▱) - Filled and hollow horizontal bars
- **Shade** (▒ / ■) - Medium shade pattern
- **Light** (░ / ■) - Light shade pattern

**Description:** Shading patterns and bar symbols for unique visualizations.

**VHS Tape:** `examples/vhs/symbols_shades_bars.tape`

**Generate GIF:** `vhs examples/vhs/symbols_shades_bars.tape`

![Shades & Bars Demo](examples/vhs/target/symbols_shades_bars.gif)

---

## 6. Legend Positioning Demo

**File:** `examples/legend_positioning.rs`

**Run:** `cargo run --example legend_positioning`

**Features:**
This example demonstrates all legend positioning and layout options:
- **4 Legend Positions:**
  - Right (default)
  - Left
  - Top
  - Bottom
- **2 Legend Layouts:**
  - Vertical (default) - Items stacked in a column
  - Horizontal - Items in a single row
- Interactive controls:
  - ↑/↓ or k/j - Navigate between positions
  - ←/→ or h/l - Toggle between Vertical and Horizontal layouts
  - Shows all 8 combinations (4 positions × 2 layouts)
- Smart space calculation for horizontal legends to prevent cutoff
- Dynamic legend sizing based on content and layout

**Description:** Demonstrates how to use `LegendPosition` and `LegendLayout` enums to customize where and how the legend appears relative to the pie chart. Perfect for understanding how different positions affect the overall layout and readability.

**VHS Tape:** `examples/vhs/legend_positioning.tape`

**Generate GIF:** `vhs examples/vhs/legend_positioning.tape`

![Legend Positioning Demo](examples/vhs/output/legend_positioning.gif)

---

## 7. Legend Alignment Demo

**File:** `examples/legend_alignment.rs`

**Run:** `cargo run --example legend_alignment`

**Features:**
This example demonstrates legend alignment options for better visual balance:
- **3 Alignment Options:**
  - Left (default) - Items align to the left edge
  - Center - Items are centered within the legend area
  - Right - Items align to the right edge
- **Combined with:**
  - 4 Legend positions (Right, Left, Top, Bottom)
  - 2 Legend layouts (Vertical, Horizontal)
- Interactive controls:
  - ↑/↓ or k/j - Change alignment
  - ←/→ or h/l - Change position
  - Space - Toggle layout
  - Shows all 24 combinations (3 alignments × 4 positions × 2 layouts)
- Particularly useful for grid layouts and small chart areas

**Description:** Demonstrates the new `LegendAlignment` feature that controls how legend items are aligned within their allocated space. This is especially useful when working with grid layouts or varying chart sizes, as it allows you to create visually balanced and aesthetically pleasing legends. Center alignment works particularly well in tight spaces like the custom symbols grid example.

---

## 8. Title Positioning Demo

**File:** `examples/title_positioning.rs`

**Run:** `cargo run --example title_positioning`

**Features:**
This example demonstrates all title positioning and alignment options:
- **2 Vertical Positions:**
  - Top (default)
  - Bottom
- **3 Horizontal Alignments:**
  - Start (Left in LTR)
  - Center
  - End (Right in LTR)
- Interactive controls:
  - ↑/↓ or k/j - Toggle between Top and Bottom positions
  - ←/→ or h/l - Cycle through Start/Center/End alignments
  - Shows all 6 combinations (2 positions × 3 alignments)
- Uses `BlockExt` trait for ergonomic title customization
- Applies to the block wrapper around the chart

**Description:** Demonstrates how to use `TitleAlignment` and `TitlePosition` with the `BlockExt` trait to customize block title placement. Shows how titles can be positioned at the top or bottom with left, center, or right alignment.

**VHS Tape:** `examples/vhs/title_positioning.tape`

**Generate GIF:** `vhs examples/vhs/title_positioning.tape`

![Title Positioning Demo](examples/vhs/output/title_positioning.gif)

---

## 9. Title Styles Demo

**File:** `examples/title_styles_example.rs`

**Run:** `cargo run --example title_styles_example`

**Features:**
This example demonstrates all available title font styles using Unicode character variants:
- **10 Font Styles:**
  - Normal (default, no transformation)
  - Bold (Mathematical Bold Unicode)
  - Italic (Mathematical Italic Unicode)
  - Bold Italic (Combined bold and italic)
  - Script (Cursive/flowing script style)
  - Bold Script (Bold weight script)
  - Sans Serif (Clean sans-serif)
  - Bold Sans Serif (Bold sans-serif)
  - Italic Sans Serif (Italic sans-serif)
  - Monospace (Fixed-width monospace)
- Interactive controls:
  - ↑/↓ or k/j - Navigate between different font styles
  - Home/End - Jump to first/last style
  - Shows live preview of all ASCII letters, numbers, and styled titles
- Each style converts ASCII text to Unicode character equivalents
- Character preview shows the full alphabet in each style

**Description:** Demonstrates how to use `TitleStyle` enum to apply different Unicode font styles to block titles. Each style uses specific Unicode code points (Mathematical Alphanumeric Symbols) to achieve visual typography effects in terminal interfaces without requiring special fonts. Perfect for adding visual hierarchy and emphasis to chart titles.

**VHS Tape:** `examples/vhs/title_styles.tape`

**Generate GIF:** `vhs examples/vhs/title_styles.tape`

![Title Styles Demo](examples/vhs/output/title_styles.gif)

**Note:** Font rendering depends on terminal and font support. Most modern terminals support these Unicode ranges, but appearance may vary.

---

## 10. Border Styles Demo

**File:** `examples/border_styles.rs`

**Run:** `cargo run --example border_styles`

**Features:**
This example showcases all 11 available border styles for PieChart blocks:
- **Standard** - Default single-line borders
- **Rounded** - Single-line with rounded corners
- **Dashed** - Dashed lines throughout (┄┄┄)
- **Rounded Dashed** - Rounded corners with dashed lines
- **Corner Gapped** - Continuous lines with gaps only at corners
- **Rounded Corner Gapped** - Rounded with gaps only at corners
- **Double Line** - Double-line borders (═══)
- **Double Rounded** - Mixed style: rounded corners with double-line edges*
- **Thick** - Heavy/thick line borders (━━━)
- **Thick Rounded** - Mixed style: rounded corners with thick-line edges*
- **Thick Dashed** - Thick dashed lines (┅┅┅)
- Navigate with ↑/↓ to compare border styles
- 4-row grid layout for easy comparison
- Interactive navigation between border styles
- Uses `BorderStyle` enum for easy block customization

**Description:** Demonstrates how to use predefined border styles to customize the appearance of the PieChart wrapper block. Each style shows the same data with different border aesthetics.

**Note:** *DoubleLineRounded and ThickRounded use mixed styles (single-line rounded corners with double/thick-line edges) because Unicode doesn't have true rounded double-line or thick-line box-drawing characters.

---

## 11. Custom Symbols Demo

**File:** `examples/custom_symbols.rs`

**Run:** `cargo run --example custom_symbols`

**Features:**
- Showcases 8 truly custom Unicode symbols NOT in the predefined list
- 4x2 grid layout showing:
  - Large Circle & Ring (⬤ / ○)
  - Pentagon & Ring (⬟ / ◯)
  - Circle Operators (⊕ / ⊗)
  - Symbols (☯ / ☮)
  - Science (⚛ / ☢)
  - Music (♫ / ♬)
  - Nature (☘ / ❀)
  - Tools (⚙ / ⚒)
- Demonstrates the full flexibility of custom Unicode characters
- Note: Some emoji may require specific terminal font support

**VHS Tape:** `examples/vhs/custom_symbols.tape`

**Generate GIF:** `vhs examples/vhs/custom_symbols.tape`

![Custom Symbols Demo](examples/vhs/target/custom_symbols.gif)

---

## 12. High Resolution Demo

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

**VHS Tape:** `examples/vhs/high_resolution.tape`

**Generate GIF:** `vhs examples/vhs/high_resolution.tape`

![High Resolution Demo](examples/vhs/target/high_resolution.gif)

**Note:** Demonstrates toggling between standard and high resolution modes with smooth animations

---

## Feature Categories

### Border Styles

**Border Styles:**
- Customize the **wrapper block** appearance
- Use `BorderStyle` enum with predefined styles
- Applied via `.block(BorderStyle::Rounded.block())`
- Affects the frame around the entire chart
- 11 different border styles available (single-line, double-line, thick, dashed, and corner-gapped variants)

### Symbols

**Symbols:**
- Customize the **pie chart content** and **legend markers**
- Use character constants or raw Unicode
- Applied via `.pie_char()` and `.legend_marker()`
- Affects the chart data visualization itself
- Unlimited customization potential

### Legend & Title Positioning

**Legend Positioning:**
- Control where the legend appears relative to the chart
- Use `LegendPosition` enum (Right, Left, Top, Bottom)
- Choose between Vertical or Horizontal layout
- Applied via `.legend_position()` and `.legend_layout()`
- Affects the overall chart layout and space allocation

**Legend Alignment:**
- Control how legend items are aligned within the legend area
- Use `LegendAlignment` enum (Left, Center, Right)
- Applied via `.legend_alignment()`
- Particularly useful for grid layouts and small chart areas
- Center alignment creates balanced, symmetric layouts

**Title Positioning & Styling:**
- Control block title alignment and position
- Use `TitleAlignment` (Start, Center, End) and `TitlePosition` (Top, Bottom)
- Apply Unicode font styles with `TitleStyle` (Bold, Italic, Script, etc.)
- Applied via `BlockExt` trait methods and `TitleStyle::apply()`
- Affects the wrapper block's title appearance
- 6 positioning combinations and 10 font styles available

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
| Legend positioning | `legend_positioning` |
| Legend alignment | `legend_alignment` |
| Title positioning | `title_positioning` |
| Title font styles | `title_styles_example` |
| Border customization | `border_styles` |
| Maximum customization | `custom_symbols` |
| High resolution rendering | `high_resolution` |

---

## Generating Demo GIFs

If you have [VHS](https://github.com/charmbracelet/vhs) installed:

```bash
# Generate all demo GIFs
vhs examples/vhs/piechart.tape
vhs examples/vhs/symbols_circles_squares.tape
vhs examples/vhs/symbols_stars_hearts.tape
vhs examples/vhs/symbols_triangles_hexagons.tape
vhs examples/vhs/symbols_shades_bars.tape
vhs examples/vhs/legend_positioning.tape
vhs examples/vhs/title_positioning.tape
vhs examples/vhs/title_styles.tape
vhs examples/vhs/border_styles.tape
vhs examples/vhs/custom_symbols.tape
vhs examples/vhs/high_resolution.tape

# Or generate all at once
./examples/vhs/generate_all.sh
```

All examples now have VHS tapes for generating demo GIFs!

---

## Quick Start

```bash
# See everything quickly
cargo run --example piechart                     # Main features
cargo run --example symbols_circles_squares      # Circles & squares
cargo run --example legend_positioning           # Legend placement
cargo run --example legend_alignment             # Legend alignment
cargo run --example title_positioning            # Title alignment
cargo run --example title_styles_example         # Title font styles
cargo run --example border_styles                # Border customization
cargo run --example custom_symbols               # Custom possibilities
cargo run --example high_resolution              # High-res mode (toggle with Space)

# Explore specific symbol categories
cargo run --example symbols_stars_hearts         # Stars and hearts
cargo run --example symbols_triangles_hexagons   # Triangles & hexagons
cargo run --example symbols_shades_bars          # Shades & bars
```

---

## Summary

- **11 total examples**
- **6 examples** use 2x2 grid (4 charts)
- **3 positioning/styling examples** with interactive controls
- **1 example** uses 4-row grid (11 border styles)
- **1 example** uses 4x2 grid (8 charts)
- **16 predefined symbol combinations** across 4 examples
- **8 legend positioning combinations** (4 positions × 2 layouts)
- **24 legend alignment combinations** (3 alignments × 4 positions × 2 layouts)
- **6 title positioning combinations** (2 positions × 3 alignments)
- **10 title font styles** using Unicode character variants
- **11 border style examples** in 1 example
- **8 custom symbol examples** in 1 example (with centered legend alignment)
- **1 high resolution demo** with interactive toggle
- **All examples have VHS tapes** for generating demo GIFs
- **Consistent navigation** across all examples
- **Progressive learning** from basic to advanced