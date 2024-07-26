use crate::application::repository::driver_repository::DriverRepositoryInterface;
use crate::domain::driver::driver::Driver;
use crate::domain::driver::driver_id::DriverId;
use anyhow::Result;

#[derive(Debug)]
pub struct DriverRepo {}

impl DriverRepositoryInterface for DriverRepo {
    fn get(&self, id: DriverId) -> Result<Driver> {
        todo!()
    }

    fn get_all(&self) -> Result<Vec<Driver>> {
        todo!()
    }

    fn save(&self, driver: &Driver) -> Result<()> {
        todo!()
    }

    fn delete(&self, id: DriverId) -> Result<()> {
        todo!()
    }
}
