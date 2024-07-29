use crate::adapter::gateways::api_data::route_data::RouteData;
use crate::application::repository::route_repository::IRouteRepository;
use crate::domain::route::route::Route;
use crate::domain::route::route_search_condition::RouteSearchCondition;
use crate::domain::value_object::point::Point;
use anyhow::Result;

#[derive(Debug)]
pub struct RouteRepository {}

impl IRouteRepository for RouteRepository {
    fn get(&self, condition: RouteSearchCondition) -> Result<Route> {
        let route = RouteData::new(0.0, 0.0, 0.0, 0.0, 12000, 13.1, 1000);
        let route = Route::new(
            Point::new(route.start_longitude, route.start_latitude)?,
            Point::new(route.destination_longitude, route.destination_latitude)?,
            route.estimated_time,
            route.distance,
            route.fees,
        );
        Ok(route)
    }
}
