mod segment;
mod path;
#[cfg(feature = "polyline")]
pub mod polyline;

pub use path::{RoutingType,Path};
pub use segment::Segment;

use tt::AreaType;
use sha1::Digest;

pub fn sequence_hash(ty: AreaType, seq: &Vec<u16>) -> String {
    let mut data = Vec::from([ty.into(), 0x0, 0x0]);
    data.append(&mut seq.iter()
        .flat_map(|v| [(*v >> 8) as u8, (*v & 0xff) as u8])
        .intersperse(0x0)
        .collect()
    );
    let mut hasher = sha1::Sha1::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
