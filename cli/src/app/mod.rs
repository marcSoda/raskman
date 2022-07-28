use serde::{Deserialize, Serialize};
use prettytable::{
    Table,
    format,
};

pub mod task;
pub mod group;
pub mod namespace;
pub mod tag;
pub mod note;
pub mod status;
pub mod parser;
pub mod errors;

use task::Task;
use namespace::Namespace;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rask {
    // login: String, use when integration authentication
    // pass: String,
    pub task_list: Option<Vec<Task>>,
    pub current_namespace: Namespace,
}

impl Rask {
    pub fn new() -> Self {
        Rask {
            task_list: None,
            current_namespace: Namespace::default(),
        }
    }

    //todo: maybe ret result??
    pub fn disp(&self) {
        print!("Namespace: {}", self.current_namespace.name);
        // Create the table
        let mut table = Table::new();
        let format = format::FormatBuilder::new()
            .column_separator('|')
            .padding(1, 1)
            .build();
        table.set_format(format);

        table.set_titles(row![uFg->"ID", u->"Group", u->"Tags", u->"Description"]);

        let mut ctr = 0;
        for task in self.task_list.iter().flatten() {
            ctr+=1;
            let mut groups: String = "".to_string();
            for group in task.groups.iter().flatten() {
                groups += &group.name;
                groups += " ";
            }

            let mut tags: String = "".to_string();
            for tag in task.tags.iter().flatten() {
                tags += &tag.name;
                tags += " ";
            }
            table.add_row(row![Fg->ctr, groups, tags, task.description]);
        }
        println!();
        table.printstd();
        println!();
    }
}
