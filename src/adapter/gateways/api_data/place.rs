pub struct PlaceData {
    pub id: String,
    pub name: String,
    pub longitude: f64,
    pub latitude: f64,
}

impl PlaceData {
    pub fn new(id: String, name: String, longitude: f64, latitude: f64) -> Self {
        Self {
            id,
            name,
            longitude,
            latitude,
        }
    }
}
