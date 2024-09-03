use std::{iter, rc::Rc};

use ratatui::{
    layout::{Layout, Rect}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, BorderType, Borders, List, ListDirection, ListState, Paragraph}, Frame
};

use crate::{backend::components::get_mem_from_query::get_mem_from_query, ui::main::App};

use super::super::backend::components;

pub fn mem_view_window(area: Rect, frame: &mut Frame, chunks: Rc<[Rect]>, app: &mut App) {
    
    let process = String::from("Address");
    let process_id = String::from("Value");
    

    let line_width: usize = process.len() + process_id.len();
    let space_count = area.width as usize - line_width;

    let spaces: String = iter::repeat(' ').take(space_count - 5).collect::<String>();
    let title_lines: Vec<Span<'_>> = vec![
        "  ".into(),
        process.into(),
        spaces.into(),
        process_id.into(),
    ];
    let title = Line::from(title_lines).bg(Color::from_u32(0x00151414));
    frame.render_widget(title, chunks[0]);

    let results: &Vec<(String, String)> = &app.query_results;
    let results_styled = results.clone().into_iter().map(|p| {
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
        if results.iter().position(|q| p.1 == q.1).unwrap() % 2 == 0 { Line::from(process_lines).bg(Color::from_u32(0x00363636)) }
        else { Line::from(process_lines).bg(Color::from_u32(0x00252525)) }
    
    }).collect::<Vec<Line<'_>>>();
    if results_styled.len() == 0 {
        let empty = Line::from("No Results Available").centered().bg(Color::from_u32(0x00252525));
        frame.render_widget(empty, chunks[1]);
    }
    else {
    let list = List::new(results_styled)
        .direction(ListDirection::TopToBottom)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("❚").bg(Color::from_u32(0x00151414))
        .repeat_highlight_symbol(false);
    app.mem_view_list.list = list.clone();
    frame.render_stateful_widget(list, chunks[1], &mut app.mem_view_list.state);
    }


    let lines: Vec<Span<'_>> = vec![
        "[m]: Modify Value".dark_gray(),
        "  |  ".fg(Color::from_u32(0x00151414)),
        "[c]: Copy Address".dark_gray(),
    ];
    let bar = Line::from(lines).centered().bg(Color::from_u32(0x00212121));
    frame.render_widget(bar, chunks[2]);

}