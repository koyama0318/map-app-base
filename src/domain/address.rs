use super::domain_error::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    longitude: f64,
    latitude: f64,
}

impl Address {
    pub fn new(longitude: f64, latitude: f64) -> Result<Self, DomainError> {
        if 0.0 <= longitude && longitude <= 180.0 && -90.0 <= latitude && latitude <= 90.0 {
            Ok(Self {
                longitude,
                latitude,
            })
        } else {
            Err(DomainError::ValidationError)
        }
    }

    pub fn longitude(self) -> f64 {
        self.longitude
    }

    pub fn latitude(self) -> f64 {
        self.latitude
    }
}
