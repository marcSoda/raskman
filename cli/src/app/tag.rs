use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    // numTasks: u16,
    // numProg: u16,
    // numDone: u16,
    // type: GroupTypeEnum,
}

impl Tag {
    pub fn new(name: String) -> Self {
        Tag {
            name,
        }
    }
}
