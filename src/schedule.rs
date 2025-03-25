use chrono::{DateTime, Utc};
use serde::{Serialize,Deserialize};

use crate::{BrussType, Trip};

#[derive(Serialize,Deserialize,Debug,Hash,PartialEq,Eq)]
pub struct Schedule {
    pub id: String,
    #[serde(with = "mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub departure: DateTime<Utc>,
}

impl BrussType for Schedule {
    const TYPE: crate::ty::Type = crate::ty::Type::Schedule;
}

impl Schedule {
    pub fn from_trip(trip: &Trip, date: DateTime<Utc>) -> Self {
        Self { id: trip.id.clone(), departure: date }
    }

    pub fn from_id(id: String, date: DateTime<Utc>) -> Self {
        Self { id, departure: date }
    }
}
