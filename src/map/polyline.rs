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
    pub geometry: String
}

impl From<Segment> for PolySegment {
    fn from(value: Segment) -> Self {
        let Segment { from, to, ty, geometry: coords } = value;
        let polyline = encode_coordinates::<LineString>(LineString::from_iter(coords.iter().map(|c| (c.lat, c.lng))), 5).unwrap();
        Self { from, to, ty, geometry: polyline }
    }
}
