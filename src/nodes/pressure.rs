use crate::location::Location;
use crate::events::TransientEvent;

#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub struct Pressure {
    pub id: usize,
    pub elevation: f64,
    pub pressure: Vec<f64>,
    pub consumption: Vec<f64>,
    pub loc: Location,
    pub r: f32,
    pub selected: bool,
    pub events: Vec<TransientEvent>,
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
            pressure: vec![ 101325.0 ],
            consumption: vec![ 0.0 ],
            loc: Location::new( 0.0, 0.0 ),
            r: 20.0,
            selected: false,
            events: vec![],
        }
    }

    pub fn new_with_value( id: usize, value: f64 ) -> Self {
        Pressure {
            id,
            elevation: 0.0,
            pressure: vec![ value ],
            consumption: vec![ 0.0 ],
            loc: Location::new( 0.0, 0.0 ),
            r: 20.0,
            selected: false,
            events: vec![],
        }
    }

    pub fn new_elevation( id: usize, elevation: f64 ) -> Self {
        Pressure {
            id,
            elevation,
            pressure: vec![ 101325.0 ],
            consumption: vec![ 0.0 ],
            loc: Location::new( 0.0, 0.0 ),
            r: 20.0,
            selected: false,
            events: vec![],
        }
    }

    pub fn create_transient_values(&mut self, tnodes: &[f64]) {
        let consumption = vec![ self.consumption[0]; tnodes.len() ];
        let mut pressure = vec![ self.pressure[0]; tnodes.len() ];
        for (i, t) in tnodes.iter().enumerate() {
            for event in self.events.iter() {
                if *t >= event.time()  {
                    pressure[i] = event.value();
                } 
            }
        }
        self.pressure = pressure;
        self.consumption = consumption;
    } 
}