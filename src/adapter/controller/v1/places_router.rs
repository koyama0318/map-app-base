use crate::adapter::controller::app_error::AppError;
use crate::adapter::gateways::place_repository::PlaceRepository;
use crate::application::usecase::places::get_place_usecase::{GetPlaceInput, GetPlaceUsecase};
use crate::domain::place::place_search_condition::PlaceSearchCondition;
use anyhow::Result;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use http::status::StatusCode;
use serde::Deserialize;
use tracing::info;

pub fn places_router() -> Router {
    Router::new().route("/", get(get_places))
}

#[derive(Debug, Deserialize)]
struct SearchParams {
    keyword: String,
}

async fn get_places(Query(params): Query<SearchParams>) -> Result<impl IntoResponse, AppError> {
    info!(request.params =? params, "get_places");
    // let result: Result<Place> = Err(anyhow!("unimplement"));

    // let response = result
    //     .map(|r| (StatusCode::OK, Json(r)))?;
    // Ok(response)

    let input = GetPlaceInput::new(PlaceSearchCondition::Keyword(params.keyword));
    let route_repo = PlaceRepository {};
    let usecase = GetPlaceUsecase::new(route_repo);

    let response = usecase.execute(input)?;
    Ok((StatusCode::OK, Json(response)))
}
