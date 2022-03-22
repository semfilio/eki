#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub struct Connection {
    pub id: usize,
    pub elevation: f64,
    pub pressure: f64,
    pub consumption: f64,
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
            pressure: 101325.0,
            consumption: 0.0,
        }
    }

}