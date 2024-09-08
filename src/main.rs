use std::process::exit;
use backend::components::get_mem_from_query::get_mem_from_query;
use ratatui;
use ui::main::{App, DataType};

mod backend;
mod ui;


#[tokio::main]
async fn main() {
    let app = App::new();

    tokio::spawn({
        let app = app.clone();
        let recv = app.app.lock().await.rx.clone();
        let bounds = app.get_bounds().await;
        async move {
            while let Some(msg) = recv.recv().ok() {
                match msg.data_type {
                    DataType::BeginMemoryScan => {
                        let lower_bound = usize::from_str_radix(&bounds.0 .1, 16).unwrap();
                        let upper_bound = usize::from_str_radix(&bounds.1 .1, 16).unwrap();
                        get_mem_from_query(upper_bound, lower_bound, app.clone()).await;
                    }
                }
            }
        }
    });

    color_eyre::install().unwrap();
    let mut terminal = ratatui::init();
    let app_result = ui::main::run_app(&mut terminal, app.clone()).await;
    ratatui::restore();
    if let Ok(_) = app_result {
        exit(0);
    }
}
