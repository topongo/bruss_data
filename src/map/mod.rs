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
        // split stop id (16 bytes) into 2 u8 (8 bytes)
        .flat_map(|v| [(*v >> 8) as u8, (*v & 0xff) as u8])
        // add 0x0 between each stop id (stop id == 0 doens't exists)
        .fold(Vec::new(), |mut acc, v| {
            acc.push(v);
            acc.push(0x0);
            acc
        })
    );
    // remove last 0x0, only if sequence has at least 1 stop
    if data.len() != 3 {
        data.pop();
    }
    let mut hasher = sha1::Sha1::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
