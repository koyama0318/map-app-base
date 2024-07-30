use crate::adapter::gateways::api_data::place_data::*;
use crate::application::repository::place_repository::IPlaceRepository;
use crate::domain::place::place::Place;
use crate::domain::place::place_id::PlaceId;
use crate::domain::place::place_search_condition::PlaceSearchCondition;
use crate::domain::value_object::point::Point;
use anyhow::{bail, Result};
use serde_json::from_str;

#[derive(Debug)]
pub struct PlaceRepository {}

impl IPlaceRepository for PlaceRepository {
    fn get_list(&self, search_condition: PlaceSearchCondition) -> Result<Vec<Place>> {
        let json_data = r#"
    {
       "html_attributions" : [],
       "results" : [
          {
             "business_status" : "OPERATIONAL",
             "geometry" : {
                "location" : {
                   "lat" : 35.7011803,
                   "lng" : 139.7662335
                }
             },
             "name" : "お茶の水公園",
             "place_id" : "ChIJUdWeFBmMGGARJkazm0BpQSU",
             "plus_code" : {
                "compound_code" : "PQ28+FF 文京区、東京都",
                "global_code" : "8Q7XPQ28+FF"
             },
             "vicinity" : "文京区湯島１丁目４"
          },
          {
             "business_status" : "OPERATIONAL",
             "geometry" : {
                "location" : {
                   "lat" : 35.7011803,
                   "lng" : 139.7662335
                }
             },
             "name" : "お茶の水公園",
             "place_id" : "ChIJUdWeFBmMGGARJkazm0BpQSU",
             "plus_code" : {
                "compound_code" : "PQ28+FF 文京区、東京都",
                "global_code" : "8Q7XPQ28+FF"
             },
             "vicinity" : "文京区湯島１丁目４"
          }
       ],
       "status" : "OK"
    }
    "#;

        let api_response: PlaceResponse = from_str(json_data)?;
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
