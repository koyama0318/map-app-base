use crate::domain::place::place_id::PlaceId;
use crate::domain::value_object::point::Point;

pub enum RouteSearchCondition {
    Place(PlaceId),
    Point(Point),
}

impl RouteSearchCondition {
    pub fn search_key(&self) -> String {
        match self {
            RouteSearchCondition::Place(place_id) => place_id.0.clone(),
            RouteSearchCondition::Point(point) => format!("{},{}", point.latitude, point.longitude),
        }
    }
}
