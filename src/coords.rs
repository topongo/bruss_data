use std::ops::Sub;
use serde::{Serialize,Deserialize, ser::SerializeSeq, de::Visitor, Serializer, Deserializer};
#[allow(unused_imports)]
use serde::{de::{Error as DeError,SeqAccess}, ser::Error as SerError};

#[derive(Debug,PartialEq,Clone)]
pub struct Coords {
    pub lat: f64,
    pub lng: f64
}

impl Coords {
    const DEG_METER: f64 = 113000.44;

    pub fn new(lat: f64, lng: f64) -> Self {
        Self { lat, lng }
    }
    
    pub fn to_osrm_query(&self) -> String {
        format!("{},{}", self.lng, self.lat)
    }
}

impl Sub for &Coords {
    type Output = f64;
    
    /// Returns the difference in meters from two positions
    fn sub(self, rhs: Self) -> Self::Output {
        ((self.lat - rhs.lat).abs().powi(2) + (self.lng - rhs.lng).abs().powi(2)).sqrt() * Coords::DEG_METER
    }
}

impl Serialize for Coords {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        let mut state = serializer.serialize_seq(Some(2))?;
        state.serialize_element(&self.lat)?;
        state.serialize_element(&self.lng)?;
        state.end()
    }
}

struct CoordsVisitor;

impl<'de> Visitor<'de> for CoordsVisitor {
    type Value = (f64, f64);
 
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an 2 item array representing [ lat, lng ]")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>, {
        let lat: f64 = match seq.next_element()? {
            Some(v) => v,
            None => return Err(DeError::invalid_length(0, &self))
        };
        let lng: f64 = match seq.next_element()? {
            Some(v) => v,
            None => return Err(DeError::invalid_length(1, &self))
        };

        match seq.next_element::<f64>()? {
            Some(_) => Err(DeError::invalid_length(3, &self)),
            None => Ok(if lat > lng {
                (lat, lng)
            } else {
                (lng, lat)
            })
        }
    }
}

impl<'de> Deserialize<'de> for Coords {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
        let (lat, lng): (f64, f64) = deserializer.deserialize_tuple(2, CoordsVisitor)?;
        Ok(Coords::new(lat, lng))
    }
}

