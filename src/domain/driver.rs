use super::domain_error::DomainError;
use serde::{Deserialize, Serialize};
use super::{address::Address, email::Email, password::Password};

#[derive(Debug, Serialize, Deserialize)]
pub struct UnvalidatedDriver {
    email: String,
    password: String,
    longitude: f64,
    latitude: f64,
}

impl UnvalidatedDriver {
    pub fn validate(Driver: UnvalidatedDriver) -> Result<Driver, DomainError> {
        let address = Address::new(Driver.longitude, Driver.latitude)?;
        Ok(
            Driver::new(
                Email::new(Driver.email),
                Password::new(Driver.password),
                address,
            )
        )
    }
}

pub struct Driver {
    email: Email,
    password: Password,
    address: Address,
}

impl Driver {
    fn new(email: Email, password: Password, address: Address) -> Self {
        Driver { email, password, address }
    }
}
