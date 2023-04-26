use std::f64::consts::PI;
use crate::node::Node;
use crate::fluid::Fluid;
use crate::events::TransientEvent;

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "persistence", serde(default))]
pub enum ValveType {
    AngleGlobe,
    Ball,
    Butterfly,
    Diaphragm,
    Gate,
    Globe,
    Needle,
    Pinch,
    Plug,
    Slide,
}

impl Default for ValveType {
    fn default() -> Self {
        ValveType::Butterfly
    }
}

impl ValveType {
    pub fn text(&self) -> String {
        match self {
            ValveType::AngleGlobe => "Angle Globe".to_string(),
            ValveType::Ball => "Ball".to_string(),
            ValveType::Butterfly => "Butterfly".to_string(),
            ValveType::Diaphragm => "Diaphragm".to_string(),
            ValveType::Gate => "Gate".to_string(),
            ValveType::Globe => "Globe".to_string(),
            ValveType::Needle => "Needle".to_string(),
            ValveType::Pinch => "Pinch".to_string(),
            ValveType::Plug => "Plug".to_string(),
            ValveType::Slide => "Slide".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct Valve {
    pub from: Node,
    pub to: Node,
    pub mass_flow: Vec<f64>,
    pub diameter: f64,
    pub thickness: f64,
    pub youngs_modulus: f64,
    pub invk: Vec<(f64, f64)>, // (% open, k^-1)
    pub open_percent: Vec<f64>,
    pub events: Vec<TransientEvent>,
    pub width: f32,
    pub selected: bool,
    pub valve_type: ValveType,
}

impl Valve {
    pub fn new(from: Node, to: Node ) -> Self {
        Valve { 
            from, 
            to, 
            mass_flow: vec![ 0.0 ],
            diameter: 52.5e-3,
            thickness: 0.005, // 5mm pipe
            youngs_modulus: 2.0e11, // Steel pipe TODO should be able to modify
            invk: default_valve_data(),
            open_percent: vec![ 1.0 ],
            events: vec![],
            width: 15.0, 
            selected: false,
            valve_type: ValveType::Butterfly,
        }
    }

    pub fn invk_values(&mut self) -> &mut Vec<(f64, f64)> {
        &mut self.invk
    }

    pub fn invk(&self, step: usize ) -> f64 {
        self.interpolate_invk( self.open_percent[step] )
    }

    // Linearly interpolate between two points of (open_percent, invk)
    pub fn interpolate_invk(&self, open_percent: f64 ) -> f64 {
        let mut openlower = self.invk[0].0;
        let mut openupper = self.invk[1].0;

        let mut invklower = self.invk[0].1;
        let mut invkupper = self.invk[1].1;
        
        for (index, invk_value) in self.invk.iter().enumerate() {
            if invk_value.0 < open_percent {
                openlower = self.invk[index].0;
                invklower = self.invk[index].1;
            } else {
                openupper = self.invk[index].0;
                invkupper = self.invk[index].1;
                break;
            }
        }

        if openlower == openupper {
            invklower
        } else {
            let dy = invklower - invkupper;
            let dx = openlower - openupper;
            let m = dy / dx;
            let y = invkupper + m * (open_percent - openupper);
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

    pub fn resistance(&self, q: f64, dh: f64, _nu: f64, g: f64, step: usize ) -> f64 {
        - ( q * q.abs() / ( 2. * self.area()  ) ) + self.invk( step ) * g * self.area() * dh
    }

    pub fn b_coefficient(&self, step: usize ) -> f64 {
        self.invk( step )
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

    pub fn add_transient_value( &mut self, time: f64 ) {
        let steady = self.open_percent[0];
        for event in self.events.iter() {
            self.open_percent.push( event.open_percent( time, steady ) );
        }
        if self.events.len() == 0 {
            self.open_percent.push( *self.open_percent.last().unwrap() );
        }
    }

}

fn default_valve_data() -> Vec<(f64, f64)> {
    vec![ 
        (0.000, 0.0 ),
        (0.111, 1. / 700.),
        (0.222, 1. / 160.),
        (0.333, 1. / 60.),
        (0.444, 1. / 23.),
        (0.556, 1. / 7.9),
        (0.667, 1. / 3.),
        (0.778, 1. / 1.4),
        (0.889, 1. / 0.5),
        (1.000, 1. / 0.25),
    ]
}