use std::{iter, rc::Rc};

use ratatui::{
    layout::{Layout, Rect}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span}, widgets::{List, ListDirection, ListState, Paragraph}, Frame
};

use crate::ui::ui::App;

use super::super::backend::components;

pub fn exit(frame: &mut Frame, chunks: Rc<[Rect]>) {

    let title = Line::from("Are you sure you want to exit?").bg(Color::from_u32(0x00121111)).centered();
    frame.render_widget(title, chunks[0]);

    let lines: Vec<Span<'_>> = vec![
        "[q]: Confirm Quit".light_green(),
        "  |  ".fg(Color::from_u32(0x00242222)),
        "[c]: Cancel".light_red(),
    ];
    let bar = Line::from(lines).centered().bg(Color::from_u32(0x00121111));
    frame.render_widget(bar, chunks[1]);

}