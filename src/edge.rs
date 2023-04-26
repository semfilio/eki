use crate::node::Node;
use crate::edges::{
    pipe::Pipe,
    valve::Valve,
    pump::Pump,
    bend::Bend,
    size_change::SizeChange,
};
use crate::fluid::Fluid;
use crate::events::TransientEvent;
use crate::utility;

#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub enum Edge {
    Pipe(Pipe),    
    Valve(Valve),
    Pump(Pump), 
    Bend(Bend),
    SizeChange(SizeChange),  
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Edge::Pipe(_edge) => write!(f, "Pipe"),
            Edge::Valve(_edge) => write!(f, "Valve"),
            Edge::Pump(_edge) => write!(f, "Pump"),
            Edge::Bend(_edge) => write!(f, "Bend"),
            Edge::SizeChange(_edge) => write!(f, "Size Change"),
        }
    }
}

// Macro for match statement when all cases are the same
macro_rules! match_edge {
    ($self:ident, $edge:ident, $block:block) => {
        match $self {
            Edge::Pipe($edge) => $block,
            Edge::Valve($edge) => $block,
            Edge::Pump($edge) => $block,
            Edge::Bend($edge) => $block,
            Edge::SizeChange($edge) => $block,
        }
    };
}

impl Edge {
    pub fn from(&self) -> Node {
        match_edge!(self, edge, {edge.from.clone()})
    }

    pub fn to(&self) -> Node {
        match_edge!(self, edge, {edge.to.clone()})
    }

    pub fn id(&self) -> (usize, usize) {
        match_edge!(self, edge, {(edge.from.id(), edge.to.id())})
    }

    pub fn mass_flow(&mut self) -> &mut Vec<f64> {
        match_edge!(self, edge, {&mut edge.mass_flow})
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
            Edge::Bend(_edge) => None,
            Edge::SizeChange(_edge) => None,
        }
    }

    pub fn radius(&mut self) -> Option<&mut f64> {
        match self {
            Edge::Pipe(_edge) => None,
            Edge::Valve(_edge) => None,
            Edge::Pump(_edge) => None,
            Edge::Bend(edge) => Some(&mut edge.radius),
            Edge::SizeChange(_edge) => None,
        }
    }

    pub fn angle(&mut self) -> Option<&mut f64> {
        match self {
            Edge::Pipe(_edge) => None,
            Edge::Valve(_edge) => None,
            Edge::Pump(_edge) => None,
            Edge::Bend(edge) => Some(&mut edge.angle),
            Edge::SizeChange(_edge) => None,
        }
    }

    pub fn diameter(&mut self) -> &mut f64 {
        match_edge!(self, edge, {&mut edge.diameter})
    }

    pub fn area(&self) -> f64 {
        match_edge!(self, edge, {edge.area()})
    }

    pub fn roughness(&mut self) -> Option<&mut f64> {
        match self {
            Edge::Pipe(edge) => Some(&mut edge.roughness),
            Edge::Valve(_edge) => None,
            Edge::Pump(_edge) => None,
            Edge::Bend(edge) => Some(&mut edge.roughness),
            Edge::SizeChange(_edge) => None,
        }
    }

    pub fn thickness(&mut self) -> Option<&mut f64> {
        match self {
            Edge::Pipe(edge) => Some( &mut edge.thickness ),
            Edge::Valve(edge) => Some( &mut edge.thickness ),   //TODO maybe we don't need this? just use fluid wave speed?
            Edge::Pump(edge) => Some( &mut edge.thickness ),    //TODO maybe we don't need this? just use fluid wave speed?
            Edge::Bend(edge) => Some( &mut edge.thickness ),
            Edge::SizeChange(_edge) => None,
        }
    }

    pub fn youngs_modulus(&mut self) -> Option<&mut f64> {
        match self {
            Edge::Pipe(edge) => Some( &mut edge.youngs_modulus ),
            Edge::Valve(edge) => Some( &mut edge.youngs_modulus ),  //TODO maybe we don't need this? just use fluid wave speed?
            Edge::Pump(edge) => Some( &mut edge.youngs_modulus ),   //TODO maybe we don't need this? just use fluid wave speed?
            Edge::Bend(edge) => Some( &mut edge.youngs_modulus ),
            Edge::SizeChange(_edge) => None,
        }
    }

    // 1.0 = 100% open
    pub fn open_percent(&mut self) -> Option<&mut Vec<f64>> {
        match self {
            Edge::Pipe(_edge) => None,
            Edge::Valve(edge) => Some(&mut edge.open_percent),
            Edge::Pump(_edge) => None,
            Edge::Bend(_edge) => None,
            Edge::SizeChange(_edge) => None,

        }
    }

    pub fn speed(&mut self) -> Option<&mut Vec<f64>> {
        match self {
            Edge::Pipe(_edge) => None,
            Edge::Valve(_edge) => None,
            Edge::Pump(edge) => Some(&mut edge.speed),
            Edge::Bend(_edge) => None,
            Edge::SizeChange(_edge) => None,
        }
    }

    pub fn size_ratio(&mut self) -> Option<&mut f64> {
        match self {
            Edge::Pipe(_edge) => None,
            Edge::Valve(_edge) => None,
            Edge::Pump(_edge) => None,
            Edge::Bend(_edge) => None,
            Edge::SizeChange(edge) => Some(&mut edge.beta),
        }
    }

    pub fn steady_open_percent(&mut self) -> &mut f64 {
        &mut self.open_percent().unwrap()[0]
    }

    pub fn k_values(&mut self) -> Option<&mut Vec<(f64, f64)>> {
        match self {
            Edge::Pipe(_edge) => None,
            Edge::Valve(edge) => Some(&mut edge.k),
            Edge::Pump(_edge) => None,
            Edge::Bend(_edge) => None,
            Edge::SizeChange(_edge) => None,
        }
    }

    pub fn pressure_loss_coefficient(&self, step: usize ) -> Option<f64> {
        match self {
            Edge::Pipe(_edge) => None,
            Edge::Valve(edge) => Some( edge.k( step ) ),
            Edge::Pump(_edge) => None,
            Edge::Bend(_edge) => None, //TODO: Bend pressure loss coefficient
            Edge::SizeChange(_edge) => None, //TODO: Size change pressure loss coefficient
        }
    }

    pub fn wave_speed(&self, fluid: &Fluid) -> f64 {
        match_edge!(self, edge, {edge.wave_speed( fluid )})
    } 

    // The coefficient for the M matrix (typically 0 as we assume infinte wave speed in non-pipes)
    pub fn m_coefficient(&self, fluid: &Fluid, g: f64) -> f64 {
        match self {
            Edge::Pipe(edge) => edge.m_coefficient( fluid, g ),
            Edge::Valve(_edge) => 0.0,
            Edge::Pump(_edge) => 0.0,
            Edge::Bend(_edge) => 0.0, //TODO bend m coefficient (length = curve length) ?
            Edge::SizeChange(_edge) => 0.0,
        }
    }

    // The coefficient for the B matrix (typically 1)
    pub fn b_coefficient(&self, _fluid: &Fluid, _g: f64 ) -> f64 {
        match self {
            Edge::Valve(_edge) => 1.0, //TODO valve b coefficient = k^-1
            _ => 1.0,
        }
    }

    pub fn drdq(&self, q: f64, dh: f64, nu: f64, g: f64, step: usize ) -> f64 {
        let delta = 1.0e-8;
        let r_plus = self.resistance( q + delta, dh, nu, g, step );
        let r_minus = self.resistance( q - delta, dh, nu, g, step );
        ( r_plus - r_minus ) / ( 2.0 * delta )
    }

    pub fn drdkh(&self, q:f64, dh: f64, nu: f64, g: f64, step: usize ) -> f64 {
        let delta = 1.0e-8;
        let r_plus = self.resistance( q, dh + delta, nu, g, step );
        let r_minus = self.resistance( q, dh - delta, nu, g, step );
        ( r_plus - r_minus ) / ( 2.0 * delta )
    }

    pub fn resistance(&self, q: f64, dh: f64, nu: f64, g: f64, step: usize ) -> f64 {
        match self {
            Edge::Pipe(edge) => edge.resistance( q, dh, nu, g ),
            Edge::Valve(edge) => edge.resistance( q, dh, nu, g, step ),
            Edge::Pump(edge) => edge.resistance( q, dh, nu, g, step ),
            Edge::Bend(edge) => edge.resistance( q, dh, nu, g ),
            Edge::SizeChange(edge) => edge.resistance( q, dh, nu, g ),
        }
    }

    pub fn k_laminar(&self, nu: f64 ) -> f64 {
        match_edge!(self, edge, {edge.k_laminar(nu)})
    }

    pub fn darcy_approx(&self, head_loss: f64, g: f64 ) -> f64 {
        match_edge!(self, edge, {edge.darcy_approx(head_loss, g)})
    }

    pub fn add_transient_value(&mut self, time: f64 ) {
        match self {
            Edge::Pipe(_edge) => {},
            Edge::Valve(edge) => edge.add_transient_value( time ), 
            Edge::Pump(edge) => edge.add_transient_value( time ),
            Edge::Bend(_edge) => {},
            Edge::SizeChange(_edge) => {},
        }
    }

    pub fn events(&mut self) -> Option<&mut Vec<TransientEvent>> {
        match self {
            Edge::Pipe(_edge) => None,
            Edge::Valve(edge) => Some(&mut edge.events),
            Edge::Pump(edge) => Some(&mut edge.events),
            Edge::Bend(_edge) => None,
            Edge::SizeChange(_edge) => None,
        }
    }

    pub fn add_event(&mut self, event: TransientEvent) {
        match self {
            Edge::Pipe(_edge) => {},
            Edge::Valve(edge) => edge.events.push(event),
            Edge::Pump(edge) => edge.events.push(event),
            Edge::Bend(_edge) => {},
            Edge::SizeChange(_edge) => {},
        }
    }

    pub fn pop_event(&mut self) -> Option<TransientEvent> {
        match self {
            Edge::Pipe(_edge) => None,
            Edge::Valve(edge) => edge.events.pop(),
            Edge::Pump(edge) => edge.events.pop(),
            Edge::Bend(_edge) => None,
            Edge::SizeChange(_edge) => None,
        }
    }

    pub fn selected( &mut self, select: bool ) {
        match_edge!(self, edge, {edge.selected = select})
    }

    pub fn is_selected(&self) -> bool {
        match_edge!(self, edge, {edge.selected})
    }

    pub fn update_from(&mut self, node: Node ) {
        match_edge!(self, edge, {edge.from = node})
    }

    pub fn update_to(&mut self, node: Node ) {
        match_edge!(self, edge, {edge.to = node})
    }
}
