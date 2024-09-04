

use std::sync::{Mutex, Arc, mpsc::{self, Receiver, Sender}};
use std::thread;
use std::{io, time::Duration, error::Error};
use crate::backend::{components::{
    get_mem_from_query::get_mem_from_query, get_process_list::get_process_list
}, process};

use futures::{future, pin_mut, SinkExt, StreamExt};
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
use tokio::runtime::Handle;

#[derive(Debug, Default, PartialEq, Clone)]
pub enum CurrentScreen {
    #[default]
    Main,
    SelectingProcess,
    Exiting,
}


#[derive(Debug, Default, PartialEq, Clone)]
pub enum InputMode {
    #[default]
    Normal,
    EditingQuery,
    EditingUpperBound,
    EditingLowerBound,
}

#[derive(Debug, Default, Clone)]
pub enum ScanTypes {
    #[default]
    Exact,
    Range,
    Unknown,
}

pub enum DataType {
    ProgressMsg,
    QueryResults,
    QueryProgress,
}
pub struct Data {
    pub data_type: DataType,
    pub data: String
}

impl ScanTypes {
    pub fn as_str(&self) -> &str {
        match self {
            ScanTypes::Exact => "Exact",
            ScanTypes::Range => "Range",
            ScanTypes::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Default, Clone)]
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




#[derive(Debug)]
pub struct App {
    pub open_process: i32,
    pub current_screen: CurrentScreen,
    pub proc_list: VList, // i really wish i didnt have to put this here lmfao
    pub mem_view_list: VList,
    pub query: (i32, String), // i32 is the index of the character, necessary for cursor positioning
    pub bounds: ((i32, String), (i32, String)), // upper and lower bounds for memory scanning, respectively
    pub query_results: Vec<(String, String)>,
    pub query_progress: f64,
    pub tx: Sender<Data>,
    pub rx: Receiver<Data>,
    pub progress_msg : String,
    pub input_mode: InputMode,
    pub scan_type: ScanTypes,

}

#[derive(Debug, Clone)]
pub struct AMApp {
    pub app: Arc<Mutex<App>>
}

impl AMApp {
    //setters
    pub fn modify_process(&self, process: i32) {
        let mut app = self.app.lock().unwrap();
        app.open_process = process;
    }
    pub fn modify_current_screen(&self, screen: CurrentScreen) {
        let mut app = self.app.lock().unwrap();
        app.current_screen = screen;
    }
    pub fn modify_query(&self, query: (i32, String)) {
        let mut app = self.app.lock().unwrap();
        app.query = query;
    }
    pub fn modify_bounds(&self, bounds: ((i32, String), (i32, String))) {
        let mut app = self.app.lock().unwrap();
        app.bounds = bounds;
    }
    pub fn modify_query_results(&self, results: Vec<(String, String)>) {
        let mut app = self.app.lock().unwrap();
        app.query_results = results;
    }
    pub fn modify_query_progress(&self, progress: f64) {
        let mut app = self.app.lock().unwrap();
        app.query_progress = progress;
    }
    pub fn modify_progress_msg(&self, msg: String) {
        let mut app = self.app.lock().unwrap();
        app.progress_msg = msg;
    }
    pub fn modify_input_mode(&self, mode: InputMode) {
        let mut app = self.app.lock().unwrap();
        app.input_mode = mode;
    }
    pub fn modify_scan_type(&self, scan_type: ScanTypes) {
        let mut app = self.app.lock().unwrap();
        app.scan_type = scan_type;
    }
    pub fn modify_mem_view_list(&self, action: &str, list: Option<List<'static>>) {
        match action {
            "prev" => {
                let mut app = self.app.lock().unwrap();
                app.mem_view_list.state.select_previous();
            },
            "next" => {
                let mut app = self.app.lock().unwrap();
                app.mem_view_list.state.select_next();
            },
            "set" => {
                let mut app = self.app.lock().unwrap();
                app.mem_view_list.list = list.unwrap();
            },
            _ => {
                
            }
        };
    }

    pub fn modify_proc_list(&self, action: &str, list: Option<List<'static>>) {
        match action {
            "prev" => {
                let mut app = self.app.lock().unwrap();
                app.proc_list.state.select_previous();
            },
            "next" => {
                let mut app = self.app.lock().unwrap();
                app.proc_list.state.select_next();
            },
            "set" => {
                let mut app = self.app.lock().unwrap();
                app.proc_list.list = list.unwrap();
            },
            _ => {
                
            }
        };
    }

    //getters

    pub fn get_process(&self) -> i32 {
        let app = self.app.lock().unwrap();
        app.open_process
    }
    pub fn get_current_screen(&self) -> CurrentScreen {
        let app = self.app.lock().unwrap();
        app.current_screen.clone()
    }
    pub fn get_query(&self) -> (i32, String) {
        let app = self.app.lock().unwrap();
        app.query.clone()
    }
    pub fn get_bounds(&self) -> ((i32, String), (i32, String)) {
        let app = self.app.lock().unwrap();
        app.bounds.clone()
    }
    pub fn get_query_results(&self) -> Vec<(String, String)> {
        let app = self.app.lock().unwrap();
        app.query_results.clone()
    }
    pub fn get_query_progress(&self) -> f64 {
        let app = self.app.lock().unwrap();
        app.query_progress
    }
    pub fn get_progress_msg(&self) -> String {
        let app = self.app.lock().unwrap();
        app.progress_msg.clone()
    }
    pub fn get_input_mode(&self) -> InputMode {
        let app = self.app.lock().unwrap();
        app.input_mode.clone()
    }
    pub fn get_scan_type(&self) -> ScanTypes {
        let app = self.app.lock().unwrap();
        app.scan_type.clone()
    }
    pub fn get_tx(&self) -> Sender<Data> {
        let app = self.app.lock().unwrap();
        app.tx.clone()
    }
    pub fn get_mem_view_list(&self) -> VList {
        let app = self.app.lock().unwrap();
        app.mem_view_list.clone()
    }
    pub fn get_proc_list(&self) -> VList {
        let app = self.app.lock().unwrap();
        app.proc_list.clone()
    }

}

impl App {
    pub fn new() -> AMApp {
        let (tx, rx): (Sender<Data>, Receiver<Data>) = mpsc::channel();
        let mut app = App {
            open_process: 0,
            current_screen: CurrentScreen::Main,
            proc_list: VList::new(),
            mem_view_list: VList::new(),
            query: (0, String::new()),
            bounds: ((18, String::from("0000000000000000")), (16, String::from("00007fffffffffff"))),
            input_mode: InputMode::Normal,
            query_results: Vec::new(),
            tx,
            rx,
            scan_type: ScanTypes::Exact,
            query_progress: 0.0,
            progress_msg: String::from("Query not started..."),
        };
        app.proc_list.state.select(Some(0)); // set a default value so the list renders properly
        app.mem_view_list.state.select(Some(0));
        AMApp { app: Arc::new(Mutex::new(app)) }
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
    let app = App::new();
    let res = run_app(&mut terminal, app);

    
    if let Ok(x) = res {

    }


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


fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: AMApp) -> io::Result<bool> {

    
    loop {
        terminal.draw(|f| ui(f, app.clone()))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            // todo: put key events into their own module, it's too cluttered in here
            match app.get_input_mode() {
                InputMode::Normal => match key.code {
                    KeyCode::Char('s') => {
                        if app.get_current_screen() != CurrentScreen::Main { continue };
                        app.modify_input_mode(InputMode::EditingQuery)
                    }
                    KeyCode::Char('t') => {
                        continue;
                    }
                    KeyCode::Char('b') => {
                        if app.get_current_screen() != CurrentScreen::Main { continue };
                        app.modify_input_mode(InputMode::EditingLowerBound);
                    }
                    KeyCode::Enter => {
                        
                        app.modify_input_mode(InputMode::Normal);
                        let lower_bound = usize::from_str_radix(&app.get_bounds().0.1, 16).unwrap();
                        let upper_bound = usize::from_str_radix(&app.get_bounds().1.1, 16).unwrap();
                        let tapp = app.clone();
                        // TODO: run this in a new thread somehow
                        /*let handler = thread::spawn(move || {
                            get_mem_from_query(upper_bound, lower_bound, tapp);
                        });
                        
                        handler.join().unwrap();*/
                        
                    },
                    _ => {}
                },
                InputMode::EditingQuery if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char(to_insert) => {
                        app.modify_query((app.get_query().0 + 1, format!("{}{}", app.get_query().1, to_insert)));
                    },
                    KeyCode::Backspace => {
                        // saturating sub for overflow error prevention
                        app.modify_query(((app.get_query().0 - 1).clamp(0, std::i32::MAX), app.get_query().1[..app.get_query().1.len().saturating_sub(1)].to_string()));
                    },
                    KeyCode::Esc => app.modify_input_mode(InputMode::Normal),
                    _ => {}
                },
                InputMode::EditingUpperBound | InputMode::EditingLowerBound if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Esc => {
                        app.modify_input_mode(InputMode::Normal);
                    },
                    KeyCode::Char(to_insert) => {
                        if let InputMode::EditingLowerBound = app.get_input_mode(){
                            app.modify_bounds(((app.get_bounds().0.0 + 1, format!("{}{}", app.get_bounds().0.1, to_insert)), app.get_bounds().1));
                        } else {
                            app.modify_bounds((app.get_bounds().0, (app.get_bounds().1.0 + 1, format!("{}{}", app.get_bounds().1.1, to_insert))));
                        }
                    },
                    KeyCode::Backspace => {
                        if let InputMode::EditingLowerBound = app.get_input_mode() {
                            app.modify_bounds((((app.get_bounds().0.0 - 1).clamp(0, std::i32::MAX), app.get_bounds().0.1[..app.get_bounds().0.1.len().saturating_sub(1)].to_string()), app.get_bounds().1));
                        } else {
                            app.modify_bounds((app.get_bounds().0,((app.get_bounds().1.0 - 1).clamp(0, std::i32::MAX), app.get_bounds().1.1[..app.get_bounds().1.1.len().saturating_sub(1)].to_string())));
                        }
                    },
                    KeyCode::Tab => { 
                        match app.get_input_mode() {
                        InputMode::EditingUpperBound => app.modify_input_mode(InputMode::EditingLowerBound),
                        InputMode::EditingLowerBound => app.modify_input_mode(InputMode::EditingUpperBound),
                        _ => {}
                        }
                    },
                    _ => {}
                },
                InputMode::EditingQuery | InputMode::EditingLowerBound | InputMode::EditingUpperBound => {}
            }

            match app.get_current_screen() {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('q') => {
                        if app.get_input_mode() == InputMode::EditingQuery 
                        || app.get_input_mode() == InputMode::EditingLowerBound
                        || app.get_input_mode() == InputMode::EditingUpperBound 
                        { continue; } // editing should ALWAYS take input priority
                        app.modify_current_screen(CurrentScreen::Exiting);
                    }
                    KeyCode::Char('p') => {
                        if app.get_input_mode() == InputMode::EditingQuery 
                        || app.get_input_mode() == InputMode::EditingLowerBound
                        || app.get_input_mode() == InputMode::EditingUpperBound 
                        { continue; }
                        app.modify_current_screen(CurrentScreen::SelectingProcess);
                    }
                    KeyCode::Char('j') | KeyCode::Up => {
                        app.modify_mem_view_list("prev", None);
                    }
                    KeyCode::Char('k') | KeyCode::Down => {
                        app.modify_mem_view_list("next", None);
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    KeyCode::Char('c') => {
                        app.modify_current_screen(CurrentScreen::Main);
                    }
                    _ => {}
                }
                CurrentScreen::SelectingProcess => match key.code {
                    KeyCode::Char('q') => {
                        app.modify_current_screen(CurrentScreen::Main);
                    }
                    KeyCode::Char('j') | KeyCode::Up => {
                        app.modify_proc_list("prev", None)
                    }
                    KeyCode::Char('k') | KeyCode::Down => {
                        app.modify_proc_list("next", None)
                    }
                    KeyCode::Char('c') => {
                        let processes = get_process_list();
                        let Some(idx) = app.get_proc_list().state.selected()
                        else { continue; };
                        app.modify_process(processes[idx].1 as i32);
                        app.modify_current_screen(CurrentScreen::Main);
                    }
                    _ => {}
                }
                _ => {}
            }
        }
    }
}

