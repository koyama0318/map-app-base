use super::domain_error::DomainError;
use serde::{Deserialize, Serialize};
use super::{address::Address, email::Email, password::Password};

#[derive(Debug, Serialize, Deserialize)]
pub struct UnvalidatedUser {
    email: String,
    password: String,
    longitude: f64,
    latitude: f64,
}

impl UnvalidatedUser {
    pub fn validate(user: UnvalidatedUser) -> Result<User, DomainError> {
        let address = Address::new(user.longitude, user.latitude)?;
        Ok(
            User::new(
                Email::new(user.email),
                Password::new(user.password),
                address,
            )
        )
    }
}

#[derive(Serialize)]
pub struct User {
    email: Email,
    password: Password,
    address: Address,
}

impl User {
    pub fn new(email: Email, password: Password, address: Address) -> Self {
        User { email, password, address }
    }
}
