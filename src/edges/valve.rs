use std::f64::consts::PI;
use crate::node::Node;
use crate::fluid::Fluid;

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct Valve {
    pub from: Node,
    pub to: Node,
    pub mass_flow: f64,
    pub diameter: f64,
    pub thickness: f64,
    pub youngs_modulus: f64,
    pub k: Vec<(f64, f64)>, // (% open, k)
    pub open_percent: f64,
    pub closing_time: f64,
    pub profile_exponent: f64,
}

impl Valve {
    pub fn new(from: Node, to: Node ) -> Self {
        Valve { 
            from, 
            to, 
            mass_flow: 0.0,
            diameter: 52.5e-3,
            thickness: 0.005, // 5mm pipe
            youngs_modulus: 2.0e11, // Steel pipe TODO should be able to modify
            open_percent: 1.0,
            k: vec![ 
                (0.000, 1.0e16),
                (0.111, 700.),
                (0.222, 160.),
                (0.333, 60.),
                (0.444, 23.),
                (0.555, 7.9),
                (0.666, 3.),
                (0.777, 1.4),
                (0.888, 0.5),
                (1.000, 0.25),
            ],
            closing_time: 2.1,
            profile_exponent: 1.5,
        }
    }

    pub fn k(&self) -> f64 {
        self.interpolate_k( self.open_percent )
    }

    pub fn interpolate_k(&self, open_percent: f64 ) -> f64 {
        //let k = self.valve_type.valve_data();
        let mut openlower = self.k[0].0;
        let mut openupper = self.k[1].0;

        let mut klower = self.k[0].1;
        let mut kupper = self.k[1].1;
        
        for (index, k_value) in self.k.iter().enumerate() {
            if k_value.0 < open_percent {
                openlower = self.k[index].0;
                klower = self.k[index].1;
            } else {
                openupper = self.k[index].0;
                kupper = self.k[index].1;
                break;
            }
        }

        //println!("klower: {:+e}, kupper: {:+e}, openlower: {}, openupper: {}", klower, kupper, openlower, openupper);

        if openlower == openupper {
            klower
        } else {
            let dy = klower.ln() - kupper.ln();
            let dx = openlower - openupper;
            let m = dy / dx;
            let y = kupper.ln() + m * (open_percent - openupper);
            y.exp()
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

    //TODO should get k at time step for transient solve
    pub fn resistance(&self, flow_rate: f64, _nu: f64, g: f64 ) -> f64 {
        - self.k() * flow_rate * flow_rate.abs() / ( 2. * g * self.area() * self.area()  )
    }

    pub fn k_laminar(&self, nu: f64 ) -> f64 {
        2. * 9.806 * self.area() * self.area() / ( self.k() * 10.0 * nu )
    }

    pub fn darcy_approx(&self, head_loss: f64, g: f64 ) -> f64 {
        let f = 0.1;        // assumed friction factor for initial guess
        let a = self.area();
        let result = 2.0 * g * self.diameter * a * a / ( f * 10.0 * head_loss.abs() );
        result.sqrt()
    }
}