use serde::{Deserialize, Serialize};
use chrono::{
    DateTime,
    Utc,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Note {
    text: String,
    created: DateTime<Utc>,
}

// impl Note {
//     pub fn new(text_list: Vec<&str>) -> Self {
//         let text = text_list.join(" ");
//         Note {
//             text,
//             created: Utc::now(),
//         }

//     }
// }
