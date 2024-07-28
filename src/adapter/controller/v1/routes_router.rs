use anyhow::Result;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use http::status::StatusCode;
use serde::Deserialize;
use tracing::info;

use crate::adapter::controller::app_error::AppError;
use crate::adapter::gateways::repository::route_repository::RouteRepository;
use crate::application::usecase::routes::get_route_usecase::{GetRouteInput, GetRouteUsecase};
use crate::domain::route::route_search_condition::RouteSearchCondition;
use crate::domain::value_object::point::Point;

pub fn routes_router() -> Router {
    Router::new().route("/", get(get_route))
}

#[derive(Debug, Deserialize)]
struct SearchParams {
    longitude: f64,
    latitude: f64,
}

async fn get_route(Query(params): Query<SearchParams>) -> Result<impl IntoResponse, AppError> {
    info!(request.params =? params, "get_route");
    let input = GetRouteInput::new(RouteSearchCondition::Point(Point::new(
        params.longitude,
        params.latitude,
    )?));
    let route_repo = RouteRepository {};
    let usecase = GetRouteUsecase::new(route_repo);

    let response = usecase.execute(input)?;
    Ok((StatusCode::OK, Json(response)))
}
