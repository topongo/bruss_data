use serde::{Serialize,Deserialize};
use tt::AreaType;

use crate::{Type, BrussType, Route, StopPair};
use super::sequence_hash;

#[derive(Serialize,Deserialize,Debug)]
pub struct Path {
    pub id: String,
    #[serde(rename = "type")]
    pub ty: AreaType,
    pub sequence: Vec<u16>,
    #[serde(default)]
    pub rty: RoutingType,
}

#[derive(Clone,Copy,Serialize,Deserialize,Debug,Default,PartialEq,Eq,Hash)]
#[serde(rename_all = "snake_case")]
pub enum RoutingType {
    #[default]
    Bus,
    Railway,
    Cableway,
}

impl From<&'_ Route> for RoutingType {
    fn from(value: &Route) -> Self {
        value.routing_type()
    }
}

impl Path {
    pub fn new(sequence: Vec<u16>, ty: AreaType, rty: RoutingType) -> Self {
        Self { id: sequence_hash(ty, &sequence), sequence, ty, rty }
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
        Self::new_from_segments_with_type(segments, ty, RoutingType::default())
    }

    pub fn new_from_segments_with_type(segments: Vec<StopPair>, ty: AreaType, routing_ty: RoutingType) -> Self {
        Self::new(Self::segments_to_sequence(segments), ty, routing_ty)
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
    const TYPE: Type = Type::Path;
}

impl PartialEq<Vec<u16>> for Path {
    fn eq(&self, other: &Vec<u16>) -> bool {
        &self.sequence == other
    }
}
