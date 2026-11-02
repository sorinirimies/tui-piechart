---
name: tui-piechart-rules
description: Rules for work in the tui-piechart repository that the code can't show: constraints, ownership and must-keep behaviour. Use before changing, fixing, reviewing or extending code here, together with the rust-in-projects skill.
---

# tui-piechart: rules the code can't show

Hard constraints for this repository. The general conventions (orientation, tests,
scripting, releasing) are in the `rust-in-projects` skill; this file holds only what is
specific to tui-piechart.

* **Public API and docs** use no brand names (`Zed`, `Claude`).
* **Internal macros** use `#[macro_use] mod macros`, never `#[macro_export]`.
* **Fixed-height UI** uses `Constraint::Length(n)` rather than `Percentage`.
* **Unicode escapes** are written `\u{XXXX}` or as literal characters, never
  `\uXXXX`.
* **Examples** must build (`cargo build --examples`).
* **Demo GIFs:** regenerate them after visible changes, and hide the compile
  phase in the tape (`Hide`/`Show`).
