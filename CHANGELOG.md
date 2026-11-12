# Changelog

All notable changes to this project are documented here. It is generated from
[Conventional Commits](https://www.conventionalcommits.org) by
[git-cliff](https://git-cliff.org) — do not edit it by hand.

## [1.4.4](https://github.com/sorinirimies/tui-piechart/compare/v1.4.3...v1.4.4) — 2026-11-12

### 🔄 CI and release

- Add Forgejo workflows for Codeberg ([`24f0e6d`](https://github.com/sorinirimies/tui-piechart/commit/24f0e6dfb8992839be90ccdf0fa00f50fffc3d17))

## [1.4.3](https://github.com/sorinirimies/tui-piechart/compare/v1.4.2...v1.4.3) — 2026-11-12

### ♻️ Refactor

- Remove dead_code allows the build doesn't need; gate on unused dependencies ([`7f09cc2`](https://github.com/sorinirimies/tui-piechart/commit/7f09cc2e89ce4afbe5097d58dab6ac792a65382c))

### 📚 Documentation

- Drop the per-repo agent rules skill; the shared skills cover it ([`1c6efb3`](https://github.com/sorinirimies/tui-piechart/commit/1c6efb3134c6a4530e22016ef814eb1bff067417))
- This repo's agent rules as a project skill in .agents/skills ([`861b5fc`](https://github.com/sorinirimies/tui-piechart/commit/861b5fcf271d7336ce9dc02cc91ccd04c7578a81))
- **readme:** Add the Audit and Fuzz badges ([`a351eeb`](https://github.com/sorinirimies/tui-piechart/commit/a351eeb1d0d61303d0073d32b4260d9f6e0f1b95))
- Move agent rules from .zed/rules.md to AGENTS.md ([`c8acfd0`](https://github.com/sorinirimies/tui-piechart/commit/c8acfd03679154a690d1221e3758d3aaff744cbe))

### 📦 Dependencies

- Upgrade dependencies 2026-11-12 ([`9723fd1`](https://github.com/sorinirimies/tui-piechart/commit/9723fd19fcd05bf076a22509adc15a57bd5f1161))
- Nightly cargo update 2026-11-10 ([`d52c861`](https://github.com/sorinirimies/tui-piechart/commit/d52c86107011afa468d9e05c44ce87998c3331f3))
- Nightly cargo update 2026-11-09 ([`ca2724c`](https://github.com/sorinirimies/tui-piechart/commit/ca2724ca19101cff002e07ae6bced24e17af42ed))
- Nightly cargo update 2026-11-07 ([`e81c828`](https://github.com/sorinirimies/tui-piechart/commit/e81c828e375b9c35008caf3b9ebedf129b5079b8))
- Nightly cargo update 2026-11-06 ([`499c79e`](https://github.com/sorinirimies/tui-piechart/commit/499c79ed52da062809354c874c63582ecaff16b9))
- Nightly cargo update 2026-11-05 ([`c928af7`](https://github.com/sorinirimies/tui-piechart/commit/c928af7c1df37f6e72f5512a3fa49276234a86a1))
- Nightly cargo update 2026-11-03 ([`0f886ca`](https://github.com/sorinirimies/tui-piechart/commit/0f886ca4bb5e91bf5aba5fbae1d7c0c95423a859))

### 📦 Build

- Align tooling with the library template ([`a8f5030`](https://github.com/sorinirimies/tui-piechart/commit/a8f5030d2c778dd88b4a67ebff1f1dc8e8c6c8f5))

### 🔄 CI and release

- Unset the lfs filter on GIFs so the nightly release tree stays clean ([`a70910d`](https://github.com/sorinirimies/tui-piechart/commit/a70910d4e5cf49b86885fff8311f2eff8085cbeb))
- **gitea:** Check the MSRV on Gitea too, with the same command as GitHub ([`cc138c7`](https://github.com/sorinirimies/tui-piechart/commit/cc138c77c1970ce3e70cb524b13fba66d2c800b9))
- Download archives to a file so curl retries a dropped transfer, instead of piping into tar ([`c047ffc`](https://github.com/sorinirimies/tui-piechart/commit/c047ffc3ad4c1502fbd779ae66d486800efa9fff))
- The Dependabot sweep asks for a rebase on conflicts and keeps going after a failed merge ([`28f50ab`](https://github.com/sorinirimies/tui-piechart/commit/28f50ab9ef92d73a1d2c1cd43f8a345921977366))
- Merge green Dependabot PRs, majors included, with a nightly sweep; nightly commits skip ignored lockfiles ([`943aaf3`](https://github.com/sorinirimies/tui-piechart/commit/943aaf3952b9269d844646021b0bd3cb6b5d6886))
- Scan the git history for secrets with gitleaks ([`6d7177d`](https://github.com/sorinirimies/tui-piechart/commit/6d7177dbf4cfe6887ed59449c7e24e0a6a221c7b))

### 🔧 Chores

- Ignore .codegraph index ([`547bcb4`](https://github.com/sorinirimies/tui-piechart/commit/547bcb40f8ae98abbf1192d96d1b05c723967f95))
- Remove AGENTS.md ([`48fd7bb`](https://github.com/sorinirimies/tui-piechart/commit/48fd7bb1972bba0c3e1e99543b8af5a701b83b60))

## [1.4.2](https://github.com/sorinirimies/tui-piechart/compare/v1.4.1...v1.4.2) — 2026-10-28

### 🐛 Bug fixes

- Resolve Cargo.toml merge markers ([`b39c609`](https://github.com/sorinirimies/tui-piechart/commit/b39c609e4d480660b39e129502cee5b843c78b0d))

### 📦 Dependencies

- Nightly cargo update 2026-10-28 ([`6349072`](https://github.com/sorinirimies/tui-piechart/commit/6349072827fd5389ebdf6e49f9fa1511f62cddff))
- Bump tui-nuances to 0.1.10 ([`4efb4e9`](https://github.com/sorinirimies/tui-piechart/commit/4efb4e927dee5c18896a94896f03539b31e096e3))

## [1.4.1](https://github.com/sorinirimies/tui-piechart/compare/v1.4.0...v1.4.1) — 2026-10-26

### ✨ Features

- Enable the tui-nuances theme feature by default ([`8377728`](https://github.com/sorinirimies/tui-piechart/commit/83777281e2a7a745f8917ca8474ffff50adac566))

### 📚 Documentation

- Show pie styles preview first ([`631e5ec`](https://github.com/sorinirimies/tui-piechart/commit/631e5ecd8df782eec46cfd83c0c172c58fed41a9))
- Make macro doctests runnable ([`278373c`](https://github.com/sorinirimies/tui-piechart/commit/278373c086cfe2b58e3133e0c108e8caae4b5b51))

### 📦 Dependencies

- Nightly cargo update 2026-10-26 ([`a96eb22`](https://github.com/sorinirimies/tui-piechart/commit/a96eb22f6d2743a0947875f55274b9f1c420189f))

### 🔄 CI and release

- Bump actions/upload-artifact from 4 to 7 (#10) ([`b653980`](https://github.com/sorinirimies/tui-piechart/commit/b653980c290d89eea33b3eaa87e4d02d8dca424a))
- **gitea:** Nightly workflow-action bumps ([`3b7bb69`](https://github.com/sorinirimies/tui-piechart/commit/3b7bb69fac1a50868eed8d796a87dcb16e28986b))
- Automerge major dependabot bumps too ([`dfe14ec`](https://github.com/sorinirimies/tui-piechart/commit/dfe14ec1fd04e3432b077f243b909db5d429c956))
- Automerge green PRs (workflow_run) + dependabot for actions ([`2932061`](https://github.com/sorinirimies/tui-piechart/commit/293206151e0453f8ef4a43aeae253cfbb9ba7e58))
- Auto-merge dependabot PRs ([`a54d1a1`](https://github.com/sorinirimies/tui-piechart/commit/a54d1a14a96d7bb9465b7978d1ae0321575c380b))
- Bump dependabot/fetch-metadata from 2 to 3 (#9) ([`77be590`](https://github.com/sorinirimies/tui-piechart/commit/77be5908770687c6bef4f8c5a060709df5036a20))
- Bump clechasseur/rs-clippy-check from 4 to 5 (#8) ([`0d5cc4e`](https://github.com/sorinirimies/tui-piechart/commit/0d5cc4eeaea5992d9df4acf0c25e936356e0f4e0))
- Bump softprops/action-gh-release from 2 to 3 (#7) ([`b8091f3`](https://github.com/sorinirimies/tui-piechart/commit/b8091f3e9c66d8f1db3751389df043be6c1ad744))
- Add coverage job ([`7d88127`](https://github.com/sorinirimies/tui-piechart/commit/7d881274a79392e3399b9a4a8fa59cce5247fd11))

### 🔧 Chores

- Bump tui-nuances to 0.1.7 ([`aa6ba6e`](https://github.com/sorinirimies/tui-piechart/commit/aa6ba6e4f85c4151e76c3017e1efaea97d16936a))

## [1.4.0](https://github.com/sorinirimies/tui-piechart/compare/v1.3.0...v1.4.0) — 2026-10-25

### ✨ Features

- PieChart::areas and slice_list for hit testing; refresh pie_styles demo ([`f229f2c`](https://github.com/sorinirimies/tui-piechart/commit/f229f2cb9838e484b067ea32b35ee988ca3eee47))

## [1.3.0](https://github.com/sorinirimies/tui-piechart/compare/v1.2.0...v1.3.0) — 2026-10-24

### ⚠️ Breaking changes

- Move gauge, radial, funnel, radar, treemap and waterfall widgets to tui-chart; piechart keeps pie options and styles ([`cc25719`](https://github.com/sorinirimies/tui-piechart/commit/cc25719634176ad3b29434d0d0883ec8aa8cea67))

### ♻️ Refactor

- Move gauge, radial, funnel, radar, treemap and waterfall widgets to tui-chart; piechart keeps pie options and styles ([`cc25719`](https://github.com/sorinirimies/tui-piechart/commit/cc25719634176ad3b29434d0d0883ec8aa8cea67))

### 📚 Documentation

- VHS tapes and demo GIFs for gauges, pie styles and new charts ([`25747b6`](https://github.com/sorinirimies/tui-piechart/commit/25747b63e57d563cbb6e81a9447c20b1608ed87f))

## [1.2.0](https://github.com/sorinirimies/tui-piechart/compare/v1.1.1...v1.2.0) — 2026-10-24

### ✨ Features

- RadialBar, RingStack, Funnel, RadarChart, Treemap, Waterfall; charts example; widgets fuzz target ([`1cce0c5`](https://github.com/sorinirimies/tui-piechart/commit/1cce0c5caedc74579abf1b25d1c2b3d8750f30b0))
- **pie:** Donut hole, start angle, gap, explode, center text; Segmented/Hud/Rose styles ([`8c11827`](https://github.com/sorinirimies/tui-piechart/commit/8c11827ba944a3a074380fbe94d522a226857528))
- GaugeChart with 15 styles (radial braille, linear, vertical), example and fuzz target ([`e72abb9`](https://github.com/sorinirimies/tui-piechart/commit/e72abb974ce39911169b561c94b5ecd22a4425f8))

### 🔄 CI and release

- Port tokenburn's multi-job nightly dep update; set real MSRV 1.88 ([`92f4e1e`](https://github.com/sorinirimies/tui-piechart/commit/92f4e1ec37788161d82c8fc394bdee452ba92371))
- Add dependabot, bump GitHub actions (upload-artifact v3 is deprecated) ([`6fa0dab`](https://github.com/sorinirimies/tui-piechart/commit/6fa0dab997e421aab0817b0252684b8953f8192a))

### 🔧 Chores

- Nightly dependency update 2026-10-24 ([`1a33d8a`](https://github.com/sorinirimies/tui-piechart/commit/1a33d8a575495c2dff1ce0e715ba267c3c98ab52))

## [1.1.1](https://github.com/sorinirimies/tui-piechart/compare/v1.1.0...v1.1.1) — 2026-10-23

### 🔧 Chores

- Update deps, fix clippy lints, add auto-merge workflow ([`d6028f4`](https://github.com/sorinirimies/tui-piechart/commit/d6028f44c3f8a83d2188fbb8d0c5ec46c3191140))
- Nightly dependency update 2026-10-20 ([`f800146`](https://github.com/sorinirimies/tui-piechart/commit/f800146b944a8f03f90520cd5f108d6ab7bc0955))
- Nightly dependency update 2026-10-18 ([`68d9546`](https://github.com/sorinirimies/tui-piechart/commit/68d95461a1832fd7e4478e37006888e640140d7e))
- Nightly dependency update 2026-10-17 ([`b224351`](https://github.com/sorinirimies/tui-piechart/commit/b224351605a28d12227e27e5602f9ef57a76379f))
- Nightly dependency update 2026-10-16 ([`b777aea`](https://github.com/sorinirimies/tui-piechart/commit/b777aead1936ebad035c19dfdbaa4cedcd00d2f3))
- **justfile:** Push/pull current branch instead of hardcoded main ([`31c6fcc`](https://github.com/sorinirimies/tui-piechart/commit/31c6fccb9e8332ef0b75a457ee9b5ad299817f31))
- **justfile:** Skip unconfigured remotes in *-all recipes ([`153141b`](https://github.com/sorinirimies/tui-piechart/commit/153141bc226eea95e01c9499cdb7be3e642a3f80))

## [1.1.0](https://github.com/sorinirimies/tui-piechart/compare/v1.0.3...v1.1.0) — 2026-10-14

### ✨ Features

- Make theming an optional "theme" feature ([`465a6ed`](https://github.com/sorinirimies/tui-piechart/commit/465a6ed983d4d87208a681ce1878e824d4ad3173))
- Shared theming via tui-nuances ([`9599b63`](https://github.com/sorinirimies/tui-piechart/commit/9599b6378a6a34f435cd23066b3db5e68bf9da72))

### 📚 Documentation

- Re-record demos showing results only; reference tui-nuances version ([`433f8b6`](https://github.com/sorinirimies/tui-piechart/commit/433f8b64942dbfc79674db78297e7f6c86f8ccf0))
- **readme:** Restyle crates.io version badge ([`5453578`](https://github.com/sorinirimies/tui-piechart/commit/54535785f37de7d37e2d6f743643dec55618375a))

### 🔄 CI and release

- Prebuilt Nushell, cargo-deny + cargo-audit security workflow; add cargo-fuzz ([`979fdce`](https://github.com/sorinirimies/tui-piechart/commit/979fdce84c0ef7b0a2cbd8ca8bb6221b76f453ec))
- **security:** Add cargo-audit workflow and just audit recipe ([`a980b4d`](https://github.com/sorinirimies/tui-piechart/commit/a980b4d11a01fcff98dc8f8a038e0dfbe9e77c1a))

### 🔧 Chores

- Depend on tui-nuances 0.1.6 from crates.io ([`ab59028`](https://github.com/sorinirimies/tui-piechart/commit/ab59028573004e3f7765f941ebef15aa511dfc3f))
- Depend on tui-nuances 0.1.6 ([`4fba635`](https://github.com/sorinirimies/tui-piechart/commit/4fba6357d6addc0d4d1a4f5bd35b0fc4538c12a2))
- Depend on tui-nuances 0.1.5 ([`ea799f9`](https://github.com/sorinirimies/tui-piechart/commit/ea799f93c788c515df01026830ae7ed5f8fcfcfd))
- Nightly dependency update 2026-10-14 ([`f3b6de9`](https://github.com/sorinirimies/tui-piechart/commit/f3b6de9bc0ecc6eb47ff9abb2181e7c6cf084ed4))
- Nightly dependency update 2026-10-13 ([`ef0bbc6`](https://github.com/sorinirimies/tui-piechart/commit/ef0bbc66ee6d4b9a1a9f6383c77f0d8f7258c755))
- Nightly dependency update 2026-10-12 ([`65d6976`](https://github.com/sorinirimies/tui-piechart/commit/65d697628919051bd0d4e2cfba3f21e2a5ce0ffd))
- Nightly dependency update 2026-10-11 ([`dbb61b8`](https://github.com/sorinirimies/tui-piechart/commit/dbb61b88e982d54f8f99de61fd826798d303a943))

## [1.0.3](https://github.com/sorinirimies/tui-piechart/compare/v1.0.2...v1.0.3) — 2026-10-10

### 🐛 Bug fixes

- **scripts:** Drop deprecated str downcase; exit non-zero when bump is aborted ([`3f687c0`](https://github.com/sorinirimies/tui-piechart/commit/3f687c0dd1fe3ebda04263c251323d2ee8538169))
- **ci:** Make Gitea deps-update commit-only, matching GitHub mirror ([`f29e069`](https://github.com/sorinirimies/tui-piechart/commit/f29e069ec8c7df13a944b069659d9e3330d31ede))

### 📦 Dependencies

- Nightly dependency upgrade 2026-10-04 ([`4b9a2b3`](https://github.com/sorinirimies/tui-piechart/commit/4b9a2b3b02e1a0d62f0105746554390814e8b892))
- Nightly dependency upgrade 2026-10-03 ([`8af873e`](https://github.com/sorinirimies/tui-piechart/commit/8af873e5680b9a53e7549a04fda994594fee6f6e))
- Nightly dependency upgrade 2026-10-01 ([`d8a8687`](https://github.com/sorinirimies/tui-piechart/commit/d8a8687263ad6750bacd27e8db6daa003f033e15))
- Nightly dependency upgrade 2026-09-26 ([`ba19a4e`](https://github.com/sorinirimies/tui-piechart/commit/ba19a4e908812b61284ecf7412072d271a937bec))
- Nightly dependency upgrade 2026-09-23 ([`8f65805`](https://github.com/sorinirimies/tui-piechart/commit/8f6580564fbcb943c3a86bf37dd9180974007829))
- Nightly dependency upgrade 2026-09-17 ([`9821ae9`](https://github.com/sorinirimies/tui-piechart/commit/9821ae92c817370babcceacf290b7a9c694cf3a4))
- Nightly dependency upgrade 2026-09-12 ([`7406c4f`](https://github.com/sorinirimies/tui-piechart/commit/7406c4f9282851e575d36b60ddc10a9c768f6ccc))
- Nightly dependency upgrade 2026-09-10 ([`593d362`](https://github.com/sorinirimies/tui-piechart/commit/593d36240611fc6c07b7cfaa4b5f7c50e59a1426))
- Nightly dependency upgrade 2026-09-09 ([`2c8bad0`](https://github.com/sorinirimies/tui-piechart/commit/2c8bad0fe3d4328206e876e697fbe0ece87290e8))
- Nightly dependency upgrade 2026-05-21 ([`a5d8311`](https://github.com/sorinirimies/tui-piechart/commit/a5d831144e94871f9b023c2b61c1adfd57bfd46b))
- Revert accidental ratatui 0.30 -> 0.29 downgrade ([`fb62a11`](https://github.com/sorinirimies/tui-piechart/commit/fb62a112ede053db1d8077c1043ee8bc42c2f927))
- Nightly dependency upgrade ([`845f1ce`](https://github.com/sorinirimies/tui-piechart/commit/845f1ceb73f6732852f99eb67ccdf0dfee832ba5))

### 🔄 CI and release

- Release only on real Cargo.toml dependency changes; lock-only updates commit without release ([`9d55e56`](https://github.com/sorinirimies/tui-piechart/commit/9d55e565efa81e13d5fbc56a53a91fbffe171661))
- **deps-update:** Move downgrade guard into shared Nushell script ([`b0a488d`](https://github.com/sorinirimies/tui-piechart/commit/b0a488dd2ea001a5072de3f7c9c868ce21a9a117))
- **deps-update:** Guard against silent dependency downgrades ([`dd636a0`](https://github.com/sorinirimies/tui-piechart/commit/dd636a086c141b5095ea80996753bf2046255139))
- Grant contents/pull-requests write perms to deps-update workflow ([`9a09993`](https://github.com/sorinirimies/tui-piechart/commit/9a0999368cb34183f0e2cd5331d4a721421488a7))

### 🔧 Chores

- **just:** Add force-push recipes (per-remote, tags, push-all-force-with-tags) ([`7fc19f7`](https://github.com/sorinirimies/tui-piechart/commit/7fc19f764ad4953a751e90c1dfee1a3c9b2a98ab))
- Nightly dependency update 2026-10-10 ([`4bab035`](https://github.com/sorinirimies/tui-piechart/commit/4bab035c53dbfc6bcf73608138c895c07ff5cb7e))

## [1.0.2](https://github.com/sorinirimies/tui-piechart/compare/v1.0.1...v1.0.2) — 2026-08-05

### 📚 Documentation

- Add crates.io downloads badge to README ([`03aeb82`](https://github.com/sorinirimies/tui-piechart/commit/03aeb82fe8bc34a11bc5fad7fc36de29c02c149b))

## [1.0.0](https://github.com/sorinirimies/tui-piechart/compare/v0.3.3...v1.0.0) — 2026-07-16

### 🐛 Bug fixes

- Render a 100% pie slice as a filled disc instead of a line (#2) ([`aa8ff4e`](https://github.com/sorinirimies/tui-piechart/commit/aa8ff4e4c2ba075e0c0427bb85c7a9f9a2dd0263))

### 🧪 Tests

- Add matches_test! macro and consolidate legend enum tests ([`238b02a`](https://github.com/sorinirimies/tui-piechart/commit/238b02aba63478769334489231822c3a556d2132))

### 📦 Dependencies

- Nightly dependency upgrade 2026-04-27 ([`9fbf728`](https://github.com/sorinirimies/tui-piechart/commit/9fbf728b3cc725e5eecb3df62ba5014cc1b28b86))

## [0.3.2](https://github.com/sorinirimies/tui-piechart/compare/v0.3.1...v0.3.2) — 2026-04-23

### ✨ Features

- Add nightly dependency upgrade workflow ([`896078f`](https://github.com/sorinirimies/tui-piechart/commit/896078f71e71126869fb944cdcb256b59f81537e))

### 🐛 Bug fixes

- Add Stylize trait import to all examples for ratatui 0.29 compat ([`4b45dba`](https://github.com/sorinirimies/tui-piechart/commit/4b45dba5ba8bf668b85e85c2d1f1feb231e86565))

### ♻️ Refactor

- Inline VHS generation into justfile, remove generate_all.sh ([`a2888e3`](https://github.com/sorinirimies/tui-piechart/commit/a2888e3723691b6053a0b98fe45d01c6ce0e615a))
- Migrate shell scripts to Nushell ([`8960a93`](https://github.com/sorinirimies/tui-piechart/commit/8960a934007ab3367009aa0c60a6db5d7f85ba88))

### 🔧 Chores

- Update rules and widen dep-update commit scope ([`3b72446`](https://github.com/sorinirimies/tui-piechart/commit/3b72446233b7e5a611f76e366842ca456bbdb219))

## [0.3.0](https://github.com/sorinirimies/tui-piechart/compare/v0.2.9...v0.3.0) — 2026-02-20

### 🐛 Bug fixes

- **warnings:** Remove unnecessary braces around macro arguments ([`57703a8`](https://github.com/sorinirimies/tui-piechart/commit/57703a8320fa649cf923dc367ddcc1a4d2f2a9c7))
- **warnings:** Remove unused Stylize imports from all examples ([`1ece826`](https://github.com/sorinirimies/tui-piechart/commit/1ece82667273d2541ff0b8207538b09979727a6a))

### 🧪 Tests

- Achieve 100% coverage (95.54% -> 100%) ([`2766e90`](https://github.com/sorinirimies/tui-piechart/commit/2766e900b78567560004507dadbf852bf4480b7e))

## [0.2.9](https://github.com/sorinirimies/tui-piechart/compare/v0.2.4...v0.2.9) — 2026-02-20

### 🐛 Bug fixes

- Sed -i macOS compatibility in bump_version.sh ([`19e508b`](https://github.com/sorinirimies/tui-piechart/commit/19e508b5bf2695362613e309aed3da5764607ddd))

### 🔧 Chores

- Upgrade dependencies to latest versions ([`1ef7a8e`](https://github.com/sorinirimies/tui-piechart/commit/1ef7a8ef90823c3c6016420f67ddd7568998ecc7))

## [0.2.2](https://github.com/sorinirimies/tui-piechart/compare/v0.1.6...v0.2.2) — 2025-11-29

### 📚 Documentation

- Add comprehensive examples section with all demo GIFs to README ([`6287cc6`](https://github.com/sorinirimies/tui-piechart/commit/6287cc6fa236f5eece5fb3a28b1302c322f20a7a))

## [0.1.2](https://github.com/sorinirimies/tui-piechart/releases/tag/v0.1.2) — 2025-11-08


