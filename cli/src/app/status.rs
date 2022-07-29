use serde::{Deserialize, Serialize};
//for different done and todo statues
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StatType {
    Todo,
    Done,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    pub name: String,
    pub stat_type: StatType,
}

impl Status {
    pub fn new(name: String, stat_type: StatType) -> Self {
        Status {
            name,
            stat_type,
        }
    }

    //TODO: import default status info from raskrc
    pub fn default() -> Self {
        Status {
            name: "todo".to_string(),
            stat_type: StatType::Todo,
        }
    }
}
