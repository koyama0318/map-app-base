use super::driver_id::DriverId;
use crate::domain::value_object::email::Email;
use crate::domain::value_object::password::Password;
use crate::domain::value_object::point::Point;
use anyhow::Result;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UnvalidatedDriver {
    email: String,
    password: String,
    longitude: f64,
    latitude: f64,
}

impl UnvalidatedDriver {
    pub fn validate(driver: UnvalidatedDriver) -> Result<Driver> {
        let address = Point::new(driver.longitude, driver.latitude)?;
        Ok(Driver::new(
            DriverId::default(),
            Email::new(driver.email),
            Password::new(driver.password),
            address,
        ))
    }
}

#[derive(Serialize)]
pub struct Driver {
    id: DriverId,
    email: Email,
    password: Password,
    point: Point,
}

impl Driver {
    fn new(id: DriverId, email: Email, password: Password, point: Point) -> Self {
        Driver {
            id,
            email,
            password,
            point,
        }
    }
}
