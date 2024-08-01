use crate::domain::driver::driver::Driver;
use crate::domain::route::route::Route;
use crate::domain::route::route_search_condition::RouteSearchCondition;
use crate::domain::user::user::User;
use anyhow::Result;

pub trait IRouteRepository {
    fn get(&self, user: User, driver: Driver, condition: RouteSearchCondition) -> Result<Route>;
}
