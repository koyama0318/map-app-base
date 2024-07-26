use super::repositoroy_error::RepositoryError;
use crate::domain::{user::*, user_id::UserId};
pub trait UserRepository {
    fn get(id: UserId) -> Result<User, RepositoryError>;
    fn get_all() -> Result<Vec<User>, RepositoryError>;
    fn save(user: User) -> Result<(), RepositoryError>;
    fn delete(id: UserId) -> Result<(), RepositoryError>;
}
