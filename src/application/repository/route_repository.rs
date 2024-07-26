use crate::domain::route::route::Route;
use crate::domain::route::route_search_condition::RouteSearchCondition;
use anyhow::Result;

pub trait RouteRepositoryInterface {
    fn get(&self, point: RouteSearchCondition) -> Result<Route>;
}
