use serde::Serialize;

#[derive(Debug)]
pub struct Password {
    value: String,
}

impl Password {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}

impl Serialize for Password {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.value())
    }
}
