use std::f64::consts::PI;
use crate::node::Node;
use crate::fluid::Fluid;

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct Pump {
    pub from: Node,
    pub to: Node,
    pub mass_flow: Vec<f64>,
    pub head_data: Vec<(f64, f64)>, // (Q, dH)
    //pub a: Vec<f64>,            // Vector of coefficients
    pub diameter: f64,          // TODO ??? inlet/outlet diameter ???
    pub thickness: f64,         // TODO ???
    pub youngs_modulus: f64,    // TODO ???
}

impl Pump {
    pub fn new( from: Node, to: Node ) -> Self {
        Pump {
            from,
            to,
            mass_flow: vec![ 0.0 ],
            head_data: vec![ 
                (0.000, 1.0e16),
                (0.111, 700.),
                (0.222, 160.),
                (0.333, 60.),
                (0.444, 23.),
                (0.556, 7.9),
                (0.667, 3.),
                (0.778, 1.4),
                (0.889, 0.5),
                (1.000, 0.25),
            ],
            //a: vec![ 1.0, 0.0, -1.0 ],
            diameter: 1.0,                  // TODO ??? 
            thickness: 1.0,                 // TODO ???
            youngs_modulus: 1.0,            // TODO ???

        }
    }

    pub fn new_data( from: Node, to: Node, head_data: Vec<(f64, f64)> ) -> Self {
        Pump {
            from,
            to,
            mass_flow: vec![ 0.0 ],
            head_data,
            diameter: 1.0,                  // TODO ??? 
            thickness: 1.0,                 // TODO ???
            youngs_modulus: 1.0,            // TODO ???

        }
    }

    pub fn resistance(&self, flow_rate: f64, _nu: f64, _g: f64 ) -> f64 {
        //self.a[0] + self.a[1] * flow_rate + self.a[2] * flow_rate * flow_rate.abs()
        //println!( "flow_rate = {}", flow_rate );
        //let r = self.interpolate_data( flow_rate );
        //println!("r = {}", r );
        //r
        
        self.interpolate_data( flow_rate )
    }

    //TODO interpolation utility function
    //TODO what about negative values and out of range values
    pub fn interpolate_data(&self, flow_rate: f64 ) -> f64 {
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

    //TODO ???
    pub fn k_laminar(&self, nu: f64 ) -> f64 {
        PI * 9.806 * self.diameter.powi( 4 ) / ( 128.0 * 1.0 * nu )
        /*let design_flow = 50. / ( 60. * 60. );
        let design_head = 50.;
        - design_flow / design_head*/
        //2.14e-4
    }

    //TODO ???
    pub fn darcy_approx(&self, head_loss: f64, _g: f64 ) -> f64 {
        /*let f = 0.1;        // assumed friction factor for initial guess
        let a = self.area();
        let result = 2.0 * g * self.diameter * a * a / ( f * 1.0 * head_loss.abs() );
        result.sqrt()*/
        //println!( "head_loss = {}", head_loss );
        - self.interpolate_head( head_loss.abs() )
    }

    pub fn interpolate_head(&self, head_loss: f64 ) -> f64 {
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
    }

}