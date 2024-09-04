use std::rc::Rc;

use ratatui::{
    layout::Rect, style::{Color, Style, Stylize}, widgets::{Block, Paragraph}, Frame
};

use crate::ui::main::InputMode;


pub fn addr_bounds(frame: &mut Frame, chunks: &Rc<[Rect]>, bounds: ((i32, String), (i32, String)), input_mode: InputMode) {
    
    let input = Paragraph::new(bounds.0.1.as_str())
            .style(match input_mode {
                InputMode::EditingLowerBound => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(Block::bordered().title("Address Lower Bound"))
            .bg(Color::from_u32(0x00151414));

    frame.render_widget(input, chunks[0]);

    let input = Paragraph::new(bounds.1.1.as_str())
            .style(match input_mode {
                InputMode::EditingUpperBound => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(Block::bordered().title("Address Upper Bound"))
            .bg(Color::from_u32(0x00151414));
    frame.render_widget(input, chunks[1]);

}