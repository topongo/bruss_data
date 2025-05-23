mod area;
mod route;
mod coords;
mod stop;
mod map;
mod trip;
mod helpers;
mod stop_time;
// mod log;
mod ty;

#[cfg(feature = "db")]
mod schedule;
#[cfg(feature = "db")]
pub use schedule::{Schedule, ScheduleHints};

pub use area::Area;
pub use ty::Type;
pub use route::Route;
pub use stop::{Stop,StopPair};
pub use coords::Coords;
pub use map::{Segment,Path,RoutingType,sequence_hash};
#[cfg(feature = "polyline")]
pub use map::polyline::PolySegment;
pub use trip::{Trip,Direction};
pub use stop_time::{StopTime,StopTimes};
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


