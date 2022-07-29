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
    pub head_data: Vec<(f64, f64)>,     // ( theta [rad], F_h )
    pub torque_data: Vec<(f64, f64)>,   // ( theta [rad], F_tau )
    pub q_rated: f64,                   // Rated volume flow rate [m^3 / s]
    pub h_rated: f64,                   // Rated head [m]                   
    pub n_rated: f64,                   // Rated speed [rpm]
    pub diameter: f64,                  // Impeller diameter [m]
    pub speed:  Vec<f64>,               // Speed [rpm] at each time step
    pub thickness: f64,                 // [m]
    pub youngs_modulus: f64,            // [Pa]
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
            head_data: default_head_data(),
            torque_data: default_torque_data(),
            q_rated: 600.0 / (60.0 * 60.0),         // 600m^3 / hour
            h_rated: 330.0,                         // 330m
            n_rated: 11300.0,                       // 11300 rpm
            diameter: 163.0e-3,                     // 163mm
            speed: vec![ 11300.0 ],                 // 11300 rpm
            thickness: 5.0e-3,                      // 5mm
            youngs_modulus: 2.0e11,                 // Steel
            events: vec![],
            width: 15.0, 
            selected: false,
            //TODO do we need a max/min speed?
        }
    }

    /*pub fn new_params( from: Node, to: Node, c: Vec<f64>, rated: (f64, f64, f64, f64) ) -> Self {
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
            speed: vec![ rated.2 ],
            thickness: 5.0e-3,                      // 5mm
            youngs_modulus: 2.0e11,                 // Steel
            min_diameter: rated.3,                 
            max_diameter: rated.3,                 
            min_speed: rated.2,                     
            max_speed: rated.2,                     
            events: vec![],
            width: 15.0, 
            selected: false,
        }
    }*/

    pub fn n(&self, step: usize ) -> f64 {
        self.speed[ step ] / self.n_rated
    }

    pub fn theta( n: f64, q: f64 ) -> f64 {
        let mut theta = n.atan2( q );
        if theta < 0.0 {
            theta += 2.0 * PI;
        }
        theta
    }

    //TODO interpolation utility function -> split head and torque data into separate vectors
    pub fn f_h(&self, theta: f64 ) -> f64 {
        let mut xlower = self.head_data[0].0;
        let mut xupper = self.head_data[1].0;
        let mut ylower = self.head_data[0].1;
        let mut yupper = self.head_data[1].1;
        
        for (index, value) in self.head_data.iter().enumerate() {
            if value.0 < theta {
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
            let y = yupper + m * (theta - xupper);
            y
        }
    }

    pub fn resistance(&self, flow_rate: f64, _nu: f64, _g: f64, step: usize ) -> f64 {
        let q = flow_rate / self.q_rated;
        let n = self.n( step );
        let theta = Pump::theta( n, q );
        self.h_rated * ( n * n + q * q ) * self.f_h( theta )
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
    pub fn k_laminar(&self, _nu: f64 ) -> f64 {
        //PI * 9.806 * self.diameter.powi( 4 ) / ( 128.0 * 1.0 * nu )
        - self.q_rated / self.h_rated
        //0.0
    }

    //TODO ???
    pub fn darcy_approx(&self, head_loss: f64, _g: f64 ) -> f64 {
        /*let f = 0.1;        // assumed friction factor for initial guess
        let a = self.area();
        let result = 2.0 * g * self.diameter * a * a / ( f * 1.0 * head_loss.abs() );
        - result.sqrt()*/
        //println!( "head_loss = {}", head_loss );
        //- self.interpolate_head( head_loss.abs() / self.h_rated )
        let r = self.resistance( 0.0, 0.0, 0.0, 0 );
        let delta = 1.0e-8;
        let rplus = self.resistance( delta, 0.0, 0.0, 0 );
        let rminus = self.resistance( -delta, 0.0, 0.0, 0 );
        let rd = ( rplus - rminus ) / ( 2.0 * delta );
        let rdd = ( rplus + rminus - 2.0 * r ) / ( delta.powi( 2 ));
        let disc = ( rd * rd - 4.0 * rdd * ( r - head_loss ) ).sqrt();
        ( -rd + disc ) / ( 2.0 * rdd )
        //( head_loss - r ) / rd
        //-0.001
    }

    pub fn add_transient_value( &mut self, time: f64 ) {
        let steady = self.speed[0];
        for event in self.events.iter() {
            self.speed.push( event.pump_speed( time, steady ) );
        }
        if self.events.len() == 0 {
            self.speed.push( *self.speed.last().unwrap() );
        }
    }

    pub fn interpolate_head(&self, h_loss: f64 ) -> f64 {
        let mut xlower = self.head_data[0].1;
        let mut xupper = self.head_data[1].1;

        let mut ylower = self.head_data[0].0;
        let mut yupper = self.head_data[1].0;
        
        for (index, k_value) in self.head_data.iter().enumerate() {
            if k_value.0 < h_loss {
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
            let y = yupper + m * (h_loss - xupper);
            y
        }
    }

}

// N_s = 0.46 from Chaudry p. 523-24
fn default_head_data() -> Vec<(f64, f64)> {
    vec![ 
        ( (0.0_f64).to_radians(), -0.55 ),
        ( (5.0_f64).to_radians(), -0.48 ),
        ( (10.0_f64).to_radians(), -0.38 ),
        ( (15.0_f64).to_radians(), -0.27 ),
        ( (20.0_f64).to_radians(), -0.17 ),
        ( (25.0_f64).to_radians(), -0.09 ),
        ( (30.0_f64).to_radians(), 0.06 ),
        ( (35.0_f64).to_radians(), 0.22 ),
        ( (40.0_f64).to_radians(), 0.37 ),
        ( (45.0_f64).to_radians(), 0.50 ),
        ( (50.0_f64).to_radians(), 0.64 ),
        ( (55.0_f64).to_radians(), 0.78 ),
        ( (60.0_f64).to_radians(), 0.91 ),
        ( (65.0_f64).to_radians(), 1.03 ),
        ( (70.0_f64).to_radians(), 1.13 ),
        ( (75.0_f64).to_radians(), 1.21 ),
        ( (80.0_f64).to_radians(), 1.27 ),
        ( (85.0_f64).to_radians(), 1.33 ),
        ( (90.0_f64).to_radians(), 1.35 ),
        ( (95.0_f64).to_radians(), 1.36 ),
        ( (100.0_f64).to_radians(), 1.34 ),
        ( (105.0_f64).to_radians(), 1.31 ),
        ( (110.0_f64).to_radians(), 1.28 ),
        ( (115.0_f64).to_radians(), 1.22 ),
        ( (120.0_f64).to_radians(), 1.17 ),
        ( (125.0_f64).to_radians(), 1.13 ),
        ( (130.0_f64).to_radians(), 1.09 ),
        ( (135.0_f64).to_radians(), 1.04 ),
        ( (140.0_f64).to_radians(), 0.99 ),
        ( (145.0_f64).to_radians(), 0.96 ),
        ( (150.0_f64).to_radians(), 0.91 ),
        ( (155.0_f64).to_radians(), 0.89 ),
        ( (160.0_f64).to_radians(), 0.85 ),
        ( (165.0_f64).to_radians(), 0.82 ),
        ( (170.0_f64).to_radians(), 0.79 ),
        ( (175.0_f64).to_radians(), 0.75 ),
        ( (180.0_f64).to_radians(), 0.71 ),
        ( (185.0_f64).to_radians(), 0.68 ),
        ( (190.0_f64).to_radians(), 0.65 ),
        ( (195.0_f64).to_radians(), 0.61 ),
        ( (200.0_f64).to_radians(), 0.58 ),
        ( (205.0_f64).to_radians(), 0.55 ),
        ( (210.0_f64).to_radians(), 0.54 ),
        ( (215.0_f64).to_radians(), 0.53 ),
        ( (220.0_f64).to_radians(), 0.52 ),
        ( (225.0_f64).to_radians(), 0.52 ),
        ( (230.0_f64).to_radians(), 0.53 ),
        ( (235.0_f64).to_radians(), 0.55 ),
        ( (240.0_f64).to_radians(), 0.57 ),
        ( (245.0_f64).to_radians(), 0.59 ),
        ( (250.0_f64).to_radians(), 0.61 ),
        ( (255.0_f64).to_radians(), 0.63 ),
        ( (260.0_f64).to_radians(), 0.64 ),
        ( (265.0_f64).to_radians(), 0.66 ),
        ( (270.0_f64).to_radians(), 0.66 ),
        ( (275.0_f64).to_radians(), 0.62 ),
        ( (280.0_f64).to_radians(), 0.51 ),
        ( (285.0_f64).to_radians(), 0.32 ),
        ( (290.0_f64).to_radians(), 0.23 ),
        ( (295.0_f64).to_radians(), 0.11 ),
        ( (300.0_f64).to_radians(), -0.20 ),
        ( (305.0_f64).to_radians(), -0.31 ),
        ( (310.0_f64).to_radians(), -0.39 ),
        ( (315.0_f64).to_radians(), -0.47 ),
        ( (320.0_f64).to_radians(), -0.53 ),
        ( (325.0_f64).to_radians(), -0.59 ),
        ( (330.0_f64).to_radians(), -0.64 ),
        ( (335.0_f64).to_radians(), -0.66 ),
        ( (340.0_f64).to_radians(), -0.68 ),
        ( (345.0_f64).to_radians(), -0.67 ),
        ( (350.0_f64).to_radians(), -0.66 ),
        ( (355.0_f64).to_radians(), -0.61 ),
        ( (360.0_f64).to_radians(), -0.55 ),
    ]
}

// N_s = 0.46 from Chaudry p. 523-24
fn default_torque_data() -> Vec<(f64, f64)> {
    vec![ 
        ( (0.0_f64).to_radians(), -0.43 ),
        ( (5.0_f64).to_radians(), -0.26 ),
        ( (10.0_f64).to_radians(), -0.11 ),
        ( (15.0_f64).to_radians(), -0.05 ),
        ( (20.0_f64).to_radians(), 0.04 ),
        ( (25.0_f64).to_radians(), 0.14 ),
        ( (30.0_f64).to_radians(), 0.25 ),
        ( (35.0_f64).to_radians(), 0.34 ),
        ( (40.0_f64).to_radians(), 0.42 ),
        ( (45.0_f64).to_radians(), 0.50 ),
        ( (50.0_f64).to_radians(), 0.55 ),
        ( (55.0_f64).to_radians(), 0.59 ),
        ( (60.0_f64).to_radians(), 0.61 ),
        ( (65.0_f64).to_radians(), 0.61 ),
        ( (70.0_f64).to_radians(), 0.60 ),
        ( (75.0_f64).to_radians(), 0.58 ),
        ( (80.0_f64).to_radians(), 0.55 ),
        ( (85.0_f64).to_radians(), 0.50 ),
        ( (90.0_f64).to_radians(), 0.44 ),
        ( (95.0_f64).to_radians(), 0.41 ),
        ( (100.0_f64).to_radians(), 0.37 ),
        ( (105.0_f64).to_radians(), 0.35 ),
        ( (110.0_f64).to_radians(), 0.34 ),
        ( (115.0_f64).to_radians(), 0.34 ),
        ( (120.0_f64).to_radians(), 0.36 ),
        ( (125.0_f64).to_radians(), 0.40 ),
        ( (130.0_f64).to_radians(), 0.47 ),
        ( (135.0_f64).to_radians(), 0.54 ),
        ( (140.0_f64).to_radians(), 0.62 ),
        ( (145.0_f64).to_radians(), 0.70 ),
        ( (150.0_f64).to_radians(), 0.77 ),
        ( (155.0_f64).to_radians(), 0.82 ),
        ( (160.0_f64).to_radians(), 0.86 ),
        ( (165.0_f64).to_radians(), 0.89 ),
        ( (170.0_f64).to_radians(), 0.91 ),
        ( (175.0_f64).to_radians(), 0.90 ),
        ( (180.0_f64).to_radians(), 0.88 ),
        ( (185.0_f64).to_radians(), 0.85 ),
        ( (190.0_f64).to_radians(), 0.82 ),
        ( (195.0_f64).to_radians(), 0.74 ),
        ( (200.0_f64).to_radians(), 0.67 ),
        ( (205.0_f64).to_radians(), 0.59 ),
        ( (210.0_f64).to_radians(), 0.50 ),
        ( (215.0_f64).to_radians(), 0.42 ),
        ( (220.0_f64).to_radians(), 0.33 ),
        ( (225.0_f64).to_radians(), 0.24 ),
        ( (230.0_f64).to_radians(), 0.16 ),
        ( (235.0_f64).to_radians(), 0.07 ),
        ( (240.0_f64).to_radians(), 0.01 ),
        ( (245.0_f64).to_radians(), -0.12 ),
        ( (250.0_f64).to_radians(), -0.21 ),
        ( (255.0_f64).to_radians(), -0.22 ),
        ( (260.0_f64).to_radians(), -0.35 ),
        ( (265.0_f64).to_radians(), -0.51 ),
        ( (270.0_f64).to_radians(), -0.68 ),
        ( (275.0_f64).to_radians(), -0.85 ),
        ( (280.0_f64).to_radians(), -1.02 ),
        ( (285.0_f64).to_radians(), -1.21 ),
        ( (290.0_f64).to_radians(), -1.33 ),
        ( (295.0_f64).to_radians(), -1.44 ),
        ( (300.0_f64).to_radians(), -1.56 ),
        ( (305.0_f64).to_radians(), -1.65 ),
        ( (310.0_f64).to_radians(), -1.67 ),
        ( (315.0_f64).to_radians(), -1.67 ),
        ( (320.0_f64).to_radians(), -1.63 ),
        ( (325.0_f64).to_radians(), -1.56 ),
        ( (330.0_f64).to_radians(), -1.44 ),
        ( (335.0_f64).to_radians(), -1.33 ),
        ( (340.0_f64).to_radians(), -1.18 ),
        ( (345.0_f64).to_radians(), -1.00 ),
        ( (350.0_f64).to_radians(), -0.83 ),
        ( (355.0_f64).to_radians(), -0.64 ),
        ( (360.0_f64).to_radians(), -0.43 ),
    ]
}