use super::domain_error::DomainError;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct UserId {
    id: String,
}

impl UserId {
    pub fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
        }
    }

    pub fn new(id: String) -> Result<Self, DomainError> {
        match uuid::Uuid::parse_str(&id) {
            Ok(_) => Ok(Self { id }),
            Err(_) => Err(DomainError::ValidationError),
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }
}
