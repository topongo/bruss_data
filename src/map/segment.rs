use crate::{Type,BrussType, Coords};
use serde::{Serialize,Deserialize};
use tt::AreaType;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Segment {
    pub from: u16,
    pub to: u16,
    #[serde(rename = "type")]
    pub ty: AreaType,
    pub geometry: Vec<Coords>
}

impl Segment {
    pub fn new(from: u16, to: u16, ty: AreaType, geometry: Vec<Coords>) -> Self {
        Self { from, to, ty, geometry }
    }
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.from == other.from && self.to == other.to
    }
} 

impl BrussType for Segment {
    const TYPE: Type = Type::Segment;
}

#[cfg(feature = "polyline")]
pub(crate) mod polyline { 
}


#[test]
fn sequence_hash_test() {
    use crate::sequence_hash;

    assert_eq!(sequence_hash(AreaType::E, &vec![1, 2, 3]), sequence_hash(AreaType::E, &vec![1, 2, 3]));
    assert_ne!(sequence_hash(AreaType::U, &vec![1, 2, 3]), sequence_hash(AreaType::E, &vec![1, 2, 3]));
    assert_ne!(sequence_hash(AreaType::E, &vec![1, 2, 3]), sequence_hash(AreaType::E, &vec![3, 2, 1]));
}

#[test]
fn path_test_from_segments() {
    use crate::{Path,RoutingType};

    let p1 = Path::new(vec![12, 15, 18, 20], AreaType::E, RoutingType::Bus);
    let p2 = Path::new_from_segments(vec![(12, 15), (15, 18), (18, 20)], AreaType::E);

    assert_eq!(p1.sequence, p2.sequence);
}

