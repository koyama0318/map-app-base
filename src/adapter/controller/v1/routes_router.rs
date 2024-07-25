use crate::adapter::controller::v1::handle_error::handle_error;
use crate::domain::domain_error::*;
use crate::domain::route::Route;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use http::status::StatusCode;

pub fn routes_router() -> Router {
    Router::new().route("/routes", get(get_routes))
}

async fn get_routes() -> impl IntoResponse {
    let result: Result<Route, DomainError> = Err(DomainError::ValidationError);

    match result {
        Ok(r) => Ok((StatusCode::OK, Json(r))),
        Err(e) => Err(handle_error(e)),
    }
}
