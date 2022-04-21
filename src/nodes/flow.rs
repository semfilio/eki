#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub struct Flow {
    pub id: usize,
    pub elevation: f64,
    pub pressure: Vec<f64>,
    pub consumption: Vec<f64>,
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
            pressure: vec![ 101325.0 ],
            consumption: vec![ -0.1 ],
        }
    }

    pub fn new_with_value( id: usize, value: f64 ) -> Self {
        Flow {
            id,
            elevation: 0.0,
            pressure: vec![ 101325.0 ],
            consumption: vec![ value ],
        }
    }

    pub fn new_elevation( id: usize, elevation: f64 ) -> Self {
        Flow {
            id,
            elevation,
            pressure: vec![ 101325.0 ],
            consumption: vec![ -0.1 ],
        }
    }
}