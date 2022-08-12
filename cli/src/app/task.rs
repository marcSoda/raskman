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

impl Task {
    pub fn tags_contains(&self, tok: String) -> bool {
        let (is_inverted, tok) = is_inverted(tok);

        if is_inverted {
            for tag in self.tags.iter() {
                if tag.name == tok { return false }
            }
            true
        } else {
            for tag in self.tags.iter() {
                if tag.name == tok { return true }
            }
            false
        }
    }

    pub fn groups_contains(&self, tok: String) -> bool {
        let (is_inverted, tok) = is_inverted(tok);

        if is_inverted {
            for group in self.groups.iter() {
                if group.name == tok { return false }
            }
            true
        } else {
            for group in self.groups.iter() {
                if group.name == tok { return true }
            }
            false
        }
    }

    pub fn stat_is(&self, tok: String) -> bool {
        let (is_inverted, tok) = is_inverted(tok);
        if is_inverted {
            return self.status.name != tok;
        }
        self.status.name == tok
    }
}

fn is_inverted(mut tok: String) -> (bool, String) {
    if tok.contains('-') {
        tok = tok.replace("-", "");
        return (true, tok);
    }
    (false, tok)
}
