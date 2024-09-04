use std::rc::Rc;

use ratatui::{
    layout::Rect, style::{Color, Style, Stylize}, text::Line, widgets::{Block, Paragraph}, Frame
};

use crate::ui::main::{InputMode, ScanTypes};

pub fn search_settings(frame: &mut Frame, chunks: &Rc<[Rect]>, input_mode: InputMode, query: &str, scan_type: ScanTypes) {
    
    let input = Paragraph::new(query)
            .style(match input_mode {
                InputMode::EditingQuery => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(Block::bordered().title("Search Query"))
            .bg(Color::from_u32(0x00151414));

    frame.render_widget(input, chunks[0]);

    let input = Paragraph::new(Line::from(vec![
                scan_type.as_str().into(),
    ]))
        .centered()
        .block(Block::bordered().title("Scan Type"))
        .bg(Color::from_u32(0x00151414));
    frame.render_widget(input, chunks[1]);

}