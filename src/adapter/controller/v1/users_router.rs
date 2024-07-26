use crate::adapter::controller::app_error::AppError;
use crate::adapter::gateways::user_repository::UserRepository;
use crate::application::usecase::users::create_user_usecase::{CreateUserInput, CreateUserUsecase};
use crate::application::usecase::users::delete_user_usecase::{DeleteUserInput, DeleteUserUsecase};
use crate::application::usecase::users::get_user_list_usecase::{
    GetUserListInput, GetUserListUsecase,
};
use crate::application::usecase::users::get_user_usecase::{GetUserInput, GetUserUsecase};
use crate::domain::user::user::UnvalidatedUser;
use crate::domain::user::user_id::UserId;
use axum::extract::Path;
use axum::response::{IntoResponse, Result};
use axum::routing::get;
use axum::{Json, Router};
use http::status::StatusCode;
use tracing::info;

pub fn users_router() -> Router {
    Router::new()
        .route("/", get(get_user_list).post(create_user))
        .route("/:id", get(get_user_by_id).delete(delete_user))
}

async fn get_user_list() -> Result<impl IntoResponse, AppError> {
    info!("get_user_list");

    let input = GetUserListInput::new();
    let user_repo = UserRepository {};
    let usecase = GetUserListUsecase::new(user_repo);

    let response = usecase.execute(input)?;
    Ok((StatusCode::OK, Json(response)))
}

async fn get_user_by_id(Path(id): Path<String>) -> Result<impl IntoResponse, AppError> {
    info!(request.id = id, "get_user_by_id");

    let input = GetUserInput::new(UserId::try_from(id)?);
    let user_repo = UserRepository {};
    let usecase = GetUserUsecase::new(user_repo);

    let response = usecase.execute(input)?;
    Ok((StatusCode::OK, Json(response)))
}

async fn create_user(Json(user): Json<UnvalidatedUser>) -> Result<impl IntoResponse, AppError> {
    info!(request.body =? user, "create_user");

    let input = CreateUserInput::new(user);
    let user_repo = UserRepository {};
    let usecase = CreateUserUsecase::new(user_repo);

    let response = usecase.execute(input)?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn delete_user(Path(id): Path<String>) -> Result<impl IntoResponse, AppError> {
    info!(request.id =? id, "delete_user");

    let input = DeleteUserInput::new(UserId::try_from(id)?);
    let user_repo = UserRepository {};
    let usecase = DeleteUserUsecase::new(user_repo);

    usecase.execute(input)?;
    Ok((StatusCode::NO_CONTENT, ""))
}
