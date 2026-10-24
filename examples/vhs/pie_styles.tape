# VHS Recording Script for tui-piechart: pie styles, donut hole, gap, explode
# Run: vhs examples/vhs/pie_styles.tape

Output "examples/vhs/output/pie_styles.gif"
Set Theme "Catppuccin Macchiato"
Set Width 1600
Set Height 1000
Set FontSize 16
Set Padding 20

Hide
Type "clear && cargo run --release --example pie_styles"
Enter
Sleep 5s
Show

Sleep 2s
Type "h"
Sleep 1.5s
Type "g"
Sleep 1.5s
Tab
Sleep 2s
Type "e"
Sleep 1.5s
Tab
Sleep 2s
Tab
Sleep 2s
Right
Right
Right
Right
Sleep 2s
Tab
Sleep 2s

Hide
Type "q"
Sleep 0.3s
