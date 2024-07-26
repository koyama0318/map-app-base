use crate::{
    adapter::controller::v1::handle_error::handle_error,
    domain::{
        domain_error::DomainError,
        user::{UnvalidatedUser, User},
    }, usecase::repository::user_repository::UserRepository,
};
use crate::{usecase::users::*}
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use create_user_usecase::{CreateUserInput, CreateUserUsecase};
use delete_user_usecase::{DeleteUserInput, DeleteUserUsecase};
use get_user_list_usecase::{GetUserListInput, GetUserListUsecase};
use get_user_usecase::GetUserInput;
use http::status::StatusCode;
use tracing::info;

pub fn users_router() -> Router {
    Router::new().route("/users", get(get_user_list)).route(
        "/users/:id",
        get(get_user_by_id).post(create_user).delete(delete_user),
    )
}

async fn get_user_list() -> impl IntoResponse {
    let user_repo = UserRepository {};
    let usecase = GetUserListUsecase { user_repo };

    let result = GetUserListInput::new()
        .and_then(usecase.execute);

    match result {
        Ok(r) => Ok((StatusCode::OK, Json(r))),
        Err(e) => Err(handle_error(e)),
    }
}

async fn get_user_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let input = GetUserInput { id };
    let user_repo = UserRepository {};
    let usecase = GetUserListUsecase { user_repo };

    let result = usecase.execute(input);

    match result {
        Ok(r) => Ok((StatusCode::OK, Json(r))),
        Err(e) => Err(handle_error(e)),
    }
}

async fn create_user(
    Query(user): Query<UnvalidatedUser>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let input = CreateUserInput { unvalidated_user: user };
    let user_repo = UserRepository {};
    let usecase = CreateUserUsecase { user_repo };

    let result = usecase.execute(input);

    match result {
        Ok(r) => Ok((StatusCode::CREATED, Json(r))),
        Err(e) => Err(handle_error(e)),
    }
}

async fn delete_user(Path(id): Path<String>) -> impl IntoResponse {
    let input = DeleteUserInput { id };
    let user_repo = UserRepository {};
    let usecase = DeleteUserUsecase { user_repo };

    let result = usecase.execute(input);

    match result {
        Ok(r) => Ok((StatusCode::NO_CONTENT, Json(r))),
        Err(e) => Err(handle_error(e)),
    }
}
