#![cfg(feature = "theme")]
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};
use tui_piechart::{border_style::BorderStyle, PieChart, PieSlice, Theme, Themed};

fn render(c: PieChart) -> Buffer {
    let a = Rect::new(0, 0, 40, 15);
    let mut b = Buffer::empty(a);
    c.render(a, &mut b);
    b
}

fn slices(t: &Theme) -> Vec<PieSlice<'static>> {
    vec![
        PieSlice::themed("A", 60.0, t, 0),
        PieSlice::themed("B", 40.0, t, 1),
    ]
}

#[test]
fn slice_uses_palette() {
    let t = Theme::dracula();
    assert_eq!(PieSlice::themed("x", 1.0, &t, 2).color(), t.color(2));
}

#[test]
fn no_theme_unchanged() {
    let s = vec![PieSlice::new("A", 1.0, Color::Red)];
    let b = render(PieChart::new(s));
    assert_eq!(b[(0, 0)].fg, Color::Reset);
    assert_eq!(b[(0, 0)].bg, Color::Reset);
}

#[test]
fn explicit_style_wins_both_orders() {
    let t = Theme::nord();
    let st = Style::default().fg(Color::Magenta);
    let a = render(PieChart::new(slices(&t)).theme(&t).style(st));
    let b = render(PieChart::new(slices(&t)).style(st).theme(&t));
    assert_eq!(a, b);
    assert_eq!(a[(0, 0)].fg, Color::Magenta);
    assert_eq!(a[(0, 0)].bg, t.background.unwrap_or(Color::Reset));
}

#[test]
fn theme_sets_base() {
    let t = Theme::nord();
    let b = render(PieChart::new(slices(&t)).apply_theme(&t));
    assert_eq!(b[(0, 0)].fg, t.foreground);
}

#[test]
fn all_themes_render() {
    for t in Theme::all() {
        let c = PieChart::new(slices(&t))
            .block(BorderStyle::Rounded.themed_block(&t, true))
            .theme(&t);
        let _ = render(c);
    }
}
