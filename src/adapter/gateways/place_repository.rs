use crate::application::repository::place_repository::PlaceRepositoryInterface;
use crate::domain::place::place::Place;
use crate::domain::place::place_search_condition::PlaceSearchCondition;
use anyhow::Result;

#[derive(Debug)]
pub struct PlaceRepo {}

impl PlaceRepositoryInterface for PlaceRepo {
    fn get(&self, search_condition: PlaceSearchCondition) -> Result<Place> {
        todo!()
    }
}
