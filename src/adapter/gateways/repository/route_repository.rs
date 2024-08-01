use crate::adapter::gateways::api_data::directions_data::*;
use crate::application::repository::route_repository::IRouteRepository;
use crate::domain::driver::driver::Driver;
use crate::domain::route::route::Route;
use crate::domain::route::route_search_condition::RouteSearchCondition;
use crate::domain::user::user::User;
use crate::domain::value_object::point::Point;
use anyhow::{bail, Result};
use serde_json::from_str;

#[derive(Debug)]
pub struct RouteRepository {}

impl IRouteRepository for RouteRepository {
    fn get(&self, user: User, driver: Driver, condition: RouteSearchCondition) -> Result<Route> {
        let api_key = "";

        // DriverからUserを経由して検索地点までの経路を取得する
        let origin = format!("{},{}", driver.point.latitude, driver.point.longitude);
        let waypoint = format!("{},{}", user.point.latitude, user.point.longitude);
        let destination = condition.search_key();

        let url = format!(
            "https://maps.googleapis.com/maps/api/directions/json?origin={}&destination={}&waypoints={}&key={}",
            origin, destination, waypoint, api_key
        );

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

        let routes = DirectionsResponse::deserialize_to_domain(json_data)?;
        match routes.into_iter().next() {
            Some(route) => Ok(route),
            None => bail!("No route found"),
        }
    }
}
