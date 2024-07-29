use crate::domain::value_object::point::Point;
use serde::Serialize;

use super::place_id::PlaceId;

#[derive(Serialize)]
pub struct Place {
    id: PlaceId,
    /// 地点名
    name: String,
    /// 地点
    point: Point,
    // 住所
    address: String,
}

impl Place {
    pub fn new(id: PlaceId, name: String, point: Point, address: String) -> Self {
        Self {
            id,
            name,
            point,
            address,
        }
    }
}
