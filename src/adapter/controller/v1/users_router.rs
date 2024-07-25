use crate::{
    adapter::controller::v1::handle_error::handle_error,
    domain::{
        domain_error::DomainError,
        user::{UnvalidatedUser, User},
    },
};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use http::status::StatusCode;
use tracing::info;

pub fn users_router() -> Router {
    Router::new().route("/users", get(get_user_list)).route(
        "/users/:id",
        get(get_user_by_id).post(create_user).delete(delete_user),
    )
}

async fn get_user_list() -> impl IntoResponse {
    let result: Result<Vec<User>, DomainError> = Ok(vec![]);

    match result {
        Ok(r) => Ok((StatusCode::OK, Json(r))),
        Err(e) => Err(handle_error(e)),
    }
}

async fn get_user_by_id(Path(id): Path<String>) -> impl IntoResponse {
    info!(request.id=?id, "get_user_by_id");
    let result: Result<User, DomainError> = Err(DomainError::ValidationError);

    match result {
        Ok(r) => Ok((StatusCode::OK, Json(r))),
        Err(e) => Err(handle_error(e)),
    }
}

async fn create_user(
    Query(req): Query<UnvalidatedUser>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    info!(
        request.id=?id,
        "create_user",
    );
    let result: Result<User, DomainError> = Err(DomainError::ValidationError);

    match result {
        Ok(r) => Ok((StatusCode::CREATED, Json(r))),
        Err(e) => Err(handle_error(e)),
    }
}

async fn delete_user(Path(id): Path<String>) -> impl IntoResponse {
    info!(reqeust.id=?id, "delete_user");
    let result: Result<User, DomainError> = Err(DomainError::ValidationError);

    match result {
        Ok(r) => Ok((StatusCode::NO_CONTENT, Json(r))),
        Err(e) => Err(handle_error(e)),
    }
}
