use crate::domain::route::step::Step;
use crate::domain::value_object::distance::Distance;
use crate::domain::value_object::duration::Duration;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Leg {
    start_address: String,
    end_address: String,
    distance: Distance,
    duration: Duration,
    steps: Vec<Step>,
}

impl Leg {
    pub fn new(
        start_address: String,
        end_address: String,
        distance: Distance,
        duration: Duration,
        steps: Vec<Step>,
    ) -> Self {
        Self {
            start_address,
            end_address,
            distance,
            duration,
            steps,
        }
    }
}
