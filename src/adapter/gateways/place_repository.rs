use crate::application::repository::place_repository::IPlaceRepository;
use crate::domain::place::place::Place;
use crate::domain::place::place_search_condition::PlaceSearchCondition;
use anyhow::Result;

#[derive(Debug)]
pub struct PlaceRepository {}

impl IPlaceRepository for PlaceRepository {
    fn get(&self, search_condition: PlaceSearchCondition) -> Result<Place> {
        todo!()
    }
}
