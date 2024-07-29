use crate::application::repository::place_repository::IPlaceRepository;
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
    PR: IPlaceRepository,
{
    place_repo: PR,
}

impl<PR> GetPlaceUsecase<PR>
where
    PR: IPlaceRepository,
{
    pub fn new(place_repo: PR) -> Self {
        Self { place_repo }
    }

    pub fn execute(&self, input: GetPlaceInput) -> Result<Vec<Place>> {
        self.place_repo.get_list(input.condition)
    }
}
