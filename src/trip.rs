use std::collections::HashMap;

use chrono::NaiveTime;
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tt::{TTTrip, AreaType};

use crate::{sequence_hash, BrussType, FromTT, Type};

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
    pub arrival: NaiveTime,
    pub departure: NaiveTime,
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

impl FromTT<TTTrip> for Trip {
    fn from_tt(value: TTTrip) -> Self {
        let TTTrip { id, delay, direction, next_stop, last_stop, bus_id, route, stop_times, ty, headsign } = value;
        let mut times = HashMap::new();
        let path = sequence_hash(ty, &stop_times.iter()
            .map(|st| {
                let tt::StopTime { stop, arrival, departure, .. } = *st;
                times.insert(stop, StopTime { 
                    arrival: arrival.unwrap_or(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
                    departure: departure.unwrap_or(NaiveTime::from_hms_opt(0, 0, 0).unwrap()) });
                st.stop
            })
            .collect());
        Self { 
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
        }
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
 
