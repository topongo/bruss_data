use std::collections::HashMap;

use chrono::TimeDelta;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,PartialEq,Clone)]
pub struct StopTime {
    pub arrival: TimeDelta,
    pub departure: TimeDelta,
}

#[derive(Serialize,Deserialize,Debug,PartialEq,Clone)]
#[serde(transparent)]
pub struct StopTimes(#[serde(with = "serde_stop_time")] pub(crate) HashMap<u16, StopTime>);

impl StopTimes {
    pub fn has_stop(&self, stop: &u16) -> bool {
        self.0.contains_key(stop)
    }

    pub fn get(&self, stop: &u16) -> Option<&StopTime> {
        self.0.get(stop)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&u16, &StopTime)> {
        self.0.iter()
    }
}

mod serde_stop_time {
    use serde::{Deserializer, Serializer};

    use super::*;

    pub(super) fn serialize<S, T>(map: &HashMap<u16, T>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
            T: Serialize
    {
        map.iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect::<HashMap<String, &T>>()
            .serialize(serializer)
    }

    pub(super) fn deserialize<'de, D, T>(deserializer: D) -> Result<HashMap<u16, T>, D::Error>
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
}
