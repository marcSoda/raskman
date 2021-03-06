use serde::{Deserialize, Serialize};
use chrono::{
    DateTime,
    Utc,
};

use crate::app::{
    group::Group,
    tag::Tag,
    note::Note,
    status::Status,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub uuid: String,
    pub index: u16,
    pub description: String,
    pub groups: Vec<Group>,
    pub notes: Vec<Note>,
    pub tags: Vec<Tag>,
    pub status: Status,
    pub due: Option<DateTime<Utc>>,
    pub created: DateTime<Utc>,
    pub finished: Option<DateTime<Utc>>,
    pub recur: Option<Vec<String>>,
    pub until: Option<DateTime<Utc>>,
}

//dispatcher directly calls parse_task, so this isn't needed. maybe change that?
// impl Task {
//     pub fn new(text_list: Vec<&str>) -> Self {
//         parser::parse_task(text_list).unwrap() //todo: don't just unwrap
//     }
// }
