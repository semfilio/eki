use std::f64::consts::PI;
use crate::node::Node;

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct SizeChange {
    pub from: Node,
    pub to: Node,
    pub mass_flow: Vec<f64>,        // [kg/s]
    pub diameter: f64,              // From diameter [m]
    pub beta: f64,                  // Ratio to diameter / from diameter (d_k / d_i) 
    pub width: f32,
    pub selected: bool,
}

impl SizeChange {
    pub fn new(from: Node, to: Node ) -> Self {
        SizeChange { 
            from, 
            to, 
            mass_flow: vec![ 0.0 ],
            diameter: 52.5e-3,
            beta: 1.0,              // < 1 means contraction, > 1 means expansion
            width: 15.0, 
            selected: false,
        }
    }

    pub fn new_params(from: Node, to: Node, diameter: f64, beta: f64 ) -> Self 
    {
        SizeChange { 
            from, 
            to, 
            mass_flow: vec![ 0.0 ],
            diameter,
            beta,
            width: 15.0, 
            selected: false,
        }
    }

    pub fn area(&self) -> f64 {
        PI * self.diameter * self.diameter / 4.0
    }

    fn lambda( beta: f64 ) -> f64 {
        1.0 + 0.622 * ( 1.0 - 0.215 * beta.powi(2) - 0.785 * beta.powi(5) )
    }

    fn k_contraction( beta: f64 ) -> f64 {
        let lambda = Self::lambda( beta );
        0.0696 * ( 1. - beta.powi(2) ) * lambda.powi(2) + ( lambda - 1.0 ).powi(2)
    }

    fn k_expansion( beta: f64 ) -> f64 {
        ( 1. - beta.powi(2) ).powi(2)
    }

    pub fn resistance(&self, q: f64, dh: f64, _nu: f64, g: f64 ) -> f64 {
        let mut area = self.area();
        let k;
        if q < 0.0 {
            area *= self.beta * self.beta;
            if self.beta < 1.0 {
                k = - Self::k_expansion( self.beta );
            } else {
                k = - Self::k_contraction( 1.0 / self.beta );
            }
        } else { // q >= 0.0
            if self.beta < 1.0 {
                k = Self::k_contraction( self.beta );
            } else {
                k = Self::k_expansion( 1.0 / self.beta );
            }
        }
        - ( k * q * q / ( 2. * area ) ) + g * area * dh
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