use crate::domain::domain_error::DomainError;
use crate::domain::user::*;
use crate::usecase::repository::user_repository::UserRepository;

pub struct GetUserListInput {}

impl GetUserListInput {
    pub fn new() -> Result<Self, DomainError> {
        unimplemented!()
    }
}

pub struct GetUserListUsecase<R>
where
    R: UserRepository,
{
    user_repo: R,
}

impl<R> GetUserListUsecase<R>
where
    R: UserRepository,
{
    pub fn execute(&self, input: GetUserListInput) -> Result<Vec<User>, DomainError> {
        self.user_repo.getAll()
    }
}
