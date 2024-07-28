pub struct RouteData {
    pub start_longitude: f64,
    pub start_latitude: f64,
    pub destination_longitude: f64,
    pub destination_latitude: f64,
    pub estimated_time: i64,
    pub distance: f64,
    pub fees: i32,
}

impl RouteData {
    pub fn new(
        start_longitude: f64,
        start_latitude: f64,
        destination_longitude: f64,
        destination_latitude: f64,
        estimated_time: i64,
        distance: f64,
        fees: i32,
    ) -> Self {
        Self {
            start_longitude,
            start_latitude,
            destination_longitude,
            destination_latitude,
            estimated_time,
            distance,
            fees,
        }
    }
}
