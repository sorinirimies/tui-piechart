# VHS Recording Script for tui-piechart Border Styles Demo
# This script demonstrates all available border styles for PieChart blocks
# Install VHS: https://github.com/charmbracelet/vhs
# Run: vhs examples/vhs/border_styles.tape

Output "examples/vhs/output/border_styles.gif"
Set Theme "Catppuccin Macchiato"
Set Width 1600
Set Height 1400
Set FontSize 16
Set Padding 20

Hide
Type "cargo run --example border_styles"
Enter
Sleep 2s
Show
Sleep 1s

# Show the first border style (Standard)
Sleep 1.5s

# Navigate to Rounded
Down
Sleep 1.5s

# Navigate to Dashed
Down
Sleep 1.5s

# Navigate to CornerGapped
Down
Sleep 1.5s

# Navigate to RoundedDashed
Down
Sleep 1.5s

# Navigate to RoundedCornerGapped
Down
Sleep 1.5s

# Navigate to DoubleLine
Down
Sleep 1.5s

# Navigate to DoubleLineRounded
Down
Sleep 1.5s

# Navigate to Thick
Down
Sleep 1.5s

# Navigate to ThickRounded
Down
Sleep 1.5s

# Navigate to ThickDashed
Down
Sleep 1.5s

# Back to top to show standard again
Up
Up
Up
Up
Up
Up
Up
Up
Up
Up
Sleep 1.5s

# Quick navigation through some interesting styles
Down
Down
Sleep 1s
Down
Down
Sleep 1s
Down
Down
Sleep 1s

# Show a rounded style
Up
Up
Up
Up
Sleep 1.5s

Sleep 1s

Hide
Escape
Sleep 0.3s
