use crate::application::repository::user_repository::IUserRepository;
use crate::domain::user::user::User;
use anyhow::Result;

pub struct GetUserListInput {}

impl GetUserListInput {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct GetUserListUsecase<R>
where
    R: IUserRepository,
{
    user_repo: R,
}

impl<R> GetUserListUsecase<R>
where
    R: IUserRepository,
{
    pub fn new(user_repo: R) -> Self {
        Self { user_repo }
    }

    pub fn execute(&self, _input: GetUserListInput) -> Result<Vec<User>> {
        self.user_repo.get_all()
    }
}
