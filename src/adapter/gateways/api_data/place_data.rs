use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Deserialize)]
pub struct Geometry {
    pub location: Location,
}

#[derive(Debug, Deserialize)]
pub struct PlusCode {
    pub compound_code: String,
    pub global_code: String,
}

#[derive(Debug, Deserialize)]
pub struct PlaceData {
    pub geometry: Geometry,
    pub name: String,
    pub place_id: String,
    pub types: Vec<String>,
    pub plus_code: PlusCode,
    pub vicinity: String,
}

#[derive(Debug, Deserialize)]
pub struct PlaceResponse {
    pub results: Vec<PlaceData>,
    pub status: String,
}
