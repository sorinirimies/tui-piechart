# VHS Recording Script for tui-piechart Legend Alignment Demo
# This script demonstrates all legend alignment options
# Install VHS: https://github.com/charmbracelet/vhs
# Run: vhs examples/vhs/legend_alignment.tape

Output "examples/vhs/output/legend_alignment.gif"
Set Theme "Catppuccin Macchiato"
Set Width 1600
Set Height 1200
Set FontSize 16
Set Padding 20

Hide
Type "cargo run --example legend_alignment"
Enter
Sleep 2s
Show
Sleep 1s

# Start with Left alignment, Right position, Vertical layout
Sleep 1.5s

# Change to Center alignment
Down
Sleep 1.5s

# Change to Right alignment
Down
Sleep 1.5s

# Back to Left
Up
Sleep 0.8s
Up
Sleep 0.8s

# Now explore different positions with Center alignment
Down
Sleep 0.5s

# Move to Left position
Right
Sleep 1.5s

# Move to Top position
Right
Sleep 1.5s

# Move to Bottom position
Right
Sleep 1.5s

# Toggle to Horizontal layout
Space
Sleep 1.5s

# Cycle through alignments with Horizontal layout
Down
Sleep 1.2s
Down
Sleep 1.2s
Up
Sleep 0.8s

# Move through positions with Center + Horizontal
Left
Sleep 1.2s
Left
Sleep 1.2s
Left
Sleep 1.2s

# Toggle back to Vertical
Space
Sleep 1.5s

# Show Right alignment
Down
Sleep 1.2s
Down
Sleep 1.2s

# Quick position changes with Right alignment
Right
Sleep 1s
Right
Sleep 1s
Right
Sleep 1s

# Back to center for final view
Up
Sleep 1s
Left
Sleep 1.5s

Sleep 1s

Hide
Escape
Sleep 0.3s
