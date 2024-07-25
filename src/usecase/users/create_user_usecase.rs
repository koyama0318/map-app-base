use crate::domain::domain_error::DomainError;
use crate::domain::user::*;

pub struct CreateUserUsecase {}

impl CreateUserUsecase {
    pub fn execute() -> Result<User, DomainError> {
        unimplemented!()
    }
}
