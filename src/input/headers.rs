use crate::app::{App, AppState};
use crate::models::task::Task;
use crate::storage::file::save;
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;
use std::time::Duration;

pub fn key(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    if event::poll(Duration::from_millis(200))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(k) => {
                    if app.state == AppState::AddNewTask {
                        app.input.push(k);
                    } else if app.state == AppState::Remove || app.state == AppState::Complete {
                        if let KeyCode::Char(c) = key.code {
                            if c.is_ascii_digit() {
                                let digit = c.to_digit(10).unwrap();
                                let ch: char = char::from_digit(digit, 10).unwrap();
                                app.input.push(ch);
                            }
                        }
                    }
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        app.input.clear();
                        if key.code == KeyCode::Char('q') {
                            let _ = save(&app.tasks);
                            app.is_closed = true
                        } else if key.code == KeyCode::Char('a') {
                            app.state = AppState::AddNewTask
                        } else if key.code == KeyCode::Char('r') {
                            app.state = AppState::Remove
                        } else if key.code == KeyCode::Char('c') {
                            app.state = AppState::Complete
                        }
                    }
                }

                KeyCode::Enter => match app.state {
                    AppState::AddNewTask => {
                        if app.input != "" {
                            let row = Task {
                                name: app.input.clone(),
                                status: false,
                            };
                            app.tasks.push(row);
                            app.input.clear();
                        }
                        app.state = AppState::Notshing;
                    }
                    AppState::Remove => {
                        if app.input != "" {
                            let ind: usize = app.input.parse().unwrap();
                            app.tasks.remove(ind);
                        }
                        app.state = AppState::Notshing;
                    }
                    AppState::Complete => {
                        let ind: usize = app.input.parse().unwrap();
                        if app.input != "" {
                            if let Some(item) = app.tasks.get_mut(ind) {
                                item.status = !item.status;
                            }
                        }
                        app.state = AppState::Notshing;
                    }
                    AppState::Notshing => {}
                },

                KeyCode::Backspace => {
                    app.input.pop();
                }

                _ => {}
            }
        }
    }
    Ok(())
}
