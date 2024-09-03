use ratatui::{
    style::{Color, Stylize},
    text::{Line, Span}
};

pub fn keybind_lowbar() -> Line<'static> {
    let lines: Vec<Span<'_>> = vec![
        "[q]: Quit".dark_gray(),
        "  |  ".fg(Color::from_u32(0x00242222)),
        "[p]: Select Process".dark_gray(),
        "  |  ".fg(Color::from_u32(0x00242222)),
        "[t]: Change Scan Type".dark_gray(),
        "  |  ".fg(Color::from_u32(0x00242222)),
        "[s]: Change Search Query".dark_gray(),
    ];
    let bar = Line::from(lines).centered().bg(Color::from_u32(0x00121111));
    bar
}