use std::f64::consts::PI;
use crate::node::Node;
use crate::fluid::Fluid;
use crate::utility;

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct Bend {
    pub from: Node,
    pub to: Node,
    pub mass_flow: Vec<f64>,        // [kg/s]
    pub radius: f64,                // Bend radius [m]
    pub diameter: f64,              // [m]
    pub angle: f64,                 // Bend angle (alpha) [rad]
    pub roughness: f64,             // [m]
    pub thickness: f64,             // [m]
    pub youngs_modulus: f64,        // [Pa]
    pub width: f32,
    pub selected: bool,
}

impl Bend {
    pub fn new(from: Node, to: Node ) -> Self {
        Bend { 
            from, 
            to, 
            mass_flow: vec![ 0.0 ],
            radius: 52.5e-3,                // 52.5mm bend radius
            diameter: 52.5e-3,              // 52.5mm 
            angle: PI / 2.0,                // 90 degree bend angle
            roughness: 0.05e-3,             // 0.05mm
            thickness: 5.0e-3,              // 5mm pipe
            youngs_modulus: 2.0e11,         // Steel pipe
            width: 5.0, 
            selected: false,
        }
    }

    pub fn new_params(from: Node, to: Node, radius: f64, diameter: f64, angle: f64, roughness: f64, 
        thickness: f64, youngs_modulus: f64 ) -> Self 
    {
        Bend { 
            from, 
            to, 
            mass_flow: vec![ 0.0 ],
            radius, 
            diameter,
            angle,
            roughness,
            thickness,
            youngs_modulus,
            width: 5.0, 
            selected: false,
        }
    }

    pub fn length(&self) -> f64 {
        self.radius * self.angle
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

    pub fn k(&self, flow_rate: f64, nu: f64 ) -> f64 {
        let f = self.friction_factor( flow_rate, nu );
        let rd = self.radius / self.diameter;
        let s = ( 0.5 * self.angle ).sin();
        let pow = rd.powf( 4. * self.angle / PI );
        f * self.angle * rd + ( 0.1 + 2.4 * f ) * s + ( 6.6 * f * ( s.sqrt() + s ) / pow ) 
    }

    pub fn resistance(&self, flow_rate: f64, nu: f64, g: f64 ) -> f64 {
        if flow_rate == 0.0 {
            0.0
        } else {
            let k = self.k( flow_rate, nu );
            - k * flow_rate * flow_rate.abs() / ( 2. * g * self.area() * self.area() )
        }
    }

    //TODO: implement k_laminar
    pub fn k_laminar(&self, nu: f64 ) -> f64 {
        PI * 9.806 * self.diameter.powi( 4 ) / ( 128.0 * self.length() * nu )
    }

    //TODO: implement darcy_approx
    pub fn darcy_approx(&self, head_loss: f64, g: f64 ) -> f64 {
        let f = 0.1;        // assumed friction factor for initial guess
        let a = self.area();
        let result = 2.0 * g * self.diameter * a * a / ( f * self.length() * head_loss.abs() );
        result.sqrt()
    }

}