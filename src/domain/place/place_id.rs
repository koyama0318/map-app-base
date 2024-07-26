use anyhow::{
    bail,
    Result,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
pub struct PlaceId(pub String);

impl TryFrom<String> for PlaceId {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        if Uuid::parse_str(&value).is_ok() {
            Ok(PlaceId(value))
        } else {
            bail!("Invalid UUID format")
        }
    }
}

impl Default for PlaceId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}
