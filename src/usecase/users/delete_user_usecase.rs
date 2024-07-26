use crate::domain::domain_error::DomainError;
use crate::domain::user_id::UserId;
use crate::usecase::repository::user_repository::UserRepository;

pub struct DeleteUserInput {
    pub id: UserId,
}

pub struct DeleteUserUsecase<R>
where
    R: UserRepository,
{
    user_repo: R,
}

impl<R> DeleteUserUsecase<R>
where
    R: UserRepository,
{
    pub fn execute(&self, input: DeleteUserInput) -> Result<(), DomainError> {
        self.user_repo.delete(input.id)
    }
}
