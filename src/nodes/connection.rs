use crate::location::Location;

#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub struct Connection {
    pub id: usize,
    pub elevation: f64,
    pub pressure: Vec<f64>,
    pub consumption: Vec<f64>,
    pub loc: Location,
    pub r: f32,
    pub selected: bool,
}

impl Default for Connection {
    fn default() -> Self {
        Connection::new( 0 )
    }
}

impl Connection {
    pub fn new( id: usize ) -> Self {
        Connection {
            id,
            elevation: 0.0,
            pressure: vec![ 101325.0 ],
            consumption: vec![ 0.0 ],
            loc: Location::new( 0.0, 0.0 ),
            r: 20.0,
            selected: false,
        }
    }

    pub fn new_elevation( id: usize, elevation: f64 ) -> Self {
        Connection {
            id,
            elevation,
            pressure: vec![ 101325.0 ],
            consumption: vec![ 0.0 ],
            loc: Location::new( 0.0, 0.0 ),
            r: 20.0,
            selected: false,
        }
    }

    //TODO do we need this ???
    pub fn create_transient_values(&mut self, tnodes: &[f64]) {
        self.pressure = vec![ self.pressure[0]; tnodes.len() ];
        self.consumption = vec![ self.consumption[0]; tnodes.len() ];
    } 

}