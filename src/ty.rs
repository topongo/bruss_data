use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub enum Type {
    Area,
    Stop,
    Route,
    Trip,
    Path,
    Segment,
    Schedule,
}

pub enum Identification {
    Id,
    FromTo,
    IdDate,
}

impl Type {
    pub fn collection(&self) -> &'static str {
        match self {
            Self::Area => "areas",
            Self::Stop => "stops",
            Self::Trip => "trips",
            Self::Path => "paths",
            Self::Route => "routes",
            Self::Segment => "segments",
            Self::Schedule => "schedules",
        }
    }

    pub fn identify(&self) -> Identification {
        match self {
            Self::Segment => Identification::FromTo,
            Self::Schedule => Identification::IdDate,
            _ => Identification::Id
        }
    }
}
