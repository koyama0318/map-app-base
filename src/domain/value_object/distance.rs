use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Distance {
    text: String,
    value: i32,
}

impl Distance {
    pub fn new(text: String, value: i32) -> Self {
        Self { text, value }
    }
}
