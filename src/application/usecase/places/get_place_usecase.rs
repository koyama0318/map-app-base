use crate::application::repository::place_repository::PlaceRepositoryInterface;
use crate::domain::place::place::Place;
use crate::domain::place::place_search_condition::PlaceSearchCondition;
use anyhow::Result;

pub struct GetPlaceInput {
    condition: PlaceSearchCondition,
}

impl GetPlaceInput {
    pub fn new(condition: PlaceSearchCondition) -> Self {
        Self { condition }
    }
}

pub struct GetPlaceUsecase<PR>
where
    PR: PlaceRepositoryInterface,
{
    place_repo: PR,
}

impl<PR> GetPlaceUsecase<PR>
where
    PR: PlaceRepositoryInterface,
{
    pub fn new(place_repo: PR) -> Self {
        Self { place_repo }
    }

    pub fn execute(&self, input: GetPlaceInput) -> Result<Place> {
        self.place_repo.get(input.condition)
    }
}
