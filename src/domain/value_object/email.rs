use anyhow::{Ok, Result};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Email(pub String);

impl TryFrom<String> for Email {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        Ok(Self(value))
    }
}

impl Serialize for Email {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}
