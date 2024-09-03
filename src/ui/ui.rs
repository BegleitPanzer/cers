

use std::{io, time::Duration, error::Error};
use crate::backend::components::get_process_list::get_process_list;

use super::backend::process::process::Process;
use super::rendering::ui;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::{Backend, CrosstermBackend};
use ratatui::widgets::{List, ListState};
use ratatui::{Frame, Terminal};
use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use ratatui::crossterm::event::DisableMouseCapture;
use ratatui::crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};

#[derive(Debug, Default)]
pub enum CurrentScreen {
    #[default]
    Main,
    SelectingProcess,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

#[derive(Debug, Default)]
pub struct VList {
    pub list: List<'static>,
    pub state: ListState,
}

impl VList {
    fn new() -> Self {
        let state = ListState::default();
        let list = List::default();
        VList { list, state }
    }
}




#[derive(Debug, Default)]
pub struct App {
    pub open_process: Option<Process>,
    pub current_screen: CurrentScreen,
    pub proc_list: VList, // i really wish i didnt have to put this here lmfao
    pub mem_view_list: VList,
    pub query: String,
    pub exit: bool,
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            open_process: None,
            current_screen: CurrentScreen::Main,
            proc_list: VList::new(),
            mem_view_list: VList::new(),
            query: String::new(),
            exit: false
        };
        app.proc_list.state.select(Some(0)); // set a default value so the list renders properly
        app.mem_view_list.state.select(Some(0));
        app
    }

    fn render_frame(&self, frame: &mut Frame) {
        todo!()
    }

    fn handle_key_press(&mut self, key: KeyEvent) {
        todo!();
    }

    fn handle_events(&mut self) -> io::Result<()> {
        let timeout = Duration::from_secs_f64(1.0 / 50.0);
        if !event::poll(timeout)? {
            return Ok(());
        }
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_key_press(key),
            _ => {}
        }
        Ok(())
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;


    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    KeyCode::Char('p') => {
                        app.current_screen = CurrentScreen::SelectingProcess;
                    }
                    KeyCode::Char('j') | KeyCode::Up => {
                        app.mem_view_list.state.select_previous()
                    }
                    KeyCode::Char('k') | KeyCode::Down => {
                        app.mem_view_list.state.select_next()
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    KeyCode::Char('c') => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                }
                CurrentScreen::SelectingProcess => match key.code {
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Char('j') | KeyCode::Up => {
                        app.proc_list.state.select_previous()
                    }
                    KeyCode::Char('k') | KeyCode::Down => {
                        app.proc_list.state.select_next()
                    }
                    KeyCode::Char('c') => {
                        let processes = get_process_list();
                        let Some(idx) = app.proc_list.state.selected()
                        else { continue; };
                        app.open_process = Process::open(processes[idx].1).ok();
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                }
                _ => {}
            }
        }
    }
}
