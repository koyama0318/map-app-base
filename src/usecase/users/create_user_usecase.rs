use crate::domain::domain_error::DomainError;
use crate::domain::user::{self, *};
use crate::usecase::repository::user_repository::UserRepository;

pub struct CreateUserInput {
    pub unvalidated_user: UnvalidatedUser,
}

pub struct CreateUserUsecase<UR>
where
    UR: UserRepository,
{
    user_repo: UR,
}

impl<UR> CreateUserUsecase<UR>
where
    UR: UserRepository,
{
    pub fn execute(&self, input: CreateUserInput) -> Result<User, DomainError> {
        user::UnvalidatedUser::validate(input.unvalidated_user)
            .and_then(|validated_user| self.user_repo.)
    }
}
