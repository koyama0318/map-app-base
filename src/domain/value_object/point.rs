use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
}

impl Point {
    pub fn new(latitude: f64, longitude: f64) -> Result<Self> {
        if -90.0 <= latitude && latitude <= 90.0 && -180.0 <= longitude && longitude <= 180.0 {
            Ok(Self {
                latitude,
                longitude,
            })
        } else {
            bail!("invalid longitude or latitude")
        }
    }
}

#[cfg(test)]
mod test {
    use super::Point;

    #[test]
    fn test_valid_boundaries() {
        // Test valid boundary values for latitude and longitude
        assert!(Point::new(-90.0, -180.0).is_ok());
        assert!(Point::new(90.0, 180.0).is_ok());
        assert!(Point::new(-90.0, 180.0).is_ok());
        assert!(Point::new(90.0, -180.0).is_ok());
    }

    #[test]
    fn test_invalid_latitude() {
        // Test invalid latitude values
        assert!(Point::new(-90.1, 0.0).is_err());
        assert!(Point::new(90.1, 0.0).is_err());
    }

    #[test]
    fn test_invalid_longitude() {
        // Test invalid longitude values
        assert!(Point::new(0.0, -180.1).is_err());
        assert!(Point::new(0.0, 180.1).is_err());
    }

    #[test]
    fn test_valid_inner_values() {
        // Test valid non-boundary values
        assert!(Point::new(45.0, 90.0).is_ok());
        assert!(Point::new(-45.0, -90.0).is_ok());
    }
}
