use crate::adapter::controller::app_error::AppError;
use crate::adapter::gateways::driver_repository::DriverRepository;
use crate::application::usecase::drivers::create_driver_usecase::{
    CreateDriverInput, CreateDriverUsecase,
};
use crate::application::usecase::drivers::delete_driver_usecase::{
    DeleteDriverInput, DeleteDriverUsecase,
};
use crate::application::usecase::drivers::get_driver_list_usecase::{
    GetDriverListInput, GetDriverListUsecase,
};
use crate::application::usecase::drivers::get_driver_usecase::{GetDriverInput, GetDriverUsecase};
use crate::domain::driver::driver::UnvalidatedDriver;
use crate::domain::driver::driver_id::DriverId;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use http::StatusCode;
use tracing::info;

pub fn drivers_router() -> Router {
    Router::new()
        .route("/", get(get_driver_list).post(create_driver))
        .route("/:id", get(get_driver_by_id).delete(delete_driver))
}

async fn get_driver_list() -> Result<impl IntoResponse, AppError> {
    info!("get_driver_list");
    let input = GetDriverListInput::new();
    let user_repo = DriverRepository {};
    let usecase = GetDriverListUsecase::new(user_repo);

    let response = usecase.execute(input)?;
    Ok((StatusCode::OK, Json(response)))
}

async fn get_driver_by_id(Path(id): Path<String>) -> Result<impl IntoResponse, AppError> {
    info!(request.id = id, "get_driver_by_id");
    let input = GetDriverInput::new(DriverId::try_from(id)?);
    let user_repo = DriverRepository {};
    let usecase = GetDriverUsecase::new(user_repo);

    let response = usecase.execute(input)?;
    Ok((StatusCode::OK, Json(response)))
}

async fn create_driver(Json(user): Json<UnvalidatedDriver>) -> Result<impl IntoResponse, AppError> {
    info!(request =? user, "create_driver");
    let input = CreateDriverInput::new(user);
    let user_repo = DriverRepository {};
    let usecase = CreateDriverUsecase::new(user_repo);

    let response = usecase.execute(input)?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn delete_driver(Path(id): Path<String>) -> Result<impl IntoResponse, AppError> {
    info!(request.id = id, "delete_driver");
    let input = DeleteDriverInput::new(DriverId::try_from(id)?);
    let user_repo = DriverRepository {};
    let usecase = DeleteDriverUsecase::new(user_repo);

    usecase.execute(input)?;
    Ok((StatusCode::NO_CONTENT, ""))
}
