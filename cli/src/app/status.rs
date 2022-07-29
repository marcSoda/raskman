use serde::{Deserialize, Serialize};
//for different done and todo statues
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StatType {
    Done,
    Doing,
    Todo,
    None,
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

    pub fn undefined() -> Self {
        Status {
            name: " ".to_string(),
            stat_type: StatType::None,
        }
    }
}
