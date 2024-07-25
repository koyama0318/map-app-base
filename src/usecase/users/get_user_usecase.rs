use crate::domain::domain_error::DomainError;
use crate::domain::user::*;

pub struct GetUserUsecase {}

impl GetUserUsecase {
    pub fn execute() -> Result<User, DomainError> {
        unimplemented!()
    }
}
