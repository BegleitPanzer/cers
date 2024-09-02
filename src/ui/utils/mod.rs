pub mod utils {
    use ratatui::{
        layout::{Constraint, Direction, Layout, Rect},
        style::{Color, Style, Stylize},
        text::{Line, Span, Text},
        widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
        Frame,
    };

    use std::iter;


    /*pub fn center_text(lines: Vec<Span<'_>>) -> Line<'_> {
        let dummy_line = Line::from(lines.clone());
        let space_count = term_size::dimensions().unwrap().0 / 2 as usize - dummy_line.width() / 2;
        let spaces = iter::repeat(' ').take(space_count).collect::<String>();
        // rust doesn't let you prepend to a vec natively, so this is a little rough
        let x: Vec<Span<'_>> = vec![spaces.into()];
        let x: Vec<Span<'_>> = x.clone().into_iter()
        .chain(lines.into_iter())
        .chain(x.into_iter())
        .collect::<Vec<Span<'_>>>();
        Line::from(x)
    }*/
}