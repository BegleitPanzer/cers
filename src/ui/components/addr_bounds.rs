use std::{iter, rc::Rc};

use ratatui::{
    layout::{Layout, Rect}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, BorderType, Borders, List, ListDirection, ListState, Paragraph}, Frame
};

use crate::{backend::components::get_mem_from_query::get_mem_from_query, ui::main::{App, InputMode}};

use super::super::backend::components;

pub fn addr_bounds(area: Rect, frame: &mut Frame, chunks: &Rc<[Rect]>, app: &mut App) {
    
    let input = Paragraph::new(app.bounds.0.1.as_str())
            .style(match app.input_mode {
                InputMode::EditingLowerBound => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(Block::bordered().title("Address Lower Bound"))
            .bg(Color::from_u32(0x00151414));

    frame.render_widget(input, chunks[0]);

    let input = Paragraph::new(app.bounds.1.1.as_str())
            .style(match app.input_mode {
                InputMode::EditingUpperBound => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(Block::bordered().title("Address Upper Bound"))
            .bg(Color::from_u32(0x00151414));
    frame.render_widget(input, chunks[1]);

}