use crate::application::repository::route_repository::IRouteRepository;
use crate::domain::route::route::Route;
use crate::domain::route::route_search_condition::RouteSearchCondition;
use anyhow::Result;

#[derive(Debug)]
pub struct RouteRepository {}

impl IRouteRepository for RouteRepository {
    fn get(&self, condition: RouteSearchCondition) -> Result<Route> {
        todo!()
    }
}
