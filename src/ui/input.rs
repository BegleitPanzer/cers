use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::backend::components::get_process_list::get_process_list;

use super::main::{AMApp, CurrentScreen, Data, DataType, InputMode};

pub async fn handle_input(app: AMApp, key: KeyEvent) -> bool {
    
    // Input Mode-based key handling
    match app.get_input_mode().await {
        InputMode::Normal => match key.code {
            KeyCode::Char('s') => {
                if app.get_current_screen().await != CurrentScreen::Main { return false };
                app.modify_input_mode(InputMode::EditingQuery).await
            }
            KeyCode::Char('t') => {
                return false;
            }
            KeyCode::Char('b') => {
                if app.get_current_screen().await != CurrentScreen::Main { return false };
                app.modify_input_mode(InputMode::EditingLowerBound).await;
            }
            KeyCode::Enter => {
                if app.get_process().await == 0 { app.modify_progress_msg("Please select a process.".to_string()).await; return false; }
                if app.get_query().await.1.is_empty() { app.modify_progress_msg("Please enter a query.".to_string()).await; return false; }
                if let Err(x) = app.get_tx().await.send(Data { data_type: DataType::BeginMemoryScan, data: String::new() })
                { app.modify_progress_msg(format!("Error sending data: {}", x)).await; }
                else {}

            },
            _ => {}
        },
        InputMode::EditingQuery if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char(to_insert) => {
                app.modify_query((app.get_query().await.0 + 1, format!("{}{}", app.get_query().await.1, to_insert))).await;
            },
            KeyCode::Backspace => {
                // saturating sub for overflow error prevention
                app.modify_query(((app.get_query().await.0 - 1).clamp(0, std::i32::MAX), app.get_query().await.1[..app.get_query().await.1.len().saturating_sub(1)].to_string())).await;
            },
            KeyCode::Esc => app.modify_input_mode(InputMode::Normal).await,
            _ => {}
        },
        InputMode::EditingUpperBound | InputMode::EditingLowerBound if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Esc => {
                app.modify_input_mode(InputMode::Normal).await;
            },
            KeyCode::Char(to_insert) => {
                if let InputMode::EditingLowerBound = app.get_input_mode().await {
                    app.modify_bounds(((app.get_bounds().await.0.0 + 1, format!("{}{}", app.get_bounds().await.0.1, to_insert)), app.get_bounds().await.1)).await;
                } else {
                    app.modify_bounds((app.get_bounds().await.0, (app.get_bounds().await.1.0 + 1, format!("{}{}", app.get_bounds().await.1.1, to_insert)))).await;
                }
            },
            KeyCode::Backspace => {
                if let InputMode::EditingLowerBound = app.get_input_mode().await {
                    app.modify_bounds((((app.get_bounds().await.0.0 - 1).clamp(0, std::i32::MAX), app.get_bounds().await.0.1[..app.get_bounds().await.0.1.len().saturating_sub(1)].to_string()), app.get_bounds().await.1)).await;
                } else {
                    app.modify_bounds((app.get_bounds().await.0,((app.get_bounds().await.1.0 - 1).clamp(0, std::i32::MAX), app.get_bounds().await.1.1[..app.get_bounds().await.1.1.len().saturating_sub(1)].to_string()))).await;
                }
            },
            KeyCode::Tab => { 
                match app.get_input_mode().await {
                InputMode::EditingUpperBound => app.modify_input_mode(InputMode::EditingLowerBound).await,
                InputMode::EditingLowerBound => app.modify_input_mode(InputMode::EditingUpperBound).await,
                _ => {}
                }
            },
            _ => {}
        },
        InputMode::EditingQuery | InputMode::EditingLowerBound | InputMode::EditingUpperBound => {}
    }

    match app.get_current_screen().await {
        CurrentScreen::Main => match key.code {
            KeyCode::Char('q') => {
                if app.get_input_mode().await == InputMode::EditingQuery 
                || app.get_input_mode().await == InputMode::EditingLowerBound
                || app.get_input_mode().await == InputMode::EditingUpperBound 
                { return false; } // editing should ALWAYS take input priority
                app.modify_current_screen(CurrentScreen::Exiting).await;
            }
            KeyCode::Char('p') => {
                if app.get_input_mode().await == InputMode::EditingQuery 
                || app.get_input_mode().await == InputMode::EditingLowerBound
                || app.get_input_mode().await == InputMode::EditingUpperBound 
                { return false; }
                app.modify_current_screen(CurrentScreen::SelectingProcess).await;
            }
            KeyCode::Char('j') | KeyCode::Up => {
                app.modify_mem_view_list("prev", None).await;
            }
            KeyCode::Char('k') | KeyCode::Down => {
                app.modify_mem_view_list("next", None).await;
            }
            _ => {}
        },
        CurrentScreen::Exiting => match key.code {
            KeyCode::Char('y') | KeyCode::Char('q') => {
                return true;
            }
            KeyCode::Char('c') => {
                app.modify_current_screen(CurrentScreen::Main).await;
            }
            _ => {}
        }
        CurrentScreen::SelectingProcess => match key.code {
            KeyCode::Char('q') => {
                app.modify_current_screen(CurrentScreen::Main).await;
            }
            KeyCode::Char('j') | KeyCode::Up => {
                app.modify_proc_list("prev", None).await;
            }
            KeyCode::Char('k') | KeyCode::Down => {
                app.modify_proc_list("next", None).await;
            }
            KeyCode::Char('c') => {
                let processes = get_process_list();
                let Some(idx) = app.get_proc_list().await.state.selected()
                else { return false; };
                app.modify_process(processes[idx].1 as i32).await;
                app.modify_current_screen(CurrentScreen::Main).await;
            }
            _ => {}
        }
    }
    return false;
}