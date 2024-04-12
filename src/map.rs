use crate::{Coords,BrussType};
use serde::{Serialize,Deserialize};
use tt::AreaType;
use sha1::Digest;

pub type StopPair = (u16, u16);

#[derive(Serialize,Deserialize,Debug)]
pub struct Path {
    pub id: String,
    #[serde(rename = "type")]
    pub ty: AreaType,
    pub sequence: Vec<u16>,
}

impl Path {
    pub fn new(sequence: Vec<u16>, ty: AreaType) -> Self {
        Self { id: sequence_hash(ty, &sequence), sequence, ty }
    }

    pub fn segments_to_sequence(segments: Vec<StopPair>) -> Vec<u16> {
        if segments.len() == 0 {
            Vec::new()
        } else {
            let mut o = Vec::with_capacity(segments.len() - 1);
            o.push(segments[0].0);
            let mut prev = segments[0].0;
            for (s1, s2) in &segments {
                assert_eq!(*s1, prev);
                o.push(*s2);
                prev = *s2;
            }
            o
        }
    }

    pub fn new_from_segments(segments: Vec<StopPair>, ty: AreaType) -> Self {
        Self::new(Self::segments_to_sequence(segments), ty)
    }

    pub fn segments(&self) -> Vec<StopPair> {
        let mut o = Vec::with_capacity(self.sequence.len() + 1);
        let mut p = self.sequence[0];
        for s in &self.sequence[1..] {
            o.push((p, *s));
            p = *s;
        }
        o
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id || self.sequence == other.sequence
    }
}

impl BrussType for Path {
    const DB_NAME: &'static str = "paths";
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Segment {
    pub from: u16,
    pub to: u16,
    #[serde(rename = "type")]
    pub ty: AreaType,
    pub coords: Vec<Coords>
}

impl Segment {
    pub fn new(from: u16, to: u16, ty: AreaType, coords: Vec<Coords>) -> Self {
        Self { from, to, ty, coords }
    }
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.from == other.from && self.to == other.to
    }
} 

impl PartialEq<Vec<u16>> for Path {
    fn eq(&self, other: &Vec<u16>) -> bool {
        &self.sequence == other
    }
}

impl BrussType for Segment {
    const DB_NAME: &'static str = "segments";
}

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

#[cfg(feature = "polyline")]
pub(crate) mod polyline {
    use super::Segment;
    use super::AreaType;
    use serde::Serialize;
    use polyline::encode_coordinates;
    use geo_types::LineString;

    #[derive(Serialize)]
    pub struct PolySegment {
        pub from: u16,
        pub to: u16,
        #[serde(rename = "type")]
        pub ty: AreaType,
        pub polyline: String
    }

    impl From<Segment> for PolySegment {
        fn from(value: Segment) -> Self {
            let Segment { from, to, ty, coords } = value;
            let polyline = encode_coordinates::<LineString>(LineString::from_iter(coords.iter().map(|c| (c.lat, c.lng))), 5).unwrap();
            Self { from, to, ty, polyline }
        }
    }
}


#[test]
fn sequence_hash_test() {
    assert_eq!(sequence_hash(AreaType::E, &vec![1, 2, 3]), sequence_hash(AreaType::E, &vec![1, 2, 3]));
    assert_ne!(sequence_hash(AreaType::U, &vec![1, 2, 3]), sequence_hash(AreaType::E, &vec![1, 2, 3]));
    assert_ne!(sequence_hash(AreaType::E, &vec![1, 2, 3]), sequence_hash(AreaType::E, &vec![3, 2, 1]));
}

#[test]
fn path_test_from_segments() {
    let p1 = Path::new(vec![12, 15, 18, 20], AreaType::E);
    let p2 = Path::new_from_segments(vec![(12, 15), (15, 18), (18, 20)], AreaType::E);

    assert_eq!(p1.sequence, p2.sequence);
}

