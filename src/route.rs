use serde::{Serialize,Deserialize};
use tt::{AreaType, TTRoute};

use crate::{Type, RoutingType};

use super::{BrussType, FromTT};

#[derive(Serialize, Deserialize, Debug)]
pub struct Route {
    pub id: u16,
    #[serde(rename = "type")]
    pub ty: u16,
    pub area: u16,
    // #[serde(rename = "area_ty")]
    pub area_ty: AreaType,
    pub color: String,
    pub name: String,
    pub code: String,
}

impl Route {
    pub fn new(id: u16, ty: u16, area: u16, area_ty: AreaType, color: String, name: String, code: String) -> Self {
        Self { id, area, color, name, code, ty, area_ty }
    }

    pub fn routing_type(&self) -> RoutingType {
        match self.ty {
            2 => RoutingType::Railway,
            3 => RoutingType::Bus,
            5 => RoutingType::Cableway,
            // undefined behavior
            _ => panic!("route type {} is not recognized (valid values are 2 => railway, 3 => bus, 5 => cableway)", self.ty)
        }
    }
}

impl BrussType for Route {
    const TYPE: Type = Type::Route;
}

impl FromTT<TTRoute> for Route {
    fn from_tt(value: TTRoute) -> Self {
        let TTRoute { id, ty, area, color, area_ty, name, code } = value;
        Self { id, ty, area, color, name, code, area_ty }
    }
}

