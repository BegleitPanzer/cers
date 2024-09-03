use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use super::ui::{App, CurrentScreen, CurrentlyEditing};
use super::components::{
    titlebar::titlebar, 
    keybind_lowbar::keybind_lowbar,
    process_select::process_select,
    exit::exit
};

pub fn ui(frame: &mut Frame, app: &mut App) {
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.area());
 
    frame.render_widget(titlebar(&app), chunks[0]);


    let key_notes_footer =
       keybind_lowbar();

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(0), Constraint::Percentage(100)])
        .split(chunks[2]);

    frame.render_widget(key_notes_footer, footer_chunks[1]);

    match app.current_screen {
        CurrentScreen::Exiting => {
            let outer_block = Block::bordered().title("Exit CERS").title_alignment(ratatui::layout::Alignment::Center).bg(Color::from_u32(0x00121111));
            let outer_area = centered_rect(40, 40, frame.area());
            let area = centered_rect(30, 15, frame.area());
            let exit_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Length(1),
                ])
                .split(area);
            frame.render_widget(outer_block, outer_area);
            exit(frame, exit_chunks);
        }
        CurrentScreen::SelectingProcess => {
            let outer_block = Block::bordered().title("Process Selection").title_alignment(ratatui::layout::Alignment::Center).bg(Color::from_u32(0x00121111));
            let outer_area = centered_rect(62, 64, frame.area());
            let area = centered_rect(60, 60, frame.area());
            let popup_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Min(3),
                    Constraint::Length(1),
                ])
                .split(area);
            frame.render_widget(outer_block, outer_area);
            process_select(area, frame, popup_chunks, &mut app.list_state);
            }
        _ => {

        }
    }

}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}