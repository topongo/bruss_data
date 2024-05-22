use serde::{Deserialize, Serialize};
use tt::{TTArea,AreaType};
use crate::Type;

use super::{BrussType, FromTT};

#[derive(Deserialize, Serialize, Debug)]
pub struct Area {
    pub id: u16,
    pub label: String,
    #[serde(rename = "type")]
    pub ty: AreaType
}

impl Area {
    #[allow(dead_code)]
    pub(crate) fn new(id: u16, label: String, ty: AreaType) -> Self {
        Self { id, label, ty }
    }
}

impl BrussType for Area {
    const TYPE: Type = Type::Area;
}

impl FromTT<TTArea> for Area {
    fn from_tt(value: TTArea) -> Self {
        let TTArea { id, label, ty } = value;
        Self { id, label, ty }
    }
}

