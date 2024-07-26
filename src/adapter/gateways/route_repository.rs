use crate::application::repository::route_repository::RouteRepositoryInterface;
use crate::domain::route::route::Route;
use crate::domain::route::route_search_condition::RouteSearchCondition;
use anyhow::Result;

#[derive(Debug)]
pub struct RouteRepo {}

impl RouteRepositoryInterface for RouteRepo {
    fn get(&self, condition: RouteSearchCondition) -> Result<Route> {
        todo!()
    }
}
