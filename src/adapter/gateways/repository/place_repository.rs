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
                },
                "viewport" : {
                   "northeast" : {
                      "lat" : 35.70210904999998,
                      "lng" : 139.7676165798927
                   },
                   "southwest" : {
                      "lat" : 35.69889725,
                      "lng" : 139.7649169201073
                   }
                }
             },
             "icon" : "https://maps.gstatic.com/mapfiles/place_api/icons/v1/png_71/park-71.png",
             "name" : "お茶の水公園",
             "opening_hours" : {
                "open_now" : true
             },
             "photos" : [
                {
                   "height" : 3456,
                   "html_attributions" : [
                      "\u003ca href=\"https://maps.google.com/maps/contrib/103833376207990259782\"\u003eNati B.\u003c/a\u003e"
                   ],
                   "photo_reference" : "ATtYBwJcdfr2TMyPltvCsLURR82aua1BNqNIv1MzhlBaFOC7og4LG_2tkbUABIvSerlpmK4P_LK8k0b9FIABjZL4FwWndvdt2knVCXUV885tLLL8oI4XOV4XXbiIYVyv2Qb-2AKnCZ5SSZqVH2kQ00VhCxSJFAMfXB33a0S4q2cDijhtM82O",
                   "width" : 4608
                }
             ],
             "place_id" : "ChIJUdWeFBmMGGARJkazm0BpQSU",
             "plus_code" : {
                "compound_code" : "PQ28+FF 文京区、東京都",
                "global_code" : "8Q7XPQ28+FF"
             },
             "rating" : 3.6,
             "reference" : "ChIJUdWeFBmMGGARJkazm0BpQSU",
             "scope" : "GOOGLE",
             "types" : [ "park", "tourist_attraction", "point_of_interest", "establishment" ],
             "user_ratings_total" : 126,
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
