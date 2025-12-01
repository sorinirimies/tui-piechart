# VHS Demo GIFs

This directory contains VHS tape files for generating demo GIFs for all examples in the tui-piechart project.

## What is VHS?

[VHS](https://github.com/charmbracelet/vhs) is a tool for generating terminal GIFs. It allows you to write "tape" files that script terminal interactions, which are then recorded as GIFs.

## Installation

### macOS
```bash
brew install vhs
```

### Linux
See the [VHS installation guide](https://github.com/charmbracelet/vhs#installation)

### Verify Installation
```bash
vhs --version
```

## Available Tapes

This directory contains 12 VHS tape files, one for each example:

1. **piechart.tape** - Main interactive demo with 4 chart examples
2. **border_styles.tape** - All 11 border style variants
3. **legend_positioning.tape** - 4 legend positions × 2 layouts
4. **legend_alignment.tape** - 3 alignments × 4 positions × 2 layouts
5. **title_positioning.tape** - 2 vertical positions × 3 alignments
6. **title_styles.tape** - 10 Unicode font styles for titles
7. **custom_symbols.tape** - 8 custom Unicode symbol combinations
8. **high_resolution.tape** - Standard vs braille high-resolution rendering
9. **symbols_circles_squares.tape** - Circle and square symbol styles
10. **symbols_stars_hearts.tape** - Star and heart symbol styles
11. **symbols_triangles_hexagons.tape** - Triangle and hexagon symbol styles
12. **symbols_shades_bars.tape** - Shade and bar symbol styles

## Generating Demo GIFs

### Generate All Demos

The easiest way to generate all 12 demo GIFs at once:

```bash
# From project root
./examples/vhs/generate_all.sh
```

This script will:
- Check if VHS is installed
- Build all examples in release mode
- Generate all 12 demo GIFs
- Show a summary of generated files

### Generate Individual Demos

To generate a specific demo GIF:

```bash
# From project root
vhs examples/vhs/piechart.tape
vhs examples/vhs/border_styles.tape
vhs examples/vhs/legend_positioning.tape
vhs examples/vhs/legend_alignment.tape
vhs examples/vhs/title_positioning.tape
vhs examples/vhs/title_styles.tape
vhs examples/vhs/custom_symbols.tape
vhs examples/vhs/high_resolution.tape
# ... etc
```

### Using Just Commands

If you have [just](https://github.com/casey/just) installed:

```bash
# Generate all demos
just vhs-all

# Generate individual demos
just vhs-piechart
just vhs-border-styles
just vhs-legend-positioning
just vhs-legend-alignment
just vhs-title-positioning
just vhs-title-styles-example
just vhs-custom-symbols
just vhs-high-resolution
just vhs-symbols-circles-squares
just vhs-symbols-stars-hearts
just vhs-symbols-triangles-hexagons
just vhs-symbols-shades-bars
```

## Output Location

All generated GIF files are saved to:

```
examples/vhs/output/*.gif
```

## Tape File Format

VHS tape files use a simple scripting language. Here's an example structure:

```tape
Output "examples/vhs/output/example.gif"
Set Theme "Catppuccin Macchiato"
Set Width 1600
Set Height 1200
Set FontSize 16
Set Padding 20

Hide
Type "cargo run --example example_name"
Enter
Sleep 2s
Show

# Interact with the example
Down
Sleep 1s
Right
Sleep 1s

Hide
Escape
Sleep 0.3s
```

### Common Commands

- `Output` - Specify the output GIF file path
- `Set` - Configure terminal appearance (theme, size, font, padding)
- `Type` - Type text into the terminal
- `Enter` - Press the Enter key
- `Sleep` - Wait for a duration (e.g., `1s`, `500ms`)
- `Down`, `Up`, `Left`, `Right` - Arrow keys
- `Escape` - Press the Escape key
- `Hide` / `Show` - Hide/show terminal output during recording

## Customizing Tapes

When modifying tape files, keep these guidelines in mind:

1. **Build First**: Always build examples in release mode before recording:
   ```bash
   cargo build --examples --release
   ```

2. **Timing**: Adjust `Sleep` durations to give enough time for:
   - Application startup (usually 2s)
   - Rendering after interactions (1-1.5s for key moments, 0.5-0.8s for quick navigation)

3. **Terminal Size**: Use consistent dimensions across tapes:
   - Standard examples: 1400×1000 or 1600×1200
   - Border styles (4-row grid): 1600×1400

4. **Theme**: Use "Catppuccin Macchiato" for consistency across all demos
   - Beautiful pastel color scheme with excellent readability
   - Consistent with modern terminal aesthetics

5. **Navigation Pattern**: Show key features first, then demonstrate quick navigation

## File Sizes

Generated GIFs typically range from:
- Small demos: ~300-400 KB
- Medium demos: ~600-800 KB
- Large demos: ~2-3 MB

If GIFs are too large, consider:
- Reducing recording duration
- Decreasing terminal dimensions
- Optimizing navigation speed
- Using fewer color changes

## Troubleshooting

### VHS Not Found
```bash
# Install VHS first
brew install vhs  # macOS
# or see https://github.com/charmbracelet/vhs#installation
```

### Example Not Running
```bash
# Build examples first
cargo build --examples --release
```

### Recording Takes Too Long
- Examples need time to compile and start
- The `Hide` command at the beginning hides the compilation output
- First `Sleep 2s` after `Enter` gives the example time to start

### GIF Quality Issues
- Increase terminal dimensions (`Set Width` / `Set Height`)
- Increase font size (`Set FontSize`)
- Try different themes

## Publishing Demos

VHS offers a hosting service for GIFs:

```bash
vhs publish examples/vhs/output/piechart.gif
```

This will upload the GIF and provide a shareable URL.

## Contributing

When adding new examples to the project:

1. Create a corresponding `.tape` file in this directory
2. Add it to the `TAPES` array in `generate_all.sh`
3. Add a just command in `justfile` (e.g., `vhs-example-name`)
4. Update this README with the new tape description
5. Generate the GIF and verify it looks good
6. Update `EXAMPLES.md` to reference the new demo

## Resources

- [VHS GitHub Repository](https://github.com/charmbracelet/vhs)
- [VHS Documentation](https://github.com/charmbracelet/vhs#vhs)
- [VHS Syntax Reference](https://github.com/charmbracelet/vhs/blob/main/SYNTAX.md)

---

**Happy Recording! 📹✨**