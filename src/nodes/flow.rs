use crate::location::Location;
use crate::events::TransientEvent;

#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub struct Flow {
    pub id: usize,
    pub elevation: f64,
    pub pressure: Vec<f64>,
    pub consumption: Vec<f64>,
    pub loc: Location,
    pub r: f32,
    pub selected: bool,
    pub events: Vec<TransientEvent>,
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
            loc: Location::new( 0.0, 0.0 ),
            r: 20.0,
            selected: false,
            events: vec![],
        }
    }

    pub fn new_with_value( id: usize, value: f64 ) -> Self {
        Flow {
            id,
            elevation: 0.0,
            pressure: vec![ 101325.0 ],
            consumption: vec![ value ],
            loc: Location::new( 0.0, 0.0 ),
            r: 20.0,
            selected: false,
            events: vec![],
        }
    }

    pub fn new_elevation( id: usize, elevation: f64 ) -> Self {
        Flow {
            id,
            elevation,
            pressure: vec![ 101325.0 ],
            consumption: vec![ -0.1 ],
            loc: Location::new( 0.0, 0.0 ),
            r: 20.0,
            selected: false,
            events: vec![],
        }
    }

    pub fn create_transient_values(&mut self, tnodes: &[f64]) {
        let pressure = vec![ self.pressure[0]; tnodes.len() ];
        let mut consumption = vec![ self.consumption[0]; tnodes.len() ];
        for (i, t) in tnodes.iter().enumerate() {
            for event in self.events.iter() {
                if *t >= event.time()  {
                    consumption[i] = event.value();
                } 
            }
        }
        self.pressure = pressure;
        self.consumption = consumption;
    }
}