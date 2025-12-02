# VHS Recording Script for tui-piechart Custom Symbols Demo
# This script demonstrates truly custom Unicode symbols not in the predefined list
# Install VHS: https://github.com/charmbracelet/vhs
# Run: vhs examples/custom_symbols.tape

Output "examples/vhs/output/custom_symbols.gif"
Set Theme "Catppuccin Macchiato"
Set Width 1600
Set Height 1200
Set FontSize 16
Set Padding 20

Hide
Type "cargo run --example custom_symbols"
Enter
Sleep 2s
Show
Sleep 1.5s

# Navigate through custom symbols
Sleep 1s

# Large Circle & Ring
Down
Sleep 1.5s

# Circle Operators
Down
Sleep 1.5s

# Symbols (yin yang & peace)
Down
Sleep 1.5s

# Science (atom & radioactive)
Down
Sleep 1.5s

# Music (notes)
Down
Sleep 1.5s

# Nature (shamrock & flower)
Sleep 1.5s

# Go back to top
Type "Home"
Sleep 1s

# Navigate through a few
Down
Sleep 0.8s
Down
Sleep 0.8s

# Jump to end
Type "End"
Sleep 1.5s

# Back to middle
Up
Up
Sleep 0.5s

# Final pause
Sleep 2s

Hide
Escape
Sleep 0.3s
