# VHS Recording Script for tui-piechart Shades & Bars Symbols Demo
# This script demonstrates shading patterns and bar predefined symbols
# Install VHS: https://github.com/charmbracelet/vhs
# Run: vhs examples/symbols_shades_bars.tape

Output "examples/vhs/output/symbols_shades_bars.gif"
Set Theme "Catppuccin Macchiato"
Set Width 1600
Set Height 1000
Set FontSize 16
Set Padding 20

Hide
Type "cargo run --example symbols_shades_bars"
Enter
Sleep 2s
Show
Sleep 1.5s

# Show asterism symbols (top-left)
Sleep 1s

# Navigate to Horizontal Bar (top-right)
Right
Sleep 1.5s

# Navigate to Shade (bottom-left)
Down
Left
Sleep 1.5s

# Navigate to Light (bottom-right)
Right
Sleep 1.5s

# Navigate back up
Up
Sleep 1s

# Navigate around the grid
Down
Sleep 0.8s
Right
Sleep 0.8s
Up
Sleep 0.8s
Left
Sleep 0.8s

# Return to start
Sleep 1s

# Final pause
Sleep 2s

Hide
Escape
Sleep 0.3s
