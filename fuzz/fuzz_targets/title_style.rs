#![no_main]
use libfuzzer_sys::fuzz_target;
use tui_piechart::title::Title;
use tui_piechart::*;

fuzz_target!(|data: &str| {
    for s in [TitleStyle::Normal, TitleStyle::Bold, TitleStyle::Italic, TitleStyle::BoldItalic,
              TitleStyle::Script, TitleStyle::BoldScript, TitleStyle::SansSerif,
              TitleStyle::BoldSansSerif, TitleStyle::ItalicSansSerif, TitleStyle::Monospace] {
        let _ = s.apply(data);
    }
    let _ = Title::new(data).left().bottom().render();
});
