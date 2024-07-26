use crate::application::repository::user_repository::UserRepositoryInterface;
use crate::domain::user::user::User;
use crate::domain::user::user_id::UserId;
use anyhow::Result;

pub struct GetUserInput {
    id: UserId,
}

impl GetUserInput {
    pub fn new(id: UserId) -> Self {
        Self { id }
    }
}

pub struct GetUserUsecase<UR>
where
    UR: UserRepositoryInterface,
{
    user_repo: UR,
}

impl<UR> GetUserUsecase<UR>
where
    UR: UserRepositoryInterface,
{
    pub fn new(user_repo: UR) -> Self {
        Self { user_repo }
    }

    pub fn execute(&self, input: GetUserInput) -> Result<User> {
        self.user_repo.get(input.id)
    }
}
