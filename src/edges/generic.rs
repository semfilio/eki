// A generic resistance component of the form dH = A + B * Q^n + C * Q^m
use std::f64::consts::PI;
use crate::node::Node;
use crate::fluid::Fluid;

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct Generic {
    pub from: Node,
    pub to: Node,
    pub mass_flow: Vec<f64>,            // [kg/s]
    pub coefficients: (f64, f64, f64),  // Coefficients ( A, B, C )   
    pub exponents: (f64, f64),          // Exponents ( n, m )
    pub diameter: f64,                  // [m] TODO this should be removed
    pub width: f32,
    pub selected: bool,
}

impl Generic {
    pub fn new(from: Node, to: Node ) -> Self {
        Generic { 
            from, 
            to, 
            mass_flow: vec![ 0.0 ],
            coefficients: ( 0.0, 0.0, 0.0 ),
            exponents: ( 0.0, 0.0 ),
            diameter: 52.5e-3,
            width: 5.0, 
            selected: false,
        }
    }

    pub fn new_params(from: Node, to: Node, coefficients: (f64, f64, f64),
        exponents: (f64, f64) ) -> Self 
    {
        Generic { 
            from, 
            to, 
            mass_flow: vec![ 0.0 ],
            coefficients,
            exponents,
            diameter: 52.5e-3,
            width: 5.0, 
            selected: false,
        }
    }

    //TODO need to access coefficients and exponents

    pub fn area(&self) -> f64 {
        1.0
    }

    pub fn wave_speed(&self, _fluid: &Fluid ) -> f64 {
        0.0 //TODO should be infinity
    }

    pub fn resistance(&self, q: f64, dh: f64, _nu: f64, g: f64 ) -> f64 {
        let ( a, b, c ) = self.coefficients;
        let ( n, m ) = self.exponents;
        let q_abs = q.abs();
        let r =  a + b * q * q_abs.powf( n - 1.0 ) + c * q * q_abs.powf( m - 1.0 );
        - g * self.area() *  r + g * self.area() * dh 
    }

    // TODO what would be an appropraite laminar term?
    pub fn k_laminar(&self, nu: f64 ) -> f64 {
        PI * 9.806 * self.diameter.powi( 4 ) / ( 128.0 * nu )
    }

    pub fn darcy_approx(&self, head_loss: f64, g: f64 ) -> f64 {
        let f = 0.1;        // assumed friction factor for initial guess
        let a = self.area();
        let result = 2.0 * g * self.diameter * a * a / ( f * head_loss.abs() );
        result.sqrt()
    }

}