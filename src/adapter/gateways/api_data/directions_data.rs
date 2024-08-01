use crate::domain::route::route::Route;
use crate::domain::value_object::point::Point;
use anyhow::{bail, Result};
use serde::Deserialize;
use serde_json::from_str;

#[derive(Debug, Deserialize)]
struct Distance {
    text: String,
    value: i32,
}

#[derive(Debug, Deserialize)]
struct Duration {
    text: String,
    value: i32,
}

#[derive(Debug, Deserialize)]
struct Location {
    lat: f64,
    lng: f64,
}

#[derive(Debug, Deserialize)]
struct Step {
    html_instructions: String,
    distance: Distance,
    duration: Duration,
    end_location: Location,
}

#[derive(Debug, Deserialize)]
struct Leg {
    start_address: String,
    end_address: String,
    distance: Distance,
    duration: Duration,
    steps: Vec<Step>,
}

#[derive(Debug, Deserialize)]
struct RouteData {
    legs: Vec<Leg>,
}

#[derive(Debug, Deserialize)]
pub struct DirectionsResponse {
    routes: Vec<RouteData>,
    status: String,
}

impl DirectionsResponse {
    pub fn deserialize_to_domain(json_data: &str) -> Result<Vec<Route>> {
        let api_response: DirectionsResponse = from_str(&json_data)?;

        if api_response.status != "OK" {
            bail!("Directions API error: {}", api_response.status);
        }

        let routes = api_response
            .routes
            .into_iter()
            .filter_map(|route| {
                let start = Point::new(10.0, 10.0).ok()?;
                let destination = Point::new(10.0, 20.0).ok()?;
                Some(Route::new(start, destination, 1000, 10.0, 1000))
            })
            .collect();

        Ok(routes)
    }
}
