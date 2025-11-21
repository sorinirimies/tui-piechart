# VHS Recording Script for tui-piechart Title Styles Demo
# This script demonstrates all available title font styles
# Install VHS: https://github.com/charmbracelet/vhs
# Run: vhs examples/vhs/title_styles.tape

Output "examples/vhs/output/title_styles.gif"
Set Theme "Catppuccin Macchiato"
Set Width 1600
Set Height 1200
Set FontSize 16
Set Padding 20

Hide
Type "cargo run --example title_styles_example"
Enter
Sleep 2s
Show
Sleep 1s

# Show the first style (Normal)
Sleep 1.5s

# Navigate to Bold
Down
Sleep 1.5s

# Navigate to Italic
Down
Sleep 1.5s

# Navigate to BoldItalic
Down
Sleep 1.5s

# Navigate to Script
Down
Sleep 1.5s

# Navigate to BoldScript
Down
Sleep 1.5s

# Navigate to SansSerif
Down
Sleep 1.5s

# Navigate to BoldSansSerif
Down
Sleep 1.5s

# Navigate to ItalicSansSerif
Down
Sleep 1.5s

# Navigate to Monospace
Down
Sleep 1.5s

# Back to top to show Normal again
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

# Show Script style
Up
Up
Up
Sleep 1.5s

Sleep 1s

Hide
Escape
Sleep 0.3s
