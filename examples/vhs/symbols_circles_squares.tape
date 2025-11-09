# VHS Recording Script for tui-piechart Circles & Squares Symbols Demo
# This script demonstrates circle and square predefined symbols
# Install VHS: https://github.com/charmbracelet/vhs
# Run: vhs examples/symbols_circles_squares.tape

Output "examples/vhs/target/symbols_circles_squares.gif"
Set Theme "Aardvark Blue"
Set Width 1600
Set Height 1000
Set FontSize 16
Set Padding 20

Hide
Type "cargo run --example symbols_circles_squares"
Enter
Sleep 2s
Show
Sleep 1.5s

# Show default symbols (top-left)
Sleep 1s

# Navigate to Block (top-right)
Right
Sleep 1.5s

# Navigate to Circle (bottom-left)
Down
Left
Sleep 1.5s

# Navigate to Square (bottom-right)
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
