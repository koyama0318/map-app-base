use crate::domain::value_object::distance::Distance;
use crate::domain::value_object::duration::Duration;
use crate::domain::value_object::point::Point;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Step {
    distance: Distance,
    duration: Duration,
    end_point: Point,
}

impl Step {
    pub fn new(distance: Distance, duration: Duration, end_point: Point) -> Self {
        Self {
            distance,
            duration,
            end_point,
        }
    }
}
