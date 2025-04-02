use std::hash::Hash;

use chrono::{DateTime, Utc};
use serde::{Serialize,Deserialize};
use tt::AreaType;

use crate::{trip::StopTimes, BrussType, Direction, Trip};

#[derive(Serialize,Deserialize,Debug)]
pub struct Schedule {
    pub id: String,
    #[serde(with = "mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub departure: DateTime<Utc>,
    pub hints: ScheduleHints,
}

impl BrussType for Schedule {
    const TYPE: crate::ty::Type = crate::ty::Type::Schedule;
}

impl Schedule {
    pub fn from_trip(trip: &Trip, date: DateTime<Utc>) -> Self {
        Self { id: trip.id.clone(), departure: date, hints: ScheduleHints::from(trip) }
    }
}

impl PartialEq for Schedule {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.departure == other.departure
    }
}

impl Eq for Schedule {}

// custom implementation: we only care about the id and departure time.
// hints are made to make db queries faster.
impl Hash for Schedule {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.departure.hash(state);
    }
}

#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub struct ScheduleHints {
    #[serde(rename = "type")]
    pub route: u16,
    pub ty: AreaType,
    pub times: StopTimes,
    pub direction: Direction,
}

impl From<&Trip> for ScheduleHints {
    fn from(trip: &Trip) -> Self {
        Self {
            route: trip.route,
            ty: trip.ty,
            times: trip.times.clone(),
            direction: trip.direction.clone(),
        }
    }
}
