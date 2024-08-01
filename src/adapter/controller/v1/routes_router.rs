use anyhow::Result;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use http::status::StatusCode;
use serde::Deserialize;
use tracing::info;

use crate::adapter::controller::app_error::AppError;
use crate::adapter::gateways::repository::driver_repository::DriverRepository;
use crate::adapter::gateways::repository::route_repository::RouteRepository;
use crate::adapter::gateways::repository::user_repository::UserRepository;
use crate::application::usecase::routes::get_route_usecase::{GetRouteInput, GetRouteUsecase};
use crate::domain::driver::driver::Driver;
use crate::domain::driver::driver_id::DriverId;
use crate::domain::route::route_search_condition::RouteSearchCondition;
use crate::domain::user::user_id::UserId;
use crate::domain::value_object::point::Point;

pub fn routes_router() -> Router {
    Router::new().route("/", get(get_route))
}

#[derive(Debug, Deserialize)]
struct GetRouteParams {
    user_id: String,
    driver_id: String,
    longitude: f64,
    latitude: f64,
}

async fn get_route(Query(params): Query<GetRouteParams>) -> Result<impl IntoResponse, AppError> {
    info!(request.params =? params, "get_route");

    let user_id = UserId::try_from(params.user_id)?;
    let driver_id = DriverId::try_from(params.driver_id)?;
    let point = Point::new(params.longitude, params.latitude)?;
    let input = GetRouteInput::new(user_id, driver_id, RouteSearchCondition::Point(point));

    let user_repo = UserRepository {};
    let driver_repo = DriverRepository {};
    let route_repo = RouteRepository {};

    let usecase = GetRouteUsecase::new(user_repo, driver_repo, route_repo);

    let res = usecase.execute(input)?;
    Ok((StatusCode::OK, Json(res)))
}
