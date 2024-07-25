use serde::Serialize;

use super::address::Address;

#[derive(Serialize)]
pub struct Route {
    starting_point: Address,
    destination: Address,
    /// 所要時間(s)
    estimated_time: i64,
    /// 走行距離(km)
    distance: f64,
    /// 料金(yen)
    fees: i32,
}

impl Route {
    pub fn new(
        starting_point: Address,
        destination: Address,
        estimated_time: i64,
        distance: f64,
        fees: i32,
    ) -> Self {
        Self {
            starting_point,
            destination,
            estimated_time,
            distance,
            fees,
        }
    }
}
