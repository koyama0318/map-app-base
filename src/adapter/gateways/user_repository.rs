use crate::application::repository::user_repository::IUserRepository;
use crate::domain::user::user::User;
use crate::domain::user::user_id::UserId;
use anyhow::Result;

#[derive(Debug)]
pub struct UserRepository {}

impl IUserRepository for UserRepository {
    fn get(&self, id: UserId) -> Result<User> {
        todo!()
    }

    fn get_all(&self) -> Result<Vec<User>> {
        todo!()
    }

    fn save(&self, user: &User) -> Result<()> {
        todo!()
    }

    fn delete(&self, id: UserId) -> Result<()> {
        todo!()
    }
}
