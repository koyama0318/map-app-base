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
}

impl Place {
    pub fn new(id: PlaceId, name: String, point: Point) -> Self {
        Self { id, name, point }
    }
}
