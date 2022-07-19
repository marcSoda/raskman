use std::time::Instant; //maybe just use date instead of instant

#[derive(Clone, Debug)]
pub struct Note {
    text: String,
    created: Instant,
}

impl Note {
    pub fn new(text_list: Vec<&str>) -> Self {
        let text = text_list.join(" ");
        Note {
            text,
            created: Instant::now(),
        }

    }
}
