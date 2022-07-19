#[derive(Clone, Debug)]
pub struct Tag {
    name: String,
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
