use crate::application::repository::driver_repository::DriverRepositoryInterface;
use crate::domain::driver::driver::Driver;
use crate::domain::driver::driver_id::DriverId;
use anyhow::Result;

pub struct GetDriverInput {
    id: DriverId,
}

impl GetDriverInput {
    pub fn new(id: DriverId) -> Self {
        Self { id }
    }
}

pub struct GetDriverUsecase<UR>
where
    UR: DriverRepositoryInterface,
{
    driver_repo: UR,
}

impl<UR> GetDriverUsecase<UR>
where
    UR: DriverRepositoryInterface,
{
    pub fn new(driver_repo: UR) -> Self {
        Self { driver_repo }
    }

    pub fn execute(&self, input: GetDriverInput) -> Result<Driver> {
        self.driver_repo.get(input.id)
    }
}
