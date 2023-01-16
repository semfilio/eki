use crate::location::Location;
use std::f64::consts::PI;

#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub struct Tank {
    pub id: usize,
    pub elevation: f64,
    pub pressure: Vec<f64>,
    pub consumption: Vec<f64>,
    pub z_init: f64,                // Initial fluid level [m]
    pub z_min: f64,                 // Minimum fluid level [m]
    pub z_max: f64,                 // Maximum fluid level [m]
    pub diameter: f64,              // Tank diameter [m]
    pub loc: Location,
    pub r: f32,
    pub selected: bool,
}

impl Default for Tank {
    fn default() -> Self {
        Tank::new( 0, 101325.0, 1000.0, 9.80665 )
    }
}

impl Tank {
    pub fn new( id: usize, p_atm: f64, rho: f64, g: f64 ) -> Self {
        Tank {
            id,
            elevation: 0.0,
            pressure: vec![ p_atm + rho * g * 0.5  ],
            consumption: vec![ 0.0 ],
            z_init: 0.5,
            z_min: 0.0,
            z_max: 1.0,
            diameter: 1.0,
            loc: Location::new( 0.0, 0.0 ),
            r: 20.0,
            selected: false,
        }
    }

    pub fn new_with_values( id: usize, p_atm: f64, rho: f64, g: f64, diameter: f64, 
        z_init: f64, z_min: f64, z_max: f64 
    ) -> Self {
        Tank {
            id,
            elevation: 0.0,
            pressure: vec![ p_atm + rho * g * z_init ],
            consumption: vec![ 0.0 ],
            z_init,
            z_min,
            z_max,
            diameter,
            loc: Location::new( 0.0, 0.0 ),
            r: 20.0,
            selected: false,
        }
    }

    pub fn area( &self ) -> f64 {
        0.25 * PI * self.diameter * self.diameter
    }

}