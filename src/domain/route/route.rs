use crate::domain::route::leg::Leg;
use crate::domain::value_object::point::Point;
use serde::Serialize;

#[derive(Serialize)]
pub struct Route {
    starting_point: Point,
    destination: Point,
    /// 所要時間(s)
    estimated_time: i64,
    /// 走行距離(km)
    distance: f64,
    /// 料金(yen)
    fees: i32,
    legs: Vec<Leg>,
}
