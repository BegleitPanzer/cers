use std::{io, process::exit, ptr::NonNull, thread};

use backend::components::get_mem_from_query::get_mem_from_query;
use crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use futures::executor::block_on;
use ratatui::{prelude::CrosstermBackend, Terminal};
use ui::main::{App, DataType};


mod ui;
mod backend;

#[tokio::main]
async fn main() {
    let app = App::new();

    // spawn the thread for terminal rendering
    tokio::spawn({let app = app.clone(); async move {
        let _ = enable_raw_mode();

        let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
        let _ = execute!(stderr, EnterAlternateScreen, EnableMouseCapture);
        let backend = CrosstermBackend::new(stderr);
        let mut terminal = Terminal::new(backend).unwrap();

        let res = ui::main::run_app(&mut terminal, app);
         
        if let Err(e) = res.await {
            let _ = disable_raw_mode();
            let _ = execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            );
            let _ = terminal.show_cursor();
            panic!("An error occurred while running the application");
        }
        else {
            let _ = disable_raw_mode();
            let _ = execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            );
            let _ = terminal.show_cursor();
            exit(0);
        };

      }});
      // spawn the thread for data reception over threads
      tokio::spawn({let app = app.clone(); async move {
        while let Some(msg) = app.app.lock().await.rx.recv().await {
            match msg.data_type {
                DataType::BeginMemoryScan => {
                    let lower_bound = usize::from_str_radix(&app.get_bounds().await.0.1, 16).unwrap();
                    let upper_bound = usize::from_str_radix(&app.get_bounds().await.1.1, 16).unwrap();
                    get_mem_from_query(upper_bound, lower_bound, app.clone()).await;
                },
                _ => {}
            }
        }
      }});
}