# VHS Recording Script for tui-piechart High Resolution Demo
# This script demonstrates the animated high resolution mode with smooth value transitions
# Install VHS: https://github.com/charmbracelet/vhs
# Run: vhs examples/high_resolution.tape

Output "examples/vhs/output/high_resolution.gif"
Set Theme "Catppuccin Macchiato"
Set Width 1600
Set Height 1000
Set FontSize 16
Set Padding 20

Hide
Type "cargo run --example high_resolution"
Enter
Sleep 2s
Show

# Start in Standard resolution mode - let animation play
Sleep 4s

# Toggle to High resolution (Braille) to see the smooth rendering
Space
Sleep 4s

# Toggle back to Standard to show the difference
Space
Sleep 3s

# Final toggle to High resolution for showcase of smooth animation
Space
Sleep 5s

Hide
Type "q"
Sleep 0.3s
