use crate::application::repository::driver_repository::IDriverRepository;
use crate::application::repository::route_repository::IRouteRepository;
use crate::application::repository::user_repository::IUserRepository;
use crate::domain::driver::driver_id::DriverId;
use crate::domain::route::route::Route;
use crate::domain::route::route_search_condition::RouteSearchCondition;
use crate::domain::user::user_id::UserId;
use anyhow::Result;

pub struct GetRouteInput {
    user_id: UserId,
    driver_id: DriverId,
    condition: RouteSearchCondition,
}

impl GetRouteInput {
    pub fn new(user_id: UserId, driver_id: DriverId, condition: RouteSearchCondition) -> Self {
        Self {
            user_id,
            driver_id,
            condition,
        }
    }
}

pub struct GetRouteUsecase<UR, DR, RR>
where
    UR: IUserRepository,
    DR: IDriverRepository,
    RR: IRouteRepository,
{
    user_repo: UR,
    driver_repo: DR,
    route_repo: RR,
}

impl<UR, DR, RR> GetRouteUsecase<UR, DR, RR>
where
    UR: IUserRepository,
    DR: IDriverRepository,
    RR: IRouteRepository,
{
    pub fn new(user_repo: UR, driver_repo: DR, route_repo: RR) -> Self {
        Self {
            user_repo,
            driver_repo,
            route_repo,
        }
    }

    pub fn execute(&self, input: GetRouteInput) -> Result<Route> {
        let user = self.user_repo.get(input.user_id)?;
        let driver = self.driver_repo.get(input.driver_id)?;
        let route = self.route_repo.get(user, driver, input.condition)?;
        Ok(route)
    }
}
