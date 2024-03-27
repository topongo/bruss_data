mod area;
mod route;
mod coords;
mod stop;

pub use area::Area;
pub use route::Route;
pub use stop::Stop;
pub use coords::Coords;

use serde::{de::DeserializeOwned, Serialize};
use tt::TTType;

pub trait BrussType: Serialize + DeserializeOwned {
    const DB_NAME: &'static str;
}

/// Struct that can be converted to a bruss-compatible data, that will be serialized inside a
/// database.
pub trait FromTT<From: TTType> {
    /// Convert to a
    fn from_tt(value: From) -> Self;
}



