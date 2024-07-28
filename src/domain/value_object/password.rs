use anyhow::{Ok, Result};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Password(pub String);

impl TryFrom<String> for Password {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        Ok(Self(value))
    }
}

impl Serialize for Password {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}
