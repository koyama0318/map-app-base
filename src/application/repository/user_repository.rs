use crate::domain::user::user::User;
use crate::domain::user::user_id::UserId;
use anyhow::Result;

pub trait UserRepositoryInterface {
    fn get(&self, id: UserId) -> Result<User>;
    fn get_all(&self) -> Result<Vec<User>>;
    fn save(&self, user: &User) -> Result<()>;
    fn delete(&self, id: UserId) -> Result<()>;
}
