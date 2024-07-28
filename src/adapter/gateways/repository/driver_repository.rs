use crate::adapter::gateways::db_data::driver::DriverData;
use crate::application::repository::driver_repository::IDriverRepository;
use crate::domain::driver::driver::Driver;
use crate::domain::driver::driver_id::DriverId;
use crate::domain::value_object::email::Email;
use crate::domain::value_object::password::Password;
use crate::domain::value_object::point::Point;
use anyhow::Result;

#[derive(Debug)]
pub struct DriverRepository {}

impl IDriverRepository for DriverRepository {
    fn get(&self, id: DriverId) -> Result<Driver> {
        let driver = DriverData::new(
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
            "email".to_string(),
            "password".to_string(),
            0.0,
            0.0,
        );
        let driver = Driver::new(
            DriverId::try_from(driver.id)?,
            Email::try_from(driver.email)?,
            Password::try_from(driver.password)?,
            Point::new(driver.longitude, driver.latitude)?,
        );
        Ok(driver)
    }

    fn get_all(&self) -> Result<Vec<Driver>> {
        let driver = DriverData::new(
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
            "email".to_string(),
            "password".to_string(),
            0.0,
            0.0,
        );
        let driver = Driver::new(
            DriverId::try_from(driver.id)?,
            Email::try_from(driver.email)?,
            Password::try_from(driver.password)?,
            Point::new(driver.longitude, driver.latitude)?,
        );
        Ok(vec![driver])
    }

    fn save(&self, driver: &Driver) -> Result<()> {
        Ok(())
    }

    fn delete(&self, id: DriverId) -> Result<()> {
        Ok(())
    }
}
