use crate::domain::domain_error::DomainError;
use crate::domain::user::*;
use crate::domain::user_id::UserId;
use crate::usecase::repository::user_repository::UserRepository;

pub struct GetUserInput {
    pub id: UserId,
}

pub struct GetUserUsecase<R: UserRepository> {
    user_repo: R,
}

impl<R> GetUserUsecase<R>
where
    R: UserRepository,
{
    pub fn execute(&self, input: GetUserInput) -> Result<User, DomainError> {
        self.user_repo.get(id)
    }
}
