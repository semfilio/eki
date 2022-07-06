use std::f64::consts::PI;
use crate::node::Node;
use crate::fluid::Fluid;
use crate::events::TransientEvent;

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct Pump {
    pub from: Node,
    pub to: Node,
    pub mass_flow: Vec<f64>,
    pub c: Vec<f64>,                // Vector of coefficients
    pub q_rated: f64,               // Rated volume flow rate [m^3 / s] //TODO is this needed ??
    pub h_rated: f64,               // Rated head [m]                   //TODO is this needed ??
    pub n_rated: f64,               // Rated speed [rpm]
    pub d_rated: f64,               // Rated impeller diameter [m]
    pub diameter: f64,              // Impeller diameter [m]
    pub speed:  f64,                // Speed [rpm]                      //TODO maybe this should be a vector of speeds ???
    pub thickness: f64,             // [m]
    pub youngs_modulus: f64,        // [Pa]
    pub events: Vec<TransientEvent>,
    pub width: f32,
    pub selected: bool,
}

impl Pump {
    pub fn new( from: Node, to: Node ) -> Self {
        Pump {
            from,
            to,
            mass_flow: vec![ 0.0 ],
            c: vec![ 1.0, 0.0, -1.0 ],
            q_rated: 50.0 / (60.0 * 60.0),          // 50m^3 / hour
            h_rated: 50.0,                          // 50m
            n_rated: 2950.0,                        // 2950 rpm
            d_rated: 163.0e-3,                      // 163mm
            diameter: 163.0e-3,                     // 163mm
            speed: 2950.0,
            thickness: 5.0e-3,                      // 5mm
            youngs_modulus: 2.0e11,                 // Steel
            events: vec![],
            width: 15.0, 
            selected: false,
        }
    }

    pub fn new_params( from: Node, to: Node, c: Vec<f64>, rated: (f64, f64, f64, f64) ) -> Self {
        Pump {
            from,
            to,
            mass_flow: vec![ 0.0 ],
            c,
            q_rated: rated.0,         
            h_rated: rated.1,
            n_rated: rated.2,
            d_rated: rated.3,                      
            diameter: rated.3,                     
            speed: rated.2,
            thickness: 5.0e-3,                      // 5mm
            youngs_modulus: 2.0e11,                 // Steel
            events: vec![],
            width: 15.0, 
            selected: false,
        }
    }

    // TODO make from data (Q,dH) -> regression fit polynomial


    // TODO maybe we should only calculate the coefficients once unless something changes
    pub fn c_affinity(&self) -> Vec<f64> {
        let xi = self.n_rated * self.d_rated / ( self.speed * self.diameter );
        let mut c_dash = self.c.clone();
        for i in 0..c_dash.len() {
            c_dash[i] *= xi.powi( 2 - i as i32 );
        }
        c_dash
    }

    pub fn resistance(&self, flow_rate: f64, _nu: f64, _g: f64 ) -> f64 {
        // Evaluate polynomial using Horner's method
        self.c_affinity().iter()
        .rev()
        .fold( 0.0, |acc, coeff| acc * flow_rate.clone() + coeff.clone())
    }

    //TODO interpolation utility function
    //TODO what about negative values and out of range values
    /*pub fn interpolate_data(&self, flow_rate: f64 ) -> f64 {
        let mut xlower = self.head_data[0].0;
        let mut xupper = self.head_data[1].0;

        let mut ylower = self.head_data[0].1;
        let mut yupper = self.head_data[1].1;
        
        for (index, k_value) in self.head_data.iter().enumerate() {
            if k_value.0 < flow_rate {
                xlower = self.head_data[index].0;
                ylower = self.head_data[index].1;
            } else {
                xupper = self.head_data[index].0;
                yupper = self.head_data[index].1;
                break;
            }
        }

        if xlower == xupper {
            ylower
        } else {
            let dy = ylower - yupper;
            let dx = xlower - xupper;
            let m = dy / dx;
            let y = yupper + m * (flow_rate - xupper);
            y
        }
    }*/

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

    //TODO ???
    pub fn k_laminar(&self, nu: f64 ) -> f64 {
        PI * 9.806 * self.diameter.powi( 4 ) / ( 128.0 * 1.0 * nu )
        /*let design_flow = 50. / ( 60. * 60. );
        let design_head = 50.;
        - design_flow / design_head*/
        //2.14e-4
    }

    //TODO ???
    pub fn darcy_approx(&self, head_loss: f64, g: f64 ) -> f64 {
        let f = 0.1;        // assumed friction factor for initial guess
        let a = self.area();
        let result = 2.0 * g * self.diameter * a * a / ( f * 1.0 * head_loss.abs() );
        result.sqrt()
        //println!( "head_loss = {}", head_loss );
        //- self.interpolate_head( head_loss.abs() )
    }

    /*pub fn interpolate_head(&self, head_loss: f64 ) -> f64 {
        let mut xlower = self.head_data[0].1;
        let mut xupper = self.head_data[1].1;

        let mut ylower = self.head_data[0].0;
        let mut yupper = self.head_data[1].0;
        
        for (index, k_value) in self.head_data.iter().enumerate() {
            if k_value.0 < head_loss {
                xlower = self.head_data[index].1;
                ylower = self.head_data[index].0;
            } else {
                xupper = self.head_data[index].1;
                yupper = self.head_data[index].0;
                break;
            }
        }

        if xlower == xupper {
            ylower
        } else {
            let dy = ylower - yupper;
            let dx = xlower - xupper;
            let m = dy / dx;
            let y = yupper + m * (head_loss - xupper);
            y
        }
    }*/

}