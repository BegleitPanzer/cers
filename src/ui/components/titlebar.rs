use std::iter;

use ratatui::{
    style::{Color, Stylize},
    text::{Line, Span},
};

use crate::ui::ui::App;

pub fn titlebar(app: &App) -> Line<'static> {
    let process: Span<'_> = {
        if let Some(x) = &app.open_process {
            x.name().unwrap().light_green()
        }
        else { String::from("No Process Selected!").light_red() }
    };
    let mut title_version_info = vec![
        " ".black(), 
        "CERS ".blue(), 
        dotenv::var("VERSION").unwrap().cyan(), 
        " (indev)".fg(Color::from_u32(0x00242222)),
    ];
    let process_info: Vec<Span<'_>> = vec![
        "[p] ".fg(Color::from_u32(0x00242222)),
        process,
        " ".into()
    ];

    let line_width: usize = title_version_info.iter().map(|x| x.to_string().len()).sum::<usize>() 
    + process_info.iter().map(|x| x.to_string().len()).sum::<usize>();
    let space_count = term_size::dimensions().unwrap().0 as usize - line_width;

    let spaces = iter::repeat(' ').take(space_count).collect::<String>();
    title_version_info.push(spaces.into());
    let final_vec = title_version_info.into_iter().chain(process_info.into_iter()).collect::<Vec<Span<'_>>>();
    let title = Line::from(final_vec).bg(Color::from_u32(0x00121111));

    title
}