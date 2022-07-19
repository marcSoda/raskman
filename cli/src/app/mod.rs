pub mod task;
pub mod group;
pub mod namespace;
pub mod tag;
pub mod note;
pub mod status;
pub mod parser;
pub mod errors;

use task::Task;

#[derive(Debug, Clone)]
pub struct Rask {
    // login: String, use when integration authentication
    // pass: String,
    pub task_list: Option<Vec<Task>>,
    //status_list: Vec<Status>,
}

impl Rask {
    pub fn new() -> Self {
        Rask {
            task_list: None,
        }
    }
}
