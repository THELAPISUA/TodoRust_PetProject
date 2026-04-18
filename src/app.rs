use crate::models::task::Task;
use crate::storage::file::load;
#[derive(PartialEq)]
pub enum AppState {
    Notshing,
    AddNewTask,
    Complete,
    Remove,
}
pub struct App {
    pub input: String,
    pub state: AppState,
    pub tasks: Vec<Task>,
    pub is_closed: bool,
}
impl App {
    pub fn new() -> Self {
        let input = String::new();
        let state = AppState::Notshing;
        let tasks = load("save.json").unwrap();
        Self {
            input,
            state,
            tasks,
            is_closed: false,
        }
    }
}
