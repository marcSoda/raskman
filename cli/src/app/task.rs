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
    pub description: String,
    pub groups: Option<Vec<Group>>,
    pub notes: Option<Vec<Note>>,
    pub tags: Option<Vec<Tag>>,
    pub status: Option<Status>,
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
