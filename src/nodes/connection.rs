#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub struct Connection {
    pub id: usize,
    pub elevation: f64,
    pub pressure: Vec<f64>,
    pub consumption: Vec<f64>,
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
        }
    }

    pub fn new_elevation( id: usize, elevation: f64 ) -> Self {
        Connection {
            id,
            elevation,
            pressure: vec![ 101325.0 ],
            consumption: vec![ 0.0 ],
        }
    }

}