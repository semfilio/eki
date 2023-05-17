use std::f64::consts::PI;
use crate::node::Node;

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct OpenPipe {
    pub from: Node,
    pub to: Node,
    pub mass_flow: Vec<f64>,        // [kg/s]
    pub diameter: f64,              // From diameter [m]
    pub k: f64,                     // Loss coefficient 
    pub width: f32,
    pub selected: bool,
}

impl OpenPipe {
    pub fn new(from: Node, to: Node ) -> Self {
        OpenPipe { 
            from, 
            to, 
            mass_flow: vec![ 0.0 ],
            diameter: 52.5e-3,
            k: 1.0,              
            width: 15.0, 
            selected: false,
        }
    }

    pub fn new_params(from: Node, to: Node, diameter: f64, k: f64 ) -> Self 
    {
        OpenPipe { 
            from, 
            to, 
            mass_flow: vec![ 0.0 ],
            diameter,
            k,
            width: 15.0, 
            selected: false,
        }
    }

    pub fn k(&mut self) -> &mut f64 {
        &mut self.k
    }

    pub fn area(&self) -> f64 {
        PI * self.diameter * self.diameter / 4.0
    }

    pub fn resistance(&self, q: f64, dh: f64, _nu: f64, g: f64 ) -> f64 {
        - ( self.k * q * q / ( 2. * self.area() ) ) + g * self.area() * dh
    }

    //TODO
    pub fn k_laminar(&self, nu: f64 ) -> f64 {
        PI * 9.806 * self.diameter.powi( 4 ) / ( 128.0 * 1. * nu )
    }

    //TODO
    pub fn darcy_approx(&self, head_loss: f64, g: f64 ) -> f64 {
        let f = 0.1;        // assumed friction factor for initial guess
        let a = self.area();
        let result = 2.0 * g * self.diameter * a * a / ( f * 1. * head_loss.abs() );
        result.sqrt()
    }

}