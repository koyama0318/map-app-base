use crate::domain::place::place::Place;
use crate::domain::place::place_search_condition::PlaceSearchCondition;
use anyhow::Result;

pub trait PlaceRepositoryInterface {
    fn get(&self, search_condition: PlaceSearchCondition) -> Result<Place>;
}
