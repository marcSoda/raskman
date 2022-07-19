use crate::app::namespace::Namespace;

#[derive(Clone, Debug)]
pub struct Group {
    name: String,
    namespace: Namespace,
    // numTasks: u16,
    // numProg: u16,
    // numDone: u16,
    // type: GroupTypeEnum,
}

impl Group {
    pub fn new(name: String, namespace: Namespace) -> Self {
        Group {
            name,
            namespace,
        }
    }
}
