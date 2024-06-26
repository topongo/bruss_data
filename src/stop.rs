use serde::{Serialize,Deserialize};
use tt::{TTStop,AreaType};
use crate::Type;
use crate::Coords;
use crate::InArea;

use super::BrussType;
use super::FromTT;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    const TYPE: Type = Type::Stop;
}

impl FromTT<TTStop> for Stop {
    fn from_tt(value: TTStop) -> Self {
        let TTStop { id, code, description, lat, lng, altitude, name, street, town, ty, wheelchair_boarding } = value;
        Self { id, code, description, position: Coords::new(lat, lng), altitude, name, street, town, ty, wheelchair_boarding }
    }
}

impl InArea for Stop {
    fn ty(&self) -> AreaType {
        self.ty
    }

    fn id(&self) -> u16 {
        self.id
    }
}

pub type StopPair = (u16, u16);

