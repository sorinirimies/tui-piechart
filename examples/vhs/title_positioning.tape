# VHS Recording Script for tui-piechart Title Positioning Demo
# This script demonstrates all available title positions and alignments
# Install VHS: https://github.com/charmbracelet/vhs
# Run: vhs examples/vhs/title_positioning.tape

Output "examples/vhs/output/title_positioning.gif"
Set Theme "Catppuccin Macchiato"
Set Width 1600
Set Height 1200
Set FontSize 16
Set Padding 20

Hide
Type "cargo run --example title_positioning"
Enter
Sleep 2s
Show
Sleep 1s

# Show the first position (Top - Start)
Sleep 1.5s

# Toggle to Center alignment
Right
Sleep 1.5s

# Toggle to End alignment
Right
Sleep 1.5s

# Toggle back to Start
Right
Sleep 1s

# Navigate to Bottom position
Down
Sleep 1.5s

# Toggle to Center alignment
Right
Sleep 1.5s

# Toggle to End alignment
Right
Sleep 1.5s

# Toggle back to Start
Right
Sleep 1s

# Back to Top position
Up
Sleep 1s

# Quick cycle through alignments
Right
Sleep 0.8s
Right
Sleep 0.8s
Right
Sleep 0.8s

# Show Bottom again
Down
Sleep 1s
Right
Sleep 1s

# Back to Top Center for finale
Up
Sleep 0.8s
Left
Left
Right
Sleep 1.5s

Sleep 1s

Hide
Escape
Sleep 0.3s
