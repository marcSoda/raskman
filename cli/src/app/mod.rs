use serde::{Deserialize, Serialize};
use colored::*;
use prettytable::{
    Table,
    format::*,
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
use errors::TaskNotFoundError;
use namespace::Namespace;

use crate::app::status::Status;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rask {
    // login: String, use when integration authentication
    // pass: String,
    pub task_list: Vec<Task>,
    pub current_namespace: Namespace,
    pub needs_saving: bool, //used to indicate that the task_list needs to be saved to disk
}

//todo: for list filtering, add a filter function that filters all of the tasks based on a
//a filterby enum which is either namespace, status, group, duedate(date), etc
//todo extend: when ls is entered with args, have the filter function assign temporary ids and order by them

impl Rask {
    pub fn new() -> Self {
        Rask {
            task_list: vec![],
            current_namespace: Namespace::default(),
            needs_saving: false,
        }
    }

    //get a mutable reference to a task in self.task_list
    fn get_mut_task_by_id(&mut self, task_index: u16) -> Result<&mut Task, TaskNotFoundError> {
        for t in self.task_list.iter_mut() {
            if t.index == task_index { return Ok(t); }
        }
        Err(TaskNotFoundError(task_index))
    }

    pub fn update_status(&mut self, task_index: u16, new_status: Status) -> Result<(), TaskNotFoundError> {
        let t = self.get_mut_task_by_id(task_index)?;
        let old_stat_name_1 = "%".to_string() + &t.status.name;
        let old_stat_name_2 = "s:".to_string() + &t.status.name;
        let new_stat_name_1 = "%".to_string() + &new_status.name;
        let new_stat_name_2 = "s:".to_string() + &new_status.name;
        t.description = t.description.replace(&old_stat_name_1, &new_stat_name_1);
        t.description = t.description.replace(&old_stat_name_2, &new_stat_name_2);
        t.status = new_status;
        Ok(())
    }

    //todo: maybe ret result??
    pub fn disp(&mut self) {
        if self.task_list.is_empty() {
            println!("No tasks to list");
            return;
        }
        // Create the table
        let mut table = Table::new();
        let format = FormatBuilder::new()
                             .column_separator('│')
                             .borders('│')
                             .separators(&[LinePosition::Top], LineSeparator::new('─', '┬', '┌', '┐'))
                             .separators(&[LinePosition::Title], LineSeparator::new('─', '┼', '├', '┤'))
                             .separators(&[LinePosition::Bottom], LineSeparator::new('─', '┴', '└', '┘'))
                             .padding(1, 1)
                             .build();
        table.set_format(format);

        // table.set_titles(row![cbFy->"ID", cb->"Description", cbFr->"Status", cbFb->"Group", cbFg->"Tags"]);
        table.set_titles(row!["ID".yellow(), "Description", "Status".red(), "Group".blue(), "Tags".green()]);


        for task in self.task_list.iter_mut() {
            let mut groups: String = "".to_string();
            for group in task.groups.iter() {
                groups += &group.name;
                groups += " ";
            }

            let mut tags: String = "".to_string();
            for tag in task.tags.iter() {
                tags += &tag.name;
                tags += " ";
            }

            // let stat_name = task.status.clone().unwrap_or_else(Status::undefined).name;
            let stat_name = task.status.clone().name;

            table.add_row(row![Fy->task.index, colorize(task.description.clone()), stat_name, groups, tags]);
        }
        println!("\nNamespace: {}", self.current_namespace.name.blue());
        table.printstd();
        println!();
    }
}

//color string documentation: https://docs.rs/embedded-text/0.4.0/embedded_text/style/index.html
fn colorize(s: String) -> String {
    s.trim()
        .split_whitespace()
        .map(|tok| {
            let first = tok.chars().next().unwrap_or(' ');

            match first {
                '+' => tok.green().to_string(),
                '%' => tok.red().to_string(),
                '@' => tok.blue().to_string(),
                _ => {
                    if tok.contains(':') {
                        let split: Vec<&str> = tok.split(':').collect();
                        if split.len() == 2 {
                            match split[0] {
                                "t" => tok.green().to_string(),
                                "s" => tok.red().to_string(),
                                "g" => tok.blue().to_string(),
                                _ => tok.to_string(),
                            }
                        } else {
                            tok.to_string()
                        }
                    } else {
                        tok.to_string()
                    }
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
