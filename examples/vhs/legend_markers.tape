# VHS Recording Script for tui-piechart Legend Markers Demo
# This script showcases all available legend marker symbols
# Install VHS: https://github.com/charmbracelet/vhs
# Run: vhs examples/vhs/legend_markers.tape

Output "examples/vhs/output/legend_markers.gif"
Set Theme "Catppuccin Macchiato"
Set Width 1600
Set Height 1200
Set FontSize 16
Set Padding 20

Hide
Type "cargo run --example legend_markers"
Enter
Sleep 2s
Show
Sleep 1s

# Show Default Square
Sleep 1.5s

# Navigate through markers - showing variety
Down
Sleep 1.2s

# Circle
Down
Sleep 1.2s

# Small Square
Down
Sleep 1.2s

# Arrow
Down
Sleep 1.2s

# Diamond
Down
Sleep 1.2s

# Star (popular choice)
Sleep 1.5s

# Continue through more markers
Down
Sleep 1s

# White Star
Down
Sleep 1s

# Small Circle
Down
Sleep 1s

# White Circle
Down
Sleep 1s

# Triangle
Down
Sleep 1s

# Heart
Sleep 1.2s

# White Heart
Down
Sleep 1.2s

# Plus
Down
Sleep 1s

# Cross
Down
Sleep 1s

# Check
Down
Sleep 1s

# Right Arrow
Down
Sleep 1s

# Double Right
Down
Sleep 1s

# Dash
Down
Sleep 1s

# Dot
Down
Sleep 1s

# Hexagon
Down
Sleep 1s

# Bullseye (interesting one)
Sleep 1.2s

# Square Box
Down
Sleep 1s

# Asterism
Down
Sleep 1s

# Horizontal Bar (last one)
Sleep 1.5s

# Navigate back up quickly to show navigation
Up
Sleep 0.5s
Up
Sleep 0.5s
Up
Sleep 0.5s

# Back to a popular choice (Star)
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

Sleep 1.5s

Sleep 1s

Hide
Escape
Sleep 0.3s
