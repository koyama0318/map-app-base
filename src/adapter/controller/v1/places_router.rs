use crate::adapter::controller::app_error::AppError;
use crate::adapter::gateways::repository::place_repository::PlaceRepository;
use crate::application::usecase::places::get_place_usecase::{GetPlaceInput, GetPlaceUsecase};
use crate::domain::place::place::Place;
use crate::domain::place::place_search_condition::PlaceSearchCondition;
use anyhow::Result;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::info;

pub fn places_router() -> Router {
    Router::new().route("/", get(get_places))
}

#[derive(Debug, Deserialize)]
struct SearchParams {
    keyword: String,
}

#[derive(Serialize)]
struct GetPlacesResponse {
    places: Vec<Place>,
}

impl GetPlacesResponse {
    fn new(places: Vec<Place>) -> Self {
        Self { places }
    }
}

async fn get_places(Query(params): Query<SearchParams>) -> Result<impl IntoResponse, AppError> {
    info!(request.params =? params, "get_places");
    let input = GetPlaceInput::new(PlaceSearchCondition::Keyword(params.keyword));
    let route_repo = PlaceRepository {};
    let usecase = GetPlaceUsecase::new(route_repo);

    let result = usecase.execute(input)?;
    let res = GetPlacesResponse::new(result);
    Ok((StatusCode::OK, Json(res)))
}
