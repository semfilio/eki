use std::f64::consts::PI;
use crate::node::Node;
use crate::fluid::Fluid;

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct SafetyValve {
    pub from: Node,
    pub to: Node,
    pub mass_flow: Vec<f64>,
    pub diameter: f64,
    pub thickness: f64,
    pub youngs_modulus: f64,
    pub set_dp: f64,
    pub invk: Vec<(f64, f64)>, // (% open, k^-1)
    pub open_percent: Vec<f64>,
    pub width: f32,
    pub selected: bool,
}

impl SafetyValve {
    pub fn new(from: Node, to: Node, set_dp: f64 ) -> Self {
        SafetyValve { 
            from, 
            to, 
            mass_flow: vec![ 0.0 ],
            diameter: 52.5e-3,
            thickness: 0.005, // 5mm pipe
            youngs_modulus: 2.0e11, // Steel pipe TODO should be able to modify
            set_dp,
            invk: default_safety_valve_data(),
            open_percent: vec![ 0.0 ], // closed by default
            width: 15.0, 
            selected: false,
        }
    }

    pub fn invk_values(&mut self) -> &mut Vec<(f64, f64)> {
        &mut self.invk
    }

    pub fn invk(&self, step: usize ) -> f64 {
        if self.open_percent[step] < 1.0 { // If closed 
            0.0
        } else {
            self.invk[1].1
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

    pub fn resistance(&self, q: f64, dh: f64, _nu: f64, g: f64, step: usize ) -> f64 {
        - ( q * q.abs() / ( 2. * self.area()  ) ) + self.invk(step) * g * self.area() * dh
    }

    pub fn b_coefficient(&self, step: usize ) -> f64 {
        self.invk(step)
    }

    pub fn k_laminar(&self, nu: f64 ) -> f64 {
        let f = 0.1; // assumed friction factor for initial guess
        let equivalent_length = ( 1.0 / self.invk( 0 ) ) * self.diameter / f;
        PI * 9.806 * self.diameter.powi( 4 ) / ( 128.0 * equivalent_length * nu )
    }

    pub fn darcy_approx(&self, head_loss: f64, g: f64 ) -> f64 {
        let a = self.area();
        let result = 2.0 * g * a * a / ( ( 1.0 / self.invk( 0 ) ) * head_loss.abs() );
        result.sqrt()
    }

    pub fn add_transient_value( &mut self, _time: f64 ) {
        let step = self.open_percent.len() - 1;
        let p_from = self.from.pressure()[ step ];
        let p_to = self.to.pressure()[ step ];
        let dp = p_from - p_to;
        if dp > self.set_dp {
            self.open_percent.push( 1.0 )
        } else {
            self.open_percent.push( 0.0 );
        }
    }

}

fn default_safety_valve_data() -> Vec<(f64, f64)> {
    vec![ 
        ( 0., 0.0 ),
        ( 1., 1. / 0.25 ),
    ]
}