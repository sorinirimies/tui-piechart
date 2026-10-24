# VHS Recording Script for tui-piechart: radial, funnel, radar, treemap, waterfall
# Run: vhs examples/vhs/charts.tape

Output "examples/vhs/output/charts.gif"
Set Theme "Catppuccin Macchiato"
Set Width 1600
Set Height 1000
Set FontSize 16
Set Padding 20

Hide
Type "clear && cargo run --release --example charts"
Enter
Sleep 5s
Show

Sleep 4s
Type "t"
Sleep 3s
Tab
Sleep 5s

Hide
Type "q"
Sleep 0.3s
