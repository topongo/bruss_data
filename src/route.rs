use serde::{Serialize,Deserialize};
use tt::{AreaType, TTRoute};

use super::{BrussType, FromTT};

#[derive(Serialize, Deserialize, Debug)]
pub struct Route {
    pub id: u16,
    #[serde(rename = "type")]
    pub ty: u16,
    area: u16,
    pub area_ty: AreaType,
    color: String,
    name: String,
    code: String,
}

impl Route {
    pub fn new(id: u16, ty: u16, area: u16, area_ty: AreaType, color: String, name: String, code: String) -> Self {
        Self { id, area, color, name, code, ty, area_ty }
    }
}

impl BrussType for Route {
    const DB_NAME: &'static str = "routes";
}

impl FromTT<TTRoute> for Route {
    fn from_tt(value: TTRoute) -> Self {
        let TTRoute { id, ty, area, color, area_ty, name, code } = value;
        Self { id, ty, area, color, name, code, area_ty }
    }
}

