use anyhow::{bail, Result};
use serde::Deserialize;
use serde_json::from_str;

use crate::domain::{
    place::{place::Place, place_id::PlaceId},
    value_object::point::Point,
};

#[derive(Debug, Deserialize)]
struct Location {
    lat: f64,
    lng: f64,
}

#[derive(Debug, Deserialize)]
struct Geometry {
    location: Location,
}

#[derive(Debug, Deserialize)]
struct PlusCode {
    compound_code: String,
    global_code: String,
}

#[derive(Debug, Deserialize)]
struct PlaceData {
    geometry: Geometry,
    name: String,
    place_id: String,
    plus_code: PlusCode,
    vicinity: String,
}

#[derive(Debug, Deserialize)]
pub struct PlaceResponse {
    results: Vec<PlaceData>,
    status: String,
}

impl PlaceResponse {
    pub fn deserialize_to_domain(json_data: &str) -> Result<Vec<Place>> {
        let api_response: PlaceResponse = from_str(&json_data)?;

        if api_response.status != "OK" {
            bail!("Places API error: {}", api_response.status);
        }

        let places: Vec<Place> = api_response
            .results
            .into_iter()
            .filter_map(|place| {
                println!("{:?}", place);
                let id = PlaceId::try_from(place.place_id).ok()?;
                let point =
                    Point::new(place.geometry.location.lat, place.geometry.location.lng).ok()?;
                let address = place.plus_code.compound_code + &place.vicinity;
                Some(Place::new(id, place.name, point, address))
            })
            .collect();

        Ok(places)
    }
}
