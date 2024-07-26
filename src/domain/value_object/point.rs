use anyhow::{
    anyhow,
    Result,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    longitude: f64,
    latitude: f64,
}

impl Point {
    pub fn new(longitude: f64, latitude: f64) -> Result<Self> {
        if 0.0 <= longitude && longitude <= 180.0 && -90.0 <= latitude && latitude <= 90.0 {
            Ok(Self {
                longitude,
                latitude,
            })
        } else {
            Err(anyhow!("invalid longitude or latitude"))
        }
    }

    pub fn longitude(self) -> f64 {
        self.longitude
    }

    pub fn latitude(self) -> f64 {
        self.latitude
    }
}

#[cfg(test)]
mod test {
    use super::Point;

    #[test]
    fn point_validation_test() {
        assert!(!Point::new(0.0, -90.0).is_err());
        assert!(!Point::new(180.0, 90.0).is_err());
        assert!(Point::new(-1.0, 0.0).is_err());
        assert!(Point::new(0.0, -91.0).is_err());
        assert!(Point::new(-1.0, -91.0).is_err());
        assert!(Point::new(181.0, 0.0).is_err());
        assert!(Point::new(0.0, 91.0).is_err());
        assert!(Point::new(181.0, 91.0).is_err());
    }
}
