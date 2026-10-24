# VHS Recording Script for tui-piechart: gauge showcase
# Run: vhs examples/vhs/gauges.tape

Output "examples/vhs/output/gauges.gif"
Set Theme "Catppuccin Macchiato"
Set Width 1600
Set Height 1000
Set FontSize 16
Set Padding 20

Hide
Type "clear && cargo run --release --example gauges"
Enter
Sleep 5s
Show

Sleep 4s
Type "z"
Sleep 3s
Type "z"
Tab
Sleep 5s
Tab
Sleep 5s
Tab
Sleep 3s

Hide
Type "q"
Sleep 0.3s
