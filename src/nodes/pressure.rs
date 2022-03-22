#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub struct Pressure {
    pub id: usize,
    pub elevation: f64,
    pub pressure: f64,
    pub consumption: f64,
}

impl Default for Pressure {
    fn default() -> Self {
        Pressure::new( 0 )
    }
}

impl Pressure {
    pub fn new( id: usize ) -> Self {
        Pressure {
            id,
            elevation: 0.0,
            pressure: 101325.0,
            consumption: 0.0,
        }
    } 
}