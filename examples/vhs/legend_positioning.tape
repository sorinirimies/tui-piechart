# VHS Recording Script for tui-piechart Legend Positioning Demo
# This script demonstrates all available legend positions and layouts
# Install VHS: https://github.com/charmbracelet/vhs
# Run: vhs examples/vhs/legend_positioning.tape

Output "examples/vhs/output/legend_positioning.gif"
Set Theme "Catppuccin Macchiato"
Set Width 1600
Set Height 1200
Set FontSize 16
Set Padding 20

Hide
Type "cargo run --example legend_positioning"
Enter
Sleep 2s
Show
Sleep 1s

# Show the first position (Right - Vertical)
Sleep 1.5s

# Toggle to Horizontal layout
Right
Sleep 1.5s

# Toggle back to Vertical
Left
Sleep 1s

# Navigate to Left position
Down
Sleep 1.5s

# Toggle to Horizontal layout
Right
Sleep 1.5s

# Toggle back to Vertical
Left
Sleep 1s

# Navigate to Top position
Down
Sleep 1.5s

# Toggle to Horizontal layout
Right
Sleep 1.5s

# Toggle back to Vertical
Left
Sleep 1s

# Navigate to Bottom position
Down
Sleep 1.5s

# Toggle to Horizontal layout
Right
Sleep 1.5s

# Toggle back to Vertical
Left
Sleep 1s

# Quick navigation back through positions
Up
Sleep 0.8s
Up
Sleep 0.8s
Up
Sleep 0.8s

# Show Right with Horizontal one more time
Right
Sleep 1.5s

Sleep 1s

Hide
Escape
Sleep 0.3s
