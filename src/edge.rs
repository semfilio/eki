use crate::node::Node;
use crate::edges::{
    pipe::Pipe,
    valve::Valve,
    pump::Pump,
};
use crate::fluid::Fluid;
use crate::events::TransientEvent;
use crate::utility;

#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub enum Edge {
    Pipe(Pipe),    
    Valve(Valve),
    Pump(Pump),   
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Edge::Pipe(_edge) => write!(f, "Pipe"),
            Edge::Valve(_edge) => write!(f, "Valve"),
            Edge::Pump(_edge) => write!(f, "Pump"),
        }
    }
}

impl Edge {
    pub fn from(&self) -> Node {
        match self {
            Edge::Pipe(edge) => edge.from.clone(),
            Edge::Valve(edge) => edge.from.clone(),
            Edge::Pump(edge) => edge.from.clone(),
        }
    }

    pub fn to(&self) -> Node {
        match self {
            Edge::Pipe(edge) => edge.to.clone(),
            Edge::Valve(edge) => edge.to.clone(),
            Edge::Pump(edge) => edge.to.clone(),
        }
    }

    pub fn id(&self) -> (usize, usize) {
        match self {
            Edge::Pipe(edge) => (edge.from.id(), edge.to.id()),
            Edge::Valve(edge) => (edge.from.id(), edge.to.id()),
            Edge::Pump(edge) => (edge.from.id(), edge.to.id()),
        }
    }

    pub fn mass_flow(&mut self) -> &mut Vec<f64> {
        match self {
            Edge::Pipe(edge) => &mut edge.mass_flow,
            Edge::Valve(edge) => &mut edge.mass_flow,
            Edge::Pump(edge) => &mut edge.mass_flow,
        }
    }

    pub fn steady_mass_flow(&mut self) -> &mut f64 {
        &mut self.mass_flow()[0]
    }

    pub fn current_mass_flow(&mut self) -> f64 {
        *self.mass_flow().last().unwrap()
    }

    pub fn max_mass_flow(&mut self) -> f64 {
        utility::max_value( self.mass_flow() )
    }

    pub fn min_mass_flow(&mut self) -> f64 {
        utility::min_value( self.mass_flow() )
    }

    pub fn length(&mut self) -> Option<&mut f64> {
        match self {
            Edge::Pipe(edge) => Some(&mut edge.length),
            Edge::Valve(_edge) => None,
            Edge::Pump(_edge) => None,
        }
    }

    pub fn diameter(&mut self) -> &mut f64 {
        match self {
            Edge::Pipe(edge) => &mut edge.diameter,
            Edge::Valve(edge) => &mut edge.diameter,
            Edge::Pump(edge) => &mut edge.diameter,     // Impeller diameter
        }
    }

    pub fn area(&self) -> f64 {
        match self {
            Edge::Pipe(edge) => edge.area(),
            Edge::Valve(edge) => edge.area(),
            Edge::Pump(edge) => edge.area(),
        }
    }

    pub fn roughness(&mut self) -> Option<&mut f64> {
        match self {
            Edge::Pipe(edge) => Some(&mut edge.roughness),
            Edge::Valve(_edge) => None,
            Edge::Pump(_edge) => None,
        }
    }

    pub fn thickness(&mut self) -> &mut f64 {
        match self {
            Edge::Pipe(edge) => &mut edge.thickness,
            Edge::Valve(edge) => &mut edge.thickness,
            Edge::Pump(edge) => &mut edge.thickness,
        }
    }

    pub fn youngs_modulus(&mut self) -> &mut f64 {
        match self {
            Edge::Pipe(edge) => &mut edge.youngs_modulus,
            Edge::Valve(edge) => &mut edge.youngs_modulus,
            Edge::Pump(edge) => &mut edge.youngs_modulus,
        }
    }

    // 1.0 = 100% open
    pub fn open_percent(&mut self) -> Option<&mut Vec<f64>> {
        match self {
            Edge::Pipe(_edge) => None,
            Edge::Valve(edge) => Some(&mut edge.open_percent),
            Edge::Pump(_edge) => None,
        }
    }

    pub fn speed(&mut self) -> Option<&mut Vec<f64>> {
        match self {
            Edge::Pipe(_edge) => None,
            Edge::Valve(_edge) => None,
            Edge::Pump(edge) => Some(&mut edge.speed),
        }
    }

    /*pub fn max_speed(&mut self) -> Option<&mut f64> {
        match self {
            Edge::Pipe(_edge) => None,
            Edge::Valve(_edge) => None,
            Edge::Pump(edge) => Some(&mut edge.max_speed),
        }
    }

    pub fn min_speed(&mut self) -> Option<&mut f64> {
        match self {
            Edge::Pipe(_edge) => None,
            Edge::Valve(_edge) => None,
            Edge::Pump(edge) => Some(&mut edge.min_speed),
        }
    }*/

    pub fn steady_open_percent(&mut self) -> &mut f64 {
        &mut self.open_percent().unwrap()[0]
    }

    pub fn k_values(&mut self) -> Option<&mut Vec<(f64, f64)>> {
        match self {
            Edge::Pipe(_edge) => None,
            Edge::Valve(edge) => Some(&mut edge.k),
            Edge::Pump(_edge) => None,
        }
    }

    pub fn pressure_loss_coefficient(&self, step: usize ) -> Option<f64> {
        match self {
            Edge::Pipe(_edge) => None,
            Edge::Valve(edge) => Some( edge.k( step ) ),
            Edge::Pump(_edge) => None,
        }
    }

    pub fn wave_speed(&self, fluid: &Fluid) -> f64 {
        match self {
            Edge::Pipe(edge) => edge.wave_speed( fluid ),
            Edge::Valve(edge) => edge.wave_speed( fluid ),
            Edge::Pump(edge) => edge.wave_speed( fluid ),
        }
    }

    pub fn r_drdq(&self, flow_rate: f64, nu: f64, g: f64, step: usize ) -> (f64, f64) {
        let r = self.resistance( flow_rate, nu, g, step );
        let delta = 1.0e-8;
        let q_plus = flow_rate + delta;
        let r_plus = self.resistance( q_plus, nu, g, step );
        let q_minus = flow_rate - delta;
        let r_minus = self.resistance( q_minus, nu, g, step );
        let drdq = ( r_plus - r_minus ) / ( 2.0 * delta );
        ( r, drdq )
    }

    pub fn resistance(&self, flow_rate: f64, nu: f64, g: f64, step: usize ) -> f64 {
        match self {
            Edge::Pipe(edge) => edge.resistance( flow_rate, nu, g ),
            Edge::Valve(edge) => edge.resistance( flow_rate, nu, g, step ),
            Edge::Pump(edge) => edge.resistance( flow_rate, nu, g, step ),
        }
    }

    pub fn k_laminar(&self, nu: f64 ) -> f64 {
        match self {
            Edge::Pipe(edge) => edge.k_laminar(nu),
            Edge::Valve(edge) => edge.k_laminar(nu),
            Edge::Pump(edge) => edge.k_laminar(nu),
        }
    }

    pub fn darcy_approx(&self, head_loss: f64, g: f64 ) -> f64 {
        match self {
            Edge::Pipe(edge) => edge.darcy_approx(head_loss, g),
            Edge::Valve(edge) => edge.darcy_approx(head_loss, g),
            Edge::Pump(edge) => edge.darcy_approx(head_loss, g),
        }
    }

    //TODO do we need this ???
    /*pub fn create_transient_values(&mut self, tnodes: &[f64] ) {
        match self {
            Edge::Pipe(edge) => edge.create_transient_values( tnodes ),
            Edge::Valve(edge) => edge.create_transient_values( tnodes ),
            Edge::Pump(_edge) => {}, //TODO
        }
    }*/

    pub fn add_transient_value(&mut self, time: f64 ) {
        match self {
            Edge::Pipe(_edge) => {},
            Edge::Valve(edge) => edge.add_transient_value( time ), 
            Edge::Pump(edge) => edge.add_transient_value( time ),
        }
    }

    pub fn events(&mut self) -> Option<&mut Vec<TransientEvent>> {
        match self {
            Edge::Pipe(_edge) => None,
            Edge::Valve(edge) => Some(&mut edge.events),
            Edge::Pump(edge) => Some(&mut edge.events),
        }
    }

    pub fn add_event(&mut self, event: TransientEvent) {
        match self {
            Edge::Pipe(_edge) => {},
            Edge::Valve(edge) => edge.events.push(event),
            Edge::Pump(edge) => edge.events.push(event),
        }
    }

    pub fn pop_event(&mut self) -> Option<TransientEvent> {
        match self {
            Edge::Pipe(_edge) => None,
            Edge::Valve(edge) => edge.events.pop(),
            Edge::Pump(edge) => edge.events.pop(),
        }
    }

    //TODO these can be written better

    pub fn selected( &mut self, select: bool ) {
        match self {
            Edge::Pipe(edge) => edge.selected = select,
            Edge::Valve(edge) => edge.selected = select,
            Edge::Pump(edge) => edge.selected = select,
        }
    }

    pub fn is_selected(&self) -> bool {
        match self {
            Edge::Pipe(edge) => edge.selected,
            Edge::Valve(edge) => edge.selected,
            Edge::Pump(edge) => edge.selected,
        }
    }

    pub fn update_from(&mut self, node: Node ) {
        match self {
            Edge::Pipe(edge) => edge.from = node,
            Edge::Valve(edge) => edge.from = node,
            Edge::Pump(edge) => edge.from = node,
        }
    }

    pub fn update_to(&mut self, node: Node ) {
        match self {
            Edge::Pipe(edge) => edge.to = node,
            Edge::Valve(edge) => edge.to = node,
            Edge::Pump(edge) => edge.to = node,
        }
    }
}
