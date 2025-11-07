# VHS Recording Script for tui-piechart Custom Symbols Demo
# This script demonstrates truly custom Unicode symbols not in the predefined list
# Install VHS: https://github.com/charmbracelet/vhs
# Run: vhs examples/custom_symbols.tape

Output "examples/custom_symbols.gif"
Set Theme "Aardvark Blue"
Set Width 1600
Set Height 1000
Set FontSize 16
Set Padding 20

Hide
Type "cargo run --example custom_symbols"
Enter
Sleep 2s
Show
Sleep 1.5s

# Show hexagons
Sleep 1s

# Navigate through custom symbols
Down
Sleep 1.5s

# Medium circles
Down
Sleep 1.5s

# Emoji (fire & star)
Down
Sleep 1.5s

# Asterisms
Down
Sleep 1.5s

# Large circle & ring
Down
Sleep 1.5s

# Square boxes
Down
Sleep 1.5s

# Pentagon & ring
Down
Sleep 1.5s

# Art emoji
Down
Sleep 1.5s

# Electric (lightning & star)
Down
Sleep 1.5s

# Bullseye & circle plus
Down
Sleep 1.5s

# Horizontal bars
Down
Sleep 1.5s

# Symbols (yin yang & peace)
Sleep 1.5s

# Go back to top
Type "Home"
Sleep 1s

# Navigate through a few more
Down
Sleep 0.8s
Down
Sleep 0.8s
Down
Sleep 0.8s
Down
Sleep 0.8s

# Jump to end to show emoji
Type "End"
Sleep 1.5s

# Back to middle
Up
Up
Up
Up
Sleep 0.5s

# Final pause
Sleep 2s

Hide
Escape
Sleep 0.3s
