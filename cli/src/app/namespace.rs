use serde::{Deserialize, Serialize};
//maybe make definition an option
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Namespace {
    pub name: String,
    definition: String,
}

impl Namespace {
//     pub fn new(name: String, definition: String) -> Self {
//         Namespace {
//             name,
//             definition,
//             active: false,
//         }
//     }

    pub fn default() -> Self {
        Namespace {
            name: "default".to_string(),
            definition: "".to_string(),
        }
    }
}
