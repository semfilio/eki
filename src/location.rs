#[derive(Clone, Copy, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}

impl Default for Location {
    fn default() -> Self {
        Location::new( 0.0, 0.0 )
    }
}

impl Location {
    pub fn new( x: f32, y: f32  ) -> Self {
        Location { x, y }
    }
}