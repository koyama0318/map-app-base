use super::domain_error::DomainError;
use serde::{Deserialize, Serialize};
use super::{address::Address, email::Email, password::Password, user_id::UserId};

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
                UserId::default(),
                Email::new(user.email),
                Password::new(user.password),
                address,
            )
        )
    }
}

#[derive(Serialize)]
pub struct User {
    id: UserId,
    email: Email,
    password: Password,
    address: Address,
}

impl User {
    pub fn new(id: UserId, email: Email, password: Password, address: Address) -> Self {
        User { id, email, password, address }
    }
}
