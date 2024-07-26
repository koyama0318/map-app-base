use serde::Serialize;

#[derive(Debug)]
pub struct Email {
    value: String,
}

impl Email {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}

impl Serialize for Email {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.value())
    }
}
