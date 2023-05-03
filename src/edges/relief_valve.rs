use std::f64::consts::PI;
use crate::node::Node;
use crate::utility;

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct ReliefValve {
    pub from: Node,
    pub to: Node,
    pub mass_flow: Vec<f64>,
    pub diameter: f64,
    pub thickness: f64,
    pub youngs_modulus: f64,
    pub invk: Vec<(f64, f64)>, // (% open, k^-1)
    pub open_dp: Vec<(f64, f64)>, // ( dp, % open )
    pub open_percent: Vec<f64>, 
    pub width: f32,
    pub selected: bool,
}

impl ReliefValve {
    pub fn new(from: Node, to: Node, dp_open: f64, dp_full: f64 ) -> Self {
        ReliefValve { 
            from, 
            to, 
            mass_flow: vec![ 0.0 ],
            diameter: 52.5e-3,
            thickness: 0.005, // 5mm pipe
            youngs_modulus: 2.0e11, // Steel pipe TODO should be able to modify
            invk: default_relief_valve_data(),
            open_dp: default_relief_valve_open_data( dp_open, dp_full ),
            open_percent: vec![ 0.0 ], // closed by default
            width: 15.0, 
            selected: false,
        }
    }

    pub fn invk_values(&mut self) -> &mut Vec<(f64, f64)> {
        &mut self.invk
    }

    pub fn open_dp_values(&mut self) -> &mut Vec<(f64, f64)> {
        &mut self.open_dp
    }

    pub fn invk(&self, step: usize ) -> f64 {
        self.interpolate_invk( self.open_percent[step] )
    }

    // Linearly interpolate between two points of (open_percent, invk)
    pub fn interpolate_invk(&self, open_percent: f64 ) -> f64 {
        let ( open, invk ) = utility::split_into_two_vectors( &self.invk );
        utility::interpolate( open_percent, &open, &invk )
    }

    pub fn area(&self) -> f64 {
        PI * self.diameter * self.diameter / 4.0
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

        let open_percent = self.open_percent_from_dp( dp );
        self.open_percent.push( open_percent );
    }

    pub fn open_percent_from_dp( &self, dp: f64 ) -> f64 {
        if dp < self.open_dp[0].0 {  // Closed if less than dp_open
            return 0.0;
        }
        let n = self.open_dp.len() - 1;
        if dp > self.open_dp[ n ].0 { // Fully open if greater than dp_full
            return 1.0;
        }
        // Interpolate between dp_open and dp_full
        let ( dp_vec, open_vec ) = utility::split_into_two_vectors( &self.open_dp );
        utility::interpolate( dp, &dp_vec, &open_vec )
    }

}

fn default_relief_valve_data( ) -> Vec<(f64, f64)> {
    vec![ 
        ( 0.0, 0.0 ),       
        ( 1.0, 1. / 0.25 ),  
    ]
}

// Default data gives linear interpolation between dp_open and dp_full
fn default_relief_valve_open_data( dp_open: f64, dp_full: f64 ) -> Vec<(f64, f64)> {
    vec![ 
        ( dp_open, 0.0 ),       
        ( dp_full, 1.0 ),  
    ]
}