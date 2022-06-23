use crate::location::Location;

#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub struct Hidden {
    pub id: usize,
    pub loc: Location,
    pub r: f32,
    pub selected: bool,
    pub elevation: f64, //TODO not needed
    pub pressure: Vec<f64>, //TODO not needed
    pub consumption: Vec<f64>, //TODO not needed
}

impl Hidden {
    pub fn new( id: usize, x: f32, y: f32 ) -> Self {
        Hidden {
            id,
            loc: Location::new( x, y ),
            r: 5.0,
            selected: false,
            elevation: 0.0,
            pressure: vec![ 101325.0 ],
            consumption: vec![ 0.0 ],
        }
    }
}