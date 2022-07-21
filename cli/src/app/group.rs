use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    // numTasks: u16,
    // numProg: u16,
    // numDone: u16,
    // type: GroupTypeEnum,
}

impl Group {
    pub fn new(name: String) -> Self {
        Group {
            name,
        }
    }
}
