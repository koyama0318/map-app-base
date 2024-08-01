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

        let places = PlaceResponse::deserialize_to_domain(json_data)?;
        Ok(places)
    }
}
