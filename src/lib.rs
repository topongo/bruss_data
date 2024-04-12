#![feature(iter_intersperse)]

mod area;
mod route;
mod coords;
mod stop;
mod map;
mod trip;
mod helpers;

pub use area::Area;
pub use route::Route;
pub use stop::Stop;
pub use coords::Coords;
pub use map::{Segment,Path,StopPair,sequence_hash};
#[cfg(feature = "polyline")]
pub use map::polyline::PolySegment;
pub use trip::{Trip,Direction};
pub use helpers::AreaHelper;

use serde::{de::DeserializeOwned, Serialize};
use tt::{TTType, AreaType};

pub trait BrussType: Serialize + DeserializeOwned {
    const DB_NAME: &'static str;

    #[cfg(feature = "db")]
    fn get_coll(db: &mongodb::Database) -> mongodb::Collection<Self> {
        db.collection(Self::DB_NAME)
    }
}

/// Struct that can be converted to a bruss-compatible data, that will be serialized inside a
/// database.
pub trait FromTT<From: TTType> {
    /// Convert to a
    fn from_tt(value: From) -> Self;
}

pub trait InArea {
    fn ty(&self) -> AreaType;
    fn id(&self) -> u16;
}


