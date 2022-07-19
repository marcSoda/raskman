//for different done and todo statues
#[derive(Clone, Debug)]
pub enum StatType {
    Done,
    Todo,
}

#[derive(Clone, Debug)]
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
