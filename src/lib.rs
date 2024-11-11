#![feature(iter_intersperse)]

mod area;
mod route;
mod coords;
mod stop;
mod map;
mod trip;
mod helpers;
// mod log;
mod ty;
// mod database;

pub use area::Area;
use mongodb::bson::Document;
pub use ty::Type;
pub use route::Route;
pub use stop::{Stop,StopPair};
pub use coords::Coords;
pub use map::{Segment,Path,RoutingType,sequence_hash};
#[cfg(feature = "polyline")]
pub use map::polyline::PolySegment;
pub use trip::{Trip,Direction};
pub use helpers::AreaHelper;

use serde::{de::DeserializeOwned, Serialize};
use tt::{TTType, AreaType};

pub trait BrussType: Serialize + DeserializeOwned {
    const TYPE: Type;

    #[cfg(feature = "db")]
    fn get_coll(db: &mongodb::Database) -> mongodb::Collection<Self> {
        db.collection(Self::TYPE.collection())
    }
}

/// Struct that can be converted to a bruss-compatible data, that will be serialized inside a
/// database.
pub trait FromTT<From: TTType> {
    /// Converts from a tt type.
    fn from_tt(value: From) -> Self;
}

pub trait InArea {
    fn ty(&self) -> AreaType;
    fn id(&self) -> u16;
}


