use crate::application::repository::user_repository::UserRepositoryInterface;
use crate::domain::user::user::{
    UnvalidatedUser,
    User,
};
use anyhow::Result;

pub struct CreateUserInput {
    unvalidated_user: UnvalidatedUser,
}

impl CreateUserInput {
    pub fn new(unvalidated_user: UnvalidatedUser) -> Self {
        Self { unvalidated_user }
    }
}

pub struct CreateUserUsecase<UR>
where
    UR: UserRepositoryInterface,
{
    user_repo: UR,
}

impl<UR> CreateUserUsecase<UR>
where
    UR: UserRepositoryInterface,
{
    pub fn new(user_repo: UR) -> Self {
        Self { user_repo }
    }

    pub fn execute(&self, input: CreateUserInput) -> Result<User> {
        UnvalidatedUser::validate(input.unvalidated_user).and_then(|user| {
            self.user_repo.save(&user)?;
            Ok(user)
        })
    }
}
