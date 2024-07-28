use crate::adapter::gateways::api_data::place::PlaceData;
use crate::application::repository::place_repository::IPlaceRepository;
use crate::domain::place::place::Place;
use crate::domain::place::place_id::PlaceId;
use crate::domain::place::place_search_condition::PlaceSearchCondition;
use crate::domain::value_object::point::Point;
use anyhow::Result;

#[derive(Debug)]
pub struct PlaceRepository {}

impl IPlaceRepository for PlaceRepository {
    fn get(&self, search_condition: PlaceSearchCondition) -> Result<Place> {
        let place = PlaceData::new("id".to_string(), "place".to_string(), 0.0, 0.0);
        let place = Place::new(
            PlaceId::try_from(place.id)?,
            place.name,
            Point::new(place.longitude, place.latitude)?,
        );
        Ok(place)
    }
}
