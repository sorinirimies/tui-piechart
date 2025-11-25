# VHS Recording Script for tui-piechart Title Styles & Positioning Demo
# This script demonstrates all title font styles and positioning options
# Install VHS: https://github.com/charmbracelet/vhs
# Run: vhs examples/vhs/title_styles.tape

Output "examples/vhs/output/title_styles.gif"
Set Theme "Catppuccin Macchiato"
Set Width 1600
Set Height 1200
Set FontSize 16
Set Padding 20

Hide
Type "cargo run --example title_styles"
Enter
Sleep 2s
Show
Sleep 1s

# Start in Styles mode - show Normal
Sleep 1.5s

# Navigate through different font styles
Down
Sleep 1.2s

# Bold
Down
Sleep 1.2s

# Italic
Down
Sleep 1.2s

# Bold Italic
Down
Sleep 1.2s

# Script (interesting one)
Sleep 1.5s

# Bold Script
Down
Sleep 1.2s

# Sans Serif
Down
Sleep 1s

# Bold Sans
Down
Sleep 1s

# Italic Sans
Down
Sleep 1s

# Monospace
Sleep 1.5s

# Go back to a popular style (Bold)
Up
Sleep 0.3s
Up
Sleep 0.3s
Up
Sleep 0.3s
Up
Sleep 0.3s
Up
Sleep 0.3s
Up
Sleep 0.3s
Up
Sleep 0.3s
Up

Sleep 1.2s

# Now show alignment changes
Left
Sleep 1s

# Start (Left)
Right
Sleep 1s

# Center
Right
Sleep 1s

# End (Right)
Sleep 1.2s

# Toggle position to Bottom
Space
Sleep 1.5s

# Show alignment changes at bottom
Left
Sleep 1s
Left
Sleep 1s

# Start (Left) at Bottom
Right
Sleep 1s

# Center at Bottom
Right
Sleep 1s

# End (Right) at Bottom
Sleep 1.2s

# Toggle back to Top
Space
Sleep 1.2s

# Switch to Positioning mode
Type "m"
Sleep 1.5s

# Show different alignments in Positioning mode
Left
Sleep 1s

# Start
Right
Sleep 1s

# Center
Right
Sleep 1s

# End
Sleep 1.2s

# Toggle position
Space
Sleep 1.2s

# Bottom
Space
Sleep 1.2s

# Back to Top
Left
Sleep 1s
Left
Sleep 1s

# Show Start at Top
Sleep 1s

# Switch back to Styles mode
Type "m"
Sleep 1.5s

# Navigate to Script style for final display
Down
Sleep 0.5s
Down
Sleep 0.5s
Down
Sleep 0.5s
Down

Sleep 1.5s

Sleep 1s

Hide
Escape
Sleep 0.3s
