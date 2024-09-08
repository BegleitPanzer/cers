

use std::ops::Range;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::io;
use crossbeam_channel::{Receiver, Sender};
use super::input;
use super::rendering::ui;
use crossterm::event::{self, Event};
use ratatui::prelude::Backend;
use ratatui::widgets::{List, ListState};
use ratatui::Terminal;

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

#[derive(Debug)]
pub enum DataType {
    BeginMemoryScan,
}

#[derive(Debug)]
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
    pub query_results: Vec<String>,
    pub query_progress: f64,
    pub querying: bool,
    pub tx: Sender<Data>,
    pub rx: Receiver<Data>,
    pub progress_msg : Vec<String>,
    pub input_mode: InputMode,
    pub scan_type: ScanTypes,

}

#[derive(Debug, Clone)]
pub struct AMApp {
    pub app: Arc<Mutex<App>>
}

impl AMApp {
    //setters
    pub async fn modify_process(&self, process: i32) {
        let mut app = self.app.lock().await;
        app.open_process = process;
    }
    pub async fn modify_current_screen(&self, screen: CurrentScreen) {
        let mut app = self.app.lock().await;
        app.current_screen = screen;
    }
    pub async fn modify_query(&self, query: (i32, String)) {
        let mut app = self.app.lock().await;
        app.query = query;
    }
    pub async fn modify_bounds(&self, bounds: ((i32, String), (i32, String))) {
        let mut app = self.app.lock().await;
        app.bounds = bounds;
    }
    pub async fn modify_query_results(&self, results: Vec<usize>) {
        let mut app = self.app.lock().await;
        app.query_results = results.into_iter().map(|p| format!("{:#X}", p)).collect();
    }
    pub async fn modify_query_progress(&self, progress: f64) {
        let mut app = self.app.lock().await;
        app.query_progress = progress;
    }
    pub async fn modify_progress_msg(&self, msg: String) {
        let mut app = self.app.lock().await;
        app.progress_msg.push(msg.clone());
    }
    pub async fn modify_input_mode(&self, mode: InputMode) {
        let mut app = self.app.lock().await;
        app.input_mode = mode;
    }
    pub async fn modify_scan_type(&self, scan_type: ScanTypes) {
        let mut app = self.app.lock().await;
        app.scan_type = scan_type;
    }
    pub async fn modify_mem_view_list(&self, action: &str, list: Option<List<'static>>) {
        match action {
            "prev" => {
                let mut app = self.app.lock().await;
                app.mem_view_list.state.select_previous();
            },
            "next" => {
                let mut app = self.app.lock().await;
                app.mem_view_list.state.select_next();
            },
            "set" => {
                let mut app = self.app.lock().await;
                app.mem_view_list.list = list.unwrap();
            },
            "reset" => {
                let mut app = self.app.lock().await;
                app.mem_view_list = VList::new();
            }
            _ => {
                
            }
        };
    }
    pub async fn modify_querying(&self, querying: bool) {
        let mut app = self.app.lock().await;
        app.querying = querying;
    }

    pub async fn modify_proc_list(&self, action: &str, list: Option<List<'static>>) {
        match action {
            "prev" => {
                let mut app = self.app.lock().await;
                app.proc_list.state.select_previous();
            },
            "next" => {
                let mut app = self.app.lock().await;
                app.proc_list.state.select_next();
            },
            "set" => {
                let mut app = self.app.lock().await;
                app.proc_list.list = list.unwrap();
            },
            _ => {
                
            }
        };
    }

    //getters

    pub async fn get_process(&self) -> i32 {
        let app = self.app.lock().await;
        app.open_process
    }
    pub async fn get_current_screen(&self) -> CurrentScreen {
        let app = self.app.lock().await;
        app.current_screen.clone()
    }
    pub async fn get_query(&self) -> (i32, String) {
        let app = self.app.lock().await;
        app.query.clone()
    }
    pub async fn get_bounds(&self) -> ((i32, String), (i32, String)) {
        let app = self.app.lock().await;
        app.bounds.clone()
    }
    pub async fn get_query_results(&self, mut range: Range<usize>) -> Vec<String> {
        let app = self.app.lock().await;
        range.end = range.end.clamp(0, app.query_results.len());
        app.query_results.clone()[range].to_vec()
    }
    pub async fn get_query_result_count(&self) -> usize {
        let app = self.app.lock().await;
        app.query_results.len()
    }
    pub async fn get_query_progress(&self) -> f64 {
        let app = self.app.lock().await;
        app.query_progress
    }
    pub async fn get_progress_msg(&self) -> Vec<String> {
        let app = self.app.lock().await;
        app.progress_msg.clone()
    }
    pub async fn get_input_mode(&self) -> InputMode {
        let app = self.app.lock().await;
        app.input_mode.clone()
    }
    pub async fn get_scan_type(&self) -> ScanTypes {
        let app = self.app.lock().await;
        app.scan_type.clone()
    }
    pub async fn get_tx(&self) -> Sender<Data> {
        let app = self.app.lock().await;
        app.tx.clone()
    }
    pub async fn get_mem_view_list(&self) -> VList {
        let app = self.app.lock().await;
        app.mem_view_list.clone()
    }
    pub async fn get_proc_list(&self) -> VList {
        let app = self.app.lock().await;
        app.proc_list.clone()
    }
    pub async fn get_querying(&self) -> bool {
        let app = self.app.lock().await;
        app.querying
    }

}

impl App {
    pub fn new() -> AMApp {
        let (tx, rx): (Sender<Data>, Receiver<Data>) = crossbeam_channel::unbounded();
        let mut app = App {
            open_process: 0,
            current_screen: CurrentScreen::Main,
            proc_list: VList::new(),
            mem_view_list: VList::new(),
            query: (0, String::new()),
            bounds: ((18, String::from("0000000000000000")), (16, String::from("00007fffffffffff"))),
            input_mode: InputMode::Normal,
            query_results: Vec::new(),
            querying: false,
            tx,
            rx,
            scan_type: ScanTypes::Exact,
            query_progress: 0.0,
            progress_msg: vec![],
        };
        app.proc_list.state.select(Some(0)); // set a default value so the list renders properly
        app.mem_view_list.state.select(Some(0));
        AMApp { app: Arc::new(Mutex::new(app)) }
    }

}

pub async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: AMApp) -> io::Result<bool> {
    loop {
        let handle = tokio::runtime::Handle::current();
        tokio::task::block_in_place(|| {
            let x = terminal.draw(|f| {
                handle.block_on(ui(f, app.clone()))
            });
        });
        
        if !event::poll(std::time::Duration::from_millis(16))? { continue;}
        let Event::Key(key) = event::read()? else { continue };
        if key.kind == event::KeyEventKind::Release { continue; }
        
        let res = input::handle_input(app.clone(), key).await;
        if !res { continue; }
        else { return Ok(true) }   
    }
}

