use std::collections::HashMap;

use chrono::TimeDelta;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tt::{TTTrip, AreaType};

use crate::{sequence_hash, BrussType, Type};

#[derive(Serialize,Deserialize,Debug,PartialEq)]
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

#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub struct StopTime {
    pub arrival: TimeDelta,
    pub departure: TimeDelta,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Trip {
    pub id: String,
    pub delay: i32,
    pub direction: Direction,
    pub next_stop: u16,
    pub last_stop: u16,
    pub bus_id: Option<u16>,
    pub route: u16,
    pub headsign: String,
    // List of stop ids
    pub path: String,
    #[serde(serialize_with = "serialize_u16_keys", deserialize_with = "deserialize_u16_keys")]
    pub times: HashMap<u16, StopTime>,
    #[serde(rename = "type")]
    pub ty: AreaType,
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
        ty: AreaType
    ) -> Self {
        Self { id, delay, direction, next_stop, last_stop, bus_id, route, path, times, ty, headsign }
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
        let TTTrip { id, delay, direction, next_stop, last_stop, bus_id, route, stop_times, ty, headsign } = value;
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
            .collect());
        // if departure if after midnight but before 4am we assume it's the next day.
        let dep = if dep < TimeDelta::hours(4) { dep + TimeDelta::days(1) } else { dep };
        let dep = TimeDelta::from(dep);
        (Self { 
            id,
            delay: delay.unwrap_or(0.) as i32,
            direction: Direction::from(direction), 
            next_stop,
            last_stop,
            bus_id,
            route,
            path,
            ty,
            times,
            headsign,
        }, dep)
    }
}

impl PartialEq for Trip {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
 
fn serialize_u16_keys<S, T>(map: &HashMap<u16, T>, serializer: S) -> Result<S::Ok, S::Error> 
    where 
        S: Serializer,
        T: Serialize
{
    map.iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect::<HashMap<String, &T>>()
        .serialize(serializer)
}

fn deserialize_u16_keys<'de, D, T>(deserializer: D) -> Result<HashMap<u16, T>, D::Error> 
    where 
        D: Deserializer<'de>, 
        T: Deserialize<'de> 
{
    let h: HashMap<String, T> = HashMap::deserialize(deserializer)?;
    let mut o: HashMap<u16, T> = HashMap::with_capacity(h.len());
    for (k, v) in h {
        match k.parse::<u16>() {
            Ok(p) => { o.insert(p, v); },
            Err(e) => return Err(serde::de::Error::custom(format!("cannot parse int: {}", e)))
        }
    } 
    Ok(o)
}
 
