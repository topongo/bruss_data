use serde::{Serialize,Deserialize};
use tt::{TTStop,AreaType};
use crate::Coords;

use super::BrussType;
use super::FromTT;

#[derive(Serialize, Deserialize, Debug)]
pub struct Stop {
    pub id: u16,
    pub code: String,
    pub description: String,
    pub position: Coords,
    pub altitude: i32,
    pub name: String,
    pub street: Option<String>,
    pub town: Option<String>,
    #[serde(rename = "type")]
    pub ty: AreaType,
    pub wheelchair_boarding: bool
}

impl Stop {
    pub fn new(id: u16, code: String, description: String, position: Coords, altitude: i32, name: String, street: Option<String>, town: Option<String>, ty: AreaType, wheelchair_boarding: bool) -> Self {
        Self { id, code, description, position, altitude, name, street, town, ty, wheelchair_boarding }
    }
}

impl BrussType for Stop {
    const DB_NAME: &'static str = "stops";
}

impl FromTT<TTStop> for Stop {
    fn from_tt(value: TTStop) -> Self {
        let TTStop { id, code, description, lat, lng, altitude, name, street, town, ty, wheelchair_boarding } = value;
        Self { id, code, description, position: Coords::new(lat, lng), altitude, name, street, town, ty, wheelchair_boarding }
    }
}

