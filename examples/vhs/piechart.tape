# VHS Recording Script for tui-piechart Demo
# This script demonstrates the tui-piechart widget
# Install VHS: https://github.com/charmbracelet/vhs
# Run: vhs examples/piechart.tape

Output "examples/vhs/output/piechart.gif"
Set Theme "Catppuccin Macchiato"
Set Width 1400
Set Height 1000
Set FontSize 16
Set Padding 20

Hide
Type "cargo run --example piechart"
Enter
Sleep 2s
Show
Sleep 1s

# Show the first pie chart (Programming Languages)
Sleep 1s

# Navigate to Market Share
Down
Sleep 1s

# Navigate to Time Allocation
Down
Sleep 1s

# Navigate to Budget Distribution
Down
Sleep 1s

# Back to top
Up
Up
Up
Sleep 0.5s

# Switch to API Showcase
Tab
Sleep 2s

# Navigate through showcase
Sleep 2s

# Back to interactive
Tab
Sleep 1s

# Final navigation
Down
Sleep 0.5s
Down
Sleep 0.5s
Up
Sleep 0.5s

Sleep 1s

Hide
Escape
Sleep 0.3s
