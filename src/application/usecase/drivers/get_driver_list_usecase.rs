use crate::application::repository::driver_repository::DriverRepositoryInterface;
use crate::domain::driver::driver::Driver;
use anyhow::Result;

pub struct GetDriverListInput {}

impl GetDriverListInput {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct GetDriverListUsecase<R>
where
    R: DriverRepositoryInterface,
{
    driver_repo: R,
}

impl<R> GetDriverListUsecase<R>
where
    R: DriverRepositoryInterface,
{
    pub fn new(driver_repo: R) -> Self {
        Self { driver_repo }
    }

    pub fn execute(&self, _input: GetDriverListInput) -> Result<Vec<Driver>> {
        self.driver_repo.get_all()
    }
}
