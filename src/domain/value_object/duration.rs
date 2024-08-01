use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Duration {
    text: String,
    value: i32,
}

impl Duration {
    pub fn new(text: String, value: i32) -> Self {
        Self { text, value }
    }
}
