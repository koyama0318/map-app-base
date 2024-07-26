use super::user_id::UserId;
use crate::domain::value_object::email::Email;
use crate::domain::value_object::password::Password;
use crate::domain::value_object::point::Point;
use anyhow::Result;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UnvalidatedUser {
    email: String,
    password: String,
    longitude: f64,
    latitude: f64,
}

impl UnvalidatedUser {
    pub fn validate(user: UnvalidatedUser) -> Result<User> {
        let point = Point::new(user.longitude, user.latitude)?;
        Ok(User::new(
            UserId::default(),
            Email::new(user.email),
            Password::new(user.password),
            point,
        ))
    }
}

#[derive(Serialize)]
pub struct User {
    id: UserId,
    email: Email,
    password: Password,
    point: Point,
}

impl User {
    pub fn new(id: UserId, email: Email, password: Password, point: Point) -> Self {
        User {
            id,
            email,
            password,
            point,
        }
    }
}
