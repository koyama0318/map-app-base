use crate::domain::driver::driver::Driver;
use crate::domain::driver::driver_id::DriverId;
use anyhow::Result;

pub trait IDriverRepository {
    fn get(&self, id: DriverId) -> Result<Driver>;
    fn get_all(&self) -> Result<Vec<Driver>>;
    fn save(&self, driver: &Driver) -> Result<()>;
    fn delete(&self, id: DriverId) -> Result<()>;
}
