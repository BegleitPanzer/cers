use std::{iter, rc::Rc};

use ratatui::{
    layout::{Layout, Rect}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, BorderType, Borders, List, ListDirection, ListState, Paragraph}, Frame
};

use crate::ui::main::App;

use super::super::backend::components;

pub fn process_select(area: Rect, frame: &mut Frame, chunks: Rc<[Rect]>, app: &mut App) {
    
    let process = String::from("Process Name");
    let process_id = String::from("Process ID");
    

    let line_width: usize = process.len() + process_id.len();
    let space_count = area.width as usize - line_width;

    let spaces: String = iter::repeat(' ').take(space_count - 2).collect::<String>();
    let title_lines: Vec<Span<'_>> = vec![
        " ".into(),
        process.into(),
        spaces.into(),
        process_id.into(),
    ];
    let title = Line::from(title_lines).bg(Color::from_u32(0x00121111));
    frame.render_widget(title, chunks[0]);

    let processes = components::get_process_list::get_process_list();
    let processes_styled = processes.clone().into_iter().map(|p| {
        let line_width: usize = p.0.len() + p.1.to_string().len();
        let space_count = area.width as usize - line_width;
        let spaces: String = iter::repeat(' ').take(space_count - 3).collect::<String>();
        let pn = p.0;
        let pid = p.1.to_string();
        let process_lines: Vec<Span<'_>> = vec![
            " ".into(),
            pn.into(),
            spaces.into(),
            pid.into(),

        ];
        if processes.iter().position(|q| p.1 == q.1).unwrap() % 2 == 0 { Line::from(process_lines).bg(Color::from_u32(0x00363636)) }
        else { Line::from(process_lines).bg(Color::from_u32(0x00242424)) }
    
    }).collect::<Vec<Line<'_>>>();
    let list = List::new(processes_styled)
        .direction(ListDirection::TopToBottom)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("❚").bg(Color::from_u32(0x00121111))
        .repeat_highlight_symbol(false);
    app.proc_list.list = list.clone();
    frame.render_stateful_widget(list, chunks[1], &mut app.proc_list.state);


    let lines: Vec<Span<'_>> = vec![
        "[c]: Confirm Process Selection".dark_gray(),
        "  |  ".fg(Color::from_u32(0x00242222)),
        "[q]: Exit Menu".dark_gray(),
    ];
    let bar = Line::from(lines).centered().bg(Color::from_u32(0x00121111));
    frame.render_widget(bar, chunks[2]);

}