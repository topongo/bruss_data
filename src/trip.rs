use std::collections::{HashMap, BTreeMap};

use chrono::NaiveTime;
use serde::{Serialize,Deserialize};
use tt::{TTTrip, AreaType};
use sha1::Digest;

use crate::{BrussType, FromTT};

#[derive(Serialize,Deserialize,Debug)]
pub enum Direction {
    Forward,
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

#[derive(Serialize,Deserialize,Debug)]
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
    // List of stop ids
    pub path: Vec<u16>,
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
        path: Vec<u16>,
        times: HashMap<u16, StopTime>,
        ty: AreaType
    ) -> Self {
        Self { id, delay, direction, next_stop, last_stop, bus_id, route, path, times, ty }
    } 
}

impl BrussType for Trip {
    const DB_NAME: &'static str = "trips";
}

impl FromTT<TTTrip> for Trip {
    fn from_tt(value: TTTrip) -> Self {
        let TTTrip { id, delay, direction, next_stop, last_stop, bus_id, route, stop_times, ty } = value;
        let mut path: BTreeMap<u16, u16> = BTreeMap::new();
        let mut times: HashMap<u16, StopTime> = HashMap::with_capacity(stop_times.len());
        for st in stop_times {
            path.insert(st.sequence, st.stop);
            times.insert(st.stop, StopTime { arrival: st.arrival, departure: st.departure });
        }
        Self { id, delay: delay.unwrap_or(0.) as i32, direction: Direction::from(direction), next_stop, last_stop, bus_id, route, path: path.values().map(|v| *v).collect(), times, ty }
    }
}

 
