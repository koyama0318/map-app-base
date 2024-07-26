use crate::application::repository::driver_repository::DriverRepositoryInterface;
use crate::domain::driver::driver_id::DriverId;
use anyhow::Result;

pub struct DeleteDriverInput {
    pub id: DriverId,
}

impl DeleteDriverInput {
    pub fn new(id: DriverId) -> Self {
        Self { id }
    }
}

pub struct DeleteDriverUsecase<UR>
where
    UR: DriverRepositoryInterface,
{
    driver_repo: UR,
}

impl<UR> DeleteDriverUsecase<UR>
where
    UR: DriverRepositoryInterface,
{
    pub fn new(driver_repo: UR) -> Self {
        Self { driver_repo }
    }

    pub fn execute(&self, input: DeleteDriverInput) -> Result<()> {
        self.driver_repo.delete(input.id)
    }
}
