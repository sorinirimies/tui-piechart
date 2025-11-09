# VHS Recording Script for tui-piechart Stars & Hearts Symbols Demo
# This script demonstrates star and heart predefined symbols
# Install VHS: https://github.com/charmbracelet/vhs
# Run: vhs examples/symbols_stars_hearts.tape

Output "examples/vhs/target/symbols_stars_hearts.gif"
Set Theme "Aardvark Blue"
Set Width 1600
Set Height 1000
Set FontSize 16
Set Padding 20

Hide
Type "cargo run --example symbols_stars_hearts"
Enter
Sleep 2s
Show
Sleep 1.5s

# Show diamond symbols (top-left)
Sleep 1s

# Navigate to Star (top-right)
Right
Sleep 1.5s

# Navigate to White Star (bottom-left)
Down
Left
Sleep 1.5s

# Navigate to Heart (bottom-right)
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
