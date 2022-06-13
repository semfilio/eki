use crate::location::Location;

#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub struct Hidden {
    pub id: usize,
    pub loc: Location,
    pub r: f32,
    pub selected: bool,
}

impl Hidden {
    pub fn new( id: usize, x: f32, y: f32 ) -> Self {
        Hidden {
            id,
            loc: Location::new( x, y ),
            r: 5.0,
            selected: false,
        }
    }
}