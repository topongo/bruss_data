use std::ops::Sub;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub struct Coords {
    pub lat: f64,
    pub lng: f64
}

impl Coords {
    const DEG_METER: f32 = 113000.44;

    pub fn new(lat: f64, lng: f64) -> Self {
        Self { lat, lng }
    }
    
    pub fn to_osrm_query(&self) -> String {
        format!("{},{}", self.lng, self.lat)
    }
}

impl Sub for Coords {
    type Output = f64;
    
    /// Returns the difference in meters from two positions
    fn sub(self, rhs: Self) -> Self::Output {
        ((self.lat - rhs.lat).abs().powi(2) + (self.lng - rhs.lng).abs().powi(2)).sqrt()
    }
}
