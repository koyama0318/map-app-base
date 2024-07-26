use anyhow::{
    bail,
    Result,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
pub struct UserId(pub String);

impl TryFrom<String> for UserId {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        if Uuid::parse_str(&value).is_ok() {
            Ok(UserId(value))
        } else {
            bail!("Invalid UUID format")
        }
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}
