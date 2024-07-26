use crate::application::repository::user_repository::UserRepositoryInterface;
use crate::domain::user::user_id::UserId;
use anyhow::Result;

pub struct DeleteUserInput {
    pub id: UserId,
}

impl DeleteUserInput {
    pub fn new(id: UserId) -> Self {
        Self { id }
    }
}

pub struct DeleteUserUsecase<UR>
where
    UR: UserRepositoryInterface,
{
    user_repo: UR,
}

impl<UR> DeleteUserUsecase<UR>
where
    UR: UserRepositoryInterface,
{
    pub fn new(user_repo: UR) -> Self {
        Self { user_repo }
    }

    pub fn execute(&self, input: DeleteUserInput) -> Result<()> {
        self.user_repo.delete(input.id)
    }
}
