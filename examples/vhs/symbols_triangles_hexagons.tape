# VHS Recording Script for tui-piechart Triangles & Hexagons Symbols Demo
# This script demonstrates triangle and hexagon predefined symbols
# Install VHS: https://github.com/charmbracelet/vhs
# Run: vhs examples/symbols_triangles_hexagons.tape

Output "examples/vhs/output/symbols_triangles_hexagons.gif"
Set Theme "Catppuccin Macchiato"
Set Width 1600
Set Height 1000
Set FontSize 16
Set Padding 20

Hide
Type "cargo run --example symbols_triangles_hexagons"
Enter
Sleep 2s
Show
Sleep 1.5s

# Show triangle symbols (top-left)
Sleep 1s

# Navigate to Hexagon (top-right)
Right
Sleep 1.5s

# Navigate to Bullseye (bottom-left)
Down
Left
Sleep 1.5s

# Navigate to Square Box (bottom-right)
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
