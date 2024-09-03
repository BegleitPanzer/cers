use std::{iter, rc::Rc};

use ratatui::{
    layout::{Layout, Rect}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, BorderType, Borders, List, ListDirection, ListState, Paragraph}, Frame
};

use crate::{backend::components::get_mem_from_query::get_mem_from_query, ui::ui::{App, InputMode}};

use super::super::backend::components;

pub fn search_settings(area: Rect, frame: &mut Frame, chunks: Rc<[Rect]>, app: &mut App) {
    
    let input = Paragraph::new(app.query.as_str())
            .style(match app.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title("Search Query"))
            .bg(Color::from_u32(0x00151414));

    frame.render_widget(input, chunks[0]);

    let input = Paragraph::new(Line::from(vec![
                app.scan_type.as_str().into(),
    ]))
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .centered()
        .block(Block::bordered().title("Scan Type"))
        .bg(Color::from_u32(0x00151414));
    frame.render_widget(input, chunks[1]);

}