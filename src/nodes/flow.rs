#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub struct Flow {
    pub id: usize,
    pub elevation: f64,
    pub pressure: f64,
    pub consumption: f64,
}

impl Default for Flow {
    fn default() -> Self {
        Flow::new( 0 )
    }
}

impl Flow {
    pub fn new( id: usize ) -> Self {
        Flow {
            id,
            elevation: 0.0,
            pressure: 101325.0,
            consumption: -0.1,
        }
    }
}