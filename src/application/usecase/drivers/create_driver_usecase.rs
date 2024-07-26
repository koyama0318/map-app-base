use crate::application::repository::driver_repository::IDriverRepository;
use crate::domain::driver::driver::{Driver, UnvalidatedDriver};
use anyhow::Result;

pub struct CreateDriverInput {
    unvalidated_driver: UnvalidatedDriver,
}

impl CreateDriverInput {
    pub fn new(unvalidated_driver: UnvalidatedDriver) -> Self {
        Self { unvalidated_driver }
    }
}

pub struct CreateDriverUsecase<UR>
where
    UR: IDriverRepository,
{
    driver_repo: UR,
}

impl<UR> CreateDriverUsecase<UR>
where
    UR: IDriverRepository,
{
    pub fn new(driver_repo: UR) -> Self {
        Self { driver_repo }
    }

    pub fn execute(&self, input: CreateDriverInput) -> Result<Driver> {
        UnvalidatedDriver::validate(input.unvalidated_driver).and_then(|driver| {
            self.driver_repo.save(&driver)?;
            Ok(driver)
        })
    }
}
