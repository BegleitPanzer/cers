use crate::ui::main::App;

pub fn get_mem_from_query(app: &mut App) {
    app.query_results = vec![
        (app.bounds.0.0.to_string(), app.bounds.0.1.to_string()),
        (app.bounds.1.0.to_string(), app.bounds.1.1.to_string()),
        (app.query.0.to_string(), app.query.1.to_string()),
    ];
}