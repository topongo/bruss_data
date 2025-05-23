use std::{collections::HashMap, fmt::Display, str::FromStr};

use chrono::{TimeDelta, Utc, DateTime};
use serde::{Deserialize, Serialize};
use tt::{TTTrip, AreaType};

use crate::{sequence_hash, stop_time::{StopTime, StopTimes}, BrussType, Type};

#[derive(Serialize,Deserialize,Debug,PartialEq,Clone)]
pub enum Direction {
    #[serde(rename = "f")]
    Forward,
    #[serde(rename = "b")]
    Backward
}

impl From<u16> for Direction {
    fn from(value: u16) -> Self {
        match value {
            0 => Direction::Forward,
            1 => Direction::Backward,
            _ => panic!("unrecognized value for Direction: {}", value)
        }
    }
}

impl From<&Direction> for char {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Forward => 'f',
            Direction::Backward => 'b'
        }
    }
}

impl From<&Direction> for &str {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Forward => "f",
            Direction::Backward => "b"
        }
    }
}

#[derive(Debug)]
pub struct DirectionParseError;

impl std::error::Error for DirectionParseError {}

impl Display for DirectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Direction {
    type Err = DirectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "f" => Ok(Direction::Forward),
            "b" => Ok(Direction::Backward),
            _ => Err(DirectionParseError)
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Forward => write!(f, "f"),
            Direction::Backward => write!(f, "b")
        }
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Trip {
    pub id: String,
    #[serde(skip_serializing,default)]
    pub delay: i32,
    pub direction: Direction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_stop: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_stop: Option<u16>,
    pub bus_id: Option<u16>,
    pub route: u16,
    pub headsign: String,
    // List of stop ids
    pub path: String,
    pub times: StopTimes,
    #[serde(rename = "type")]
    pub ty: AreaType,
    pub last_event: Option<DateTime<Utc>>,
}

impl Trip {
    pub fn new(
        id: String,
        delay: i32,
        direction: Direction,
        next_stop: u16,
        last_stop: u16,
        bus_id: Option<u16>,
        route: u16,
        headsign: String,
        path: String,
        times: HashMap<u16, StopTime>,
        ty: AreaType,
        last_event: Option<DateTime<Utc>>,
    ) -> Self {
        Self { id, delay, direction, next_stop: if next_stop == 0 { None } else { Some(next_stop) }, last_stop: if last_stop == 0 { None } else { Some(last_stop) }, bus_id, route, path, times: StopTimes(times), ty, headsign, last_event }
    } 

    pub fn deep_cmp(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.delay == other.delay &&
        self.direction == other.direction &&
        self.next_stop == other.next_stop &&
        self.last_stop == other.last_stop &&
        self.bus_id == other.bus_id &&
        self.route == other.route &&
        self.headsign == other.headsign &&
        self.path == other.path &&
        self.times == other.times &&
        self.ty == other.ty
    }

    pub fn merge(self, other: Self) -> Self {
        let Self { delay, next_stop, last_stop, bus_id, .. } = other;
        Self { delay, next_stop, last_stop, bus_id, ..self }
    }
}

impl BrussType for Trip {
    const TYPE: Type = Type::Trip;
}

impl Trip {
    pub fn from_tt(value: TTTrip) -> (Self, TimeDelta) {
        let TTTrip { id, delay, direction, next_stop, last_stop, bus_id, route, stop_times, ty, headsign, last_event } = value;
        let mut times = HashMap::new();
        // this usually takes O(1) since usually stop_times[0].sequence == 1
        // (sequence starts at 1)
        let dep = stop_times.iter().find(|st| st.sequence == 1).unwrap().departure;
        let path = sequence_hash(ty, &stop_times.iter()
            .map(|st| {
                let tt::StopTime { stop, arrival, departure, .. } = *st;
                let arrival = arrival - dep;
                let departure = departure - dep;
                times.insert(stop, StopTime { 
                    arrival,
                    departure,
                });
                st.stop
            })
            .collect::<Vec<u16>>());
        // if departure if after midnight but before 4am we assume it's the next day.
        let dep = if dep < TimeDelta::hours(4) { dep + TimeDelta::days(1) } else { dep };
        let dep = TimeDelta::from(dep);
        (Self { 
            id,
            delay: delay.unwrap_or(0.) as i32,
            direction: Direction::from(direction), 
            next_stop: if next_stop == 0 { None } else { Some(next_stop) },
            last_stop: if last_stop == 0 { None } else { Some(last_stop) },
            bus_id,
            route,
            path,
            ty,
            times: StopTimes(times),
            headsign,
            last_event,
        }, dep)
    }
}

impl PartialEq for Trip {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
 
