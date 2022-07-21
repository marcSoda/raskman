use serde::{Deserialize, Serialize};
//for different done and todo statues
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StatType {
    Done,
    Todo,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    name: String,
    stat_type: StatType,
}

impl Status {
    pub fn new(name: String, stat_type: StatType) -> Self {
        Status {
            name,
            stat_type,
        }
    }
}
