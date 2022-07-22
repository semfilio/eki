use std::f64::consts::PI;
use crate::node::Node;
use crate::fluid::Fluid;
use crate::utility;

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct Pipe {
    pub from: Node,
    pub to: Node,
    pub mass_flow: Vec<f64>,        // [kg/s]
    pub length: f64,                // [m]
    pub diameter: f64,              // [m]
    pub roughness: f64,             // [m]
    pub thickness: f64,             // [m]
    pub youngs_modulus: f64,        // [Pa]
    pub width: f32,
    pub selected: bool,
}

impl Pipe {
    pub fn new(from: Node, to: Node ) -> Self {
        Pipe { 
            from, 
            to, 
            mass_flow: vec![ 0.0 ],
            length: 10.0, 
            diameter: 52.5e-3,
            roughness: 0.05e-3,
            thickness: 5.0e-3, // 5mm pipe
            youngs_modulus: 2.0e11, // Steel pipe
            width: 5.0, 
            selected: false,
        }
    }

    pub fn new_params(from: Node, to: Node, length: f64, diameter: f64, roughness: f64, 
        thickness: f64, youngs_modulus: f64 ) -> Self 
    {
        Pipe { 
            from, 
            to, 
            mass_flow: vec![ 0.0 ],
            length, 
            diameter,
            roughness,
            thickness,
            youngs_modulus,
            width: 5.0, 
            selected: false,
        }
    }

    pub fn area(&self) -> f64 {
        PI * self.diameter * self.diameter / 4.0
    }

    pub fn wave_speed(&self, fluid: &Fluid ) -> f64 {
        let k_over_rho: f64 =  fluid.bulk_modulus() / fluid.density();
        let dk: f64 = self.diameter * fluid.bulk_modulus();
        let te: f64 = self.thickness * self.youngs_modulus;
        let a = k_over_rho / ( 1.0 + ( dk / te ) );
        a.sqrt()
    }

    pub fn reynolds(&self, flow_rate: f64, nu: f64 ) -> f64 {
        flow_rate * self.diameter / ( self.area() * nu )
    }

    pub fn friction_factor(&self, flow_rate: f64, nu: f64 ) -> f64 {
        let relative: f64 = self.roughness / self.diameter;
        let re = self.reynolds( flow_rate, nu );
        utility::friction_factor( relative, re )
    }

    pub fn resistance(&self, flow_rate: f64, nu: f64, g: f64 ) -> f64 {
        if flow_rate == 0.0 {
            0.0
        } else {
            let friction = self.friction_factor( flow_rate.abs(), nu );
            let r = - friction * flow_rate * flow_rate.abs() / ( 2. * self.diameter * self.area() );
            self.length * r / ( g * self.area() )
        }
    }

    pub fn k_laminar(&self, nu: f64 ) -> f64 {
        PI * 9.806 * self.diameter.powi( 4 ) / ( 128.0 * self.length * nu )
    }

    pub fn darcy_approx(&self, head_loss: f64, g: f64 ) -> f64 {
        let f = 0.1;        // assumed friction factor for initial guess
        let a = self.area();
        let result = 2.0 * g * self.diameter * a * a / ( f * self.length * head_loss.abs() );
        result.sqrt()
    }

}