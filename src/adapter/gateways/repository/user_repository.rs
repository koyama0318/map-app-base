use crate::adapter::gateways::db_data::user::UserData;
use crate::application::repository::user_repository::IUserRepository;
use crate::domain::user::user::User;
use crate::domain::user::user_id::UserId;
use crate::domain::value_object::email::Email;
use crate::domain::value_object::password::Password;
use crate::domain::value_object::point::Point;
use anyhow::Result;

#[derive(Debug)]
pub struct UserRepository {}

impl IUserRepository for UserRepository {
    fn get(&self, id: UserId) -> Result<User> {
        let user = UserData::new(
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
            "email".to_string(),
            "password".to_string(),
            0.0,
            0.0,
        );
        let user = User::new(
            UserId::try_from(user.id)?,
            Email::try_from(user.email)?,
            Password::try_from(user.password)?,
            Point::new(user.longitude, user.latitude)?,
        );
        Ok(user)
    }

    fn get_all(&self) -> Result<Vec<User>> {
        let user = UserData::new(
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
            "email".to_string(),
            "password".to_string(),
            0.0,
            0.0,
        );
        let user = User::new(
            UserId::try_from(user.id)?,
            Email::try_from(user.email)?,
            Password::try_from(user.password)?,
            Point::new(user.longitude, user.latitude)?,
        );
        Ok(vec![user])
    }

    fn save(&self, user: &User) -> Result<()> {
        Ok(())
    }

    fn delete(&self, id: UserId) -> Result<()> {
        Ok(())
    }
}
