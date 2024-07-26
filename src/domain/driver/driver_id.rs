use anyhow::{
    bail,
    Result,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
pub struct DriverId(pub String);

impl TryFrom<String> for DriverId {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        if Uuid::parse_str(&value).is_ok() {
            Ok(DriverId(value))
        } else {
            bail!("Invalid UUID format")
        }
    }
}

impl Default for DriverId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}
