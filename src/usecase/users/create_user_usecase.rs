use crate::domain::domain_error::DomainError;
use crate::domain::user::{self, *};
use crate::usecase::repository::user_repository::UserRepository;

pub struct CreateUserInput {
    pub unvalidated_user: UnvalidatedUser,
}

pub struct CreateUserUsecase<R>
where
    R: UserRepository,
{
    user_repo: R,
}

impl<R> CreateUserUsecase<R>
where
    R: UserRepository,
{
    pub fn execute(&self, input: CreateUserInput) -> Result<User, DomainError> {
        user::UnvalidatedUser::validate(input.unvalidated_user).and_then(self.user_repo.save)
    }
}
