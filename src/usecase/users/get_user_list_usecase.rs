use crate::domain::domain_error::DomainError;
use crate::domain::user::*;

pub struct GetUserListUsecase {}

impl GetUserListUsecase {
    pub fn execute() -> Result<Vec<User>, DomainError> {
        unimplemented!()
    }
}
