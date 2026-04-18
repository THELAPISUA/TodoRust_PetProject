mod app;
mod input;
mod models;
mod storage;
mod ui;
use crate::input::headers::key;

use app::App;
use ui::{
    render::draw,
    terminal::{cleanup, init},
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = init();
    let mut app = App::new();
    loop {
        draw(&mut terminal, &mut app);
        let _ = key(&mut app);
        if app.is_closed {
            break;
        }
    }
    cleanup(&mut terminal);
    Ok(())
}
