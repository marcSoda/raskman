use std::time::Instant;

use crate::app::{
    namespace::Namespace,
    group::Group,
    tag::Tag,
    note::Note,
    status::Status,
    parser,
};

#[derive(Clone, Debug)]
pub struct Task {
    pub uuid: String,
    pub text: String,
    pub groups: Option<Vec<Group>>,
    pub namespaces: Vec<Namespace>,
    pub notes: Option<Vec<Note>>,
    pub tags: Option<Vec<Tag>>,
    pub status: Option<Status>,
    pub created: Instant,
    pub finished: Option<Instant>,
    pub recur: Option<Vec<String>>,
    pub until: Option<Instant>,
}

impl Task {
    pub fn new(text_list: Vec<&str>) -> Self {
        parser::parse_task(text_list).unwrap() //todo: don't just unwrap
    }
}
