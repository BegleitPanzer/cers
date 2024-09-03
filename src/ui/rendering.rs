use ratatui::{
    buffer::Buffer, layout::{Constraint, Direction, Layout, Position, Rect}, style::{Color, Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Widget, Wrap}, Frame
};

use super::{components::mem_view_window::mem_view_window, main::{App, CurrentScreen, InputMode}};
use super::components::{
    titlebar::titlebar, 
    keybind_lowbar::keybind_lowbar,
    process_select::process_select,
    exit::exit,
    search_settings::search_settings
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

    let main_body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[1]);

    let mem_view_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .vertical_margin(1)
        .horizontal_margin(1)
        .split(main_body[0]);

    let search_settings_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .split(main_body[1]);

    let input_bar_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(75),
            Constraint::Percentage(25),
        ])
        .split(search_settings_chunks[0]);

    let mvb = Block::bordered()
        .title(format!(" Found: {} ", app.mem_view_list.list.len()))
        .title_alignment(ratatui::layout::Alignment::Center)
        .bg(Color::from_u32(0x00151414));
    let mvba = Rect { x: main_body[0].x, y: main_body[0].y, width: main_body[0].width, height: main_body[0].height };
    frame.render_widget(mvb, mvba);
    mem_view_window(mem_view_chunks[0], frame, mem_view_chunks, app);


    search_settings(search_settings_chunks[0], frame, &input_bar_chunks, app);


    let key_notes_footer =
       keybind_lowbar();

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(0), Constraint::Percentage(100)])
        .split(chunks[2]);

    frame.render_widget(key_notes_footer, footer_chunks[1]);

    match app.input_mode {
        InputMode::Editing => frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            input_bar_chunks[0].x + app.query.0 as u16 + 1,
            // Move one line down, from the border to the input line
            input_bar_chunks[0].y + 1,
        )),
        InputMode::Normal => {}
    }

    match app.current_screen {
        CurrentScreen::Exiting => {
            let outer_block = Block::bordered().title(" Exit CERS ").title_alignment(ratatui::layout::Alignment::Center).bg(Color::from_u32(0x00121111));
            let outer_area = centered_rect(32, 40, frame.area());
            frame.render_widget(Clear, outer_area); // clears the area for this box so that the border doesn't show through
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
            
            let outer_block = Block::bordered().title(" Process Selection ").title_alignment(ratatui::layout::Alignment::Center).bg(Color::from_u32(0x00121111));
            let outer_area = centered_rect(62, 70, frame.area());
            let area = centered_rect(60, 60, frame.area());
            frame.render_widget(Clear, outer_area); // clears the area for this box so that the border doesn't show through
            let popup_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Min(3),
                    Constraint::Length(1),
                ])
                .split(area);
            frame.render_widget(outer_block, outer_area);
            process_select(area, frame, popup_chunks, app);
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