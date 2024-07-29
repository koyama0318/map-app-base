pub struct UserData {
    pub id: String,
    pub email: String,
    pub password: String,
    pub longitude: f64,
    pub latitude: f64,
}

impl UserData {
    pub fn new(id: String, email: String, password: String, longitude: f64, latitude: f64) -> Self {
        Self {
            id,
            email,
            password,
            longitude,
            latitude,
        }
    }
}
