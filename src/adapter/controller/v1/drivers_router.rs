use crate::domain::driver::UnvalidatedDriver;
use axum::extract::{Path, Query};
use axum::{routing::get, Router};
use tracing::info;

pub fn drivers_router() -> Router {
    Router::new().route("/drivers", get(get_driver_list)).route(
        "/drivers/:id",
        get(get_driver_by_id)
            .post(create_driver)
            .delete(delete_driver),
    )
}

async fn get_driver_list() {
    info!("get_driver_list");
}

async fn get_driver_by_id(Path(id): Path<String>) {
    info!(request.id=?id, "get_driver_by_id");
}

async fn create_driver(Query(req): Query<UnvalidatedDriver>, Path(id): Path<String>) {
    // info!(
    //     request.name=?req.name,
    //     request.longitude=?req.longitude,
    //     request.latitude=?req.latitude,
    //     request.id=?id,
    //     "create_driver",
    // );
}

async fn delete_driver(Path(id): Path<String>) {
    info!(request.id=?id, "delete_driver");
}
