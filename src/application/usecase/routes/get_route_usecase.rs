use crate::application::repository::route_repository::IRouteRepository;
use crate::domain::route::route::Route;
use crate::domain::route::route_search_condition::RouteSearchCondition;
use anyhow::Result;

pub struct GetRouteInput {
    condition: RouteSearchCondition,
}

impl GetRouteInput {
    pub fn new(condition: RouteSearchCondition) -> Self {
        Self { condition }
    }
}

pub struct GetRouteUsecase<RR>
where
    RR: IRouteRepository,
{
    route_repo: RR,
}

impl<RR> GetRouteUsecase<RR>
where
    RR: IRouteRepository,
{
    pub fn new(route_repo: RR) -> Self {
        Self { route_repo }
    }

    pub fn execute(&self, input: GetRouteInput) -> Result<Route> {
        self.route_repo.get(input.condition)
    }
}
