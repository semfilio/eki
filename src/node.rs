use crate::nodes::{
    pressure::Pressure,
    flow::Flow,
    connection::Connection,
    hidden::Hidden,
    tank::Tank,
};
use crate::location::Location;
use crate::utility;
use crate::events::TransientEvent;


#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub enum Node {
    Pressure(Pressure),         // Assigned Boundary Pressure
    Flow(Flow),                 // Assigned Boundary Flow
    Connection(Connection),     // Connection Node
    Hidden(Hidden),             // Hidden Node
    Tank(Tank),                 // Tank (surge tank)
}

impl Default for Node {
    fn default() -> Self {
        Node::Pressure( Pressure::default() )
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Pressure(_node) => write!(f, "Pressure"),
            Node::Flow(_node) => write!(f, "Flow"),
            Node::Connection(_node) => write!(f, "Connection"),
            Node::Hidden(_node) => write!(f, "Hidden"),
            Node::Tank(_node) => write!(f, "Tank"),
        }
    }
}

impl Node {

    pub fn id(&self) -> usize {
        match self {
            Node::Pressure(node) => node.id,
            Node::Flow(node) => node.id,
            Node::Connection(node) => node.id,
            Node::Hidden(node) => node.id,
            Node::Tank(node) => node.id,
        }
    }

    pub fn is_connection(&self) -> bool { 
        matches!(self, Node::Connection(_node))
    }

    pub fn is_known_pressure(&self) -> bool {
        matches!(self, Node::Pressure(_node))
    }

    pub fn is_known_flow(&self) -> bool {
        matches!(self, Node::Flow(_node))  
    }

    pub fn is_tank(&self) -> bool {
        matches!(self, Node::Tank(_node))  
    }

    //TODO this should probably return an option
    pub fn area(&self) -> f64 {
        match self {
            Node::Tank(node) => node.area(),
            _ => 0.0,
        }
    }

    pub fn diameter(&mut self) -> Option<&mut f64> {
        match self {
            Node::Tank(node) => Some( &mut node.diameter ),
            _ => None,
        }
    }

    pub fn z_init(&mut self) -> Option<&mut f64> {
        match self {
            Node::Tank(node) => Some( &mut node.z_init ),
            _ => None,
        }
    }

    pub fn z_max(&mut self) -> Option<&mut f64> {
        match self {
            Node::Tank(node) => Some( &mut node.z_max ),
            _ => None,
        }
    }

    pub fn z_min(&mut self) -> Option<&mut f64> {
        match self {
            Node::Tank(node) => Some( &mut node.z_min ),
            _ => None,
        }
    }

    pub fn elevation(&mut self) -> &mut f64 {
        match self {
            Node::Pressure(node) => &mut node.elevation,
            Node::Flow(node) => &mut node.elevation,
            Node::Connection(node) => &mut node.elevation,
            Node::Hidden(node) => &mut node.elevation, //TODO should be None
            Node::Tank(node) => &mut node.elevation,
        }
    }

    pub fn pressure(&mut self) -> &mut Vec<f64> {
        match self {
            Node::Pressure(node) => &mut node.pressure,
            Node::Flow(node) => &mut node.pressure,
            Node::Connection(node) => &mut node.pressure,
            Node::Hidden(node) => &mut node.pressure,
            Node::Tank(node) => &mut node.pressure,
        }
    }

    pub fn steady_pressure(&mut self) -> &mut f64 {
        &mut self.pressure()[0]
    }

    pub fn current_pressure(&mut self) -> f64 {
        *self.pressure().last().unwrap()
    }

    pub fn consumption(&mut self) -> &mut Vec<f64> {
        match self {
            Node::Pressure(node) => &mut node.consumption,
            Node::Flow(node) => &mut node.consumption,
            Node::Connection(node) => &mut node.consumption,
            Node::Hidden(node) => &mut node.consumption,
            Node::Tank(node) => &mut node.consumption,
        }
    }

    pub fn steady_consumption(&mut self) -> &mut f64 {
        &mut self.consumption()[0]
    }

    pub fn head(&mut self, g: f64, density: f64 ) -> Vec<f64> {
        let elevation = *self.elevation();
        let pressure = self.pressure();
        let mut head = vec![0.0; pressure.len()];
        for (i, p) in pressure.iter().enumerate() {
            head[i] = elevation + ((*p) / (g * density)) ;
        }
        head
    }

    pub fn steady_head(&mut self, g: f64, density: f64 ) -> f64 {
        let elevation = *self.elevation();
        let pressure = self.steady_pressure();
        let mut head = elevation;
        head += (*pressure) / (g * density) ;
        head
    }

    pub fn max_pressure(&mut self) -> f64 {
        utility::max_value( self.pressure() )
    }

    pub fn min_pressure(&mut self) -> f64 {
        utility::min_value( self.pressure() )
    }

    pub fn events(&mut self) -> Option<&mut Vec<TransientEvent>> {
        match self {
            Node::Pressure(node) => Some(&mut node.events),
            Node::Flow(node) => Some(&mut node.events),
            _ => None,
        }
    }

    pub fn add_event(&mut self, event: TransientEvent) {
        match self {
            Node::Pressure(node) => node.events.push(event),
            Node::Flow(node) => node.events.push(event),
            _ => (),
        }
    }

    pub fn pop_event(&mut self) -> Option<TransientEvent> {
        match self {
            Node::Pressure(node) => node.events.pop(),
            Node::Flow(node) => node.events.pop(),
            _ => None,
        }
    }

    //TODO do we need this???
    pub fn create_transient_values(&mut self, tnodes: &[f64] ) {
        match self {
            Node::Pressure(node) => node.create_transient_values( tnodes ),
            Node::Flow(node) => node.create_transient_values( tnodes ),
            Node::Connection(node) => node.create_transient_values( tnodes ),
            Node::Hidden(_node) => (),
            Node::Tank(_node) => (),
        }
    }

    pub fn add_transient_value(&mut self, time: f64 ) {
        match self {
            Node::Pressure(node) => node.add_transient_value( time ),
            Node::Flow(node) => node.add_transient_value( time ),
            Node::Connection(_node) => {},
            Node::Hidden(_node) => {},
            Node::Tank(_node) => {},
        }
    }

    pub fn update_id(&mut self, id: usize ) {
        match self {
            Node::Pressure(node) => node.id = id,
            Node::Flow(node) => node.id = id,
            Node::Connection(node) => node.id = id,
            Node::Hidden(node) => node.id = id,
            Node::Tank(node) => node.id = id,
        }
    }

    pub fn add_boundary_value(&mut self, value: f64 ) {
        match self {
            Node::Pressure(node) => node.pressure.push(value),
            Node::Flow(node) => node.consumption.push(value),
            Node::Connection(_node) => {},
            Node::Hidden(_node) => {},
            Node::Tank(_node) => {},
        }
    }

    pub fn selected( &mut self, selected: bool ) {
        match self {
            Node::Pressure(node) => node.selected = selected,
            Node::Flow(node) => node.selected = selected,
            Node::Connection(node) => node.selected = selected,
            Node::Hidden(node) => node.selected = selected,
            Node::Tank(node) => node.selected = selected,
        }
    }

    pub fn is_selected( &self ) -> bool {
        match self {
            Node::Pressure(node) => node.selected,
            Node::Flow(node) => node.selected,
            Node::Connection(node) => node.selected,
            Node::Hidden(node) => node.selected,
            Node::Tank(node) => node.selected,
        }
    }

    pub fn hidden(&self) -> bool {
        matches!(self, Node::Hidden(_node))
    }

    pub fn loc(&self) -> Location {
        match self {
            Node::Pressure(node) => node.loc,
            Node::Flow(node) => node.loc,
            Node::Connection(node) => node.loc,
            Node::Hidden(node) => node.loc,
            Node::Tank(node) => node.loc,
        }
    }

    pub fn r(&self) -> f32 {
        match self {
            Node::Pressure(node) => node.r,
            Node::Flow(node) => node.r,
            Node::Connection(node) => node.r,
            Node::Hidden(node) => node.r,
            Node::Tank(node) => node.r,
        }
    }

    //TODO should this be here or in UI???
    pub fn inside( &self, x: f32, y: f32 ) -> bool {
        let dx = self.loc().x - x;
        let dy = self.loc().y - y;
        (dx * dx + dy * dy).sqrt() <= self.r()
    }

    pub fn update_location(&mut self, x: f32, y: f32 ) {
        match self {
            Node::Pressure(node) => node.loc = Location::new( x, y ),
            Node::Flow(node) => node.loc = Location::new( x, y ),
            Node::Connection(node) => node.loc = Location::new( x, y ),
            Node::Hidden(node) => node.loc = Location::new( x, y ),
            Node::Tank(node) => node.loc = Location::new( x, y ),
        }
    }

    pub fn radius(&mut self, radius: f32 ) {
        match self {
            Node::Pressure(node) => node.r = radius,
            Node::Flow(node) => node.r = radius,
            Node::Connection(node) => node.r = radius,
            Node::Hidden(node) => node.r = radius,
            Node::Tank(node) => node.r = radius,
        }
    }

    pub fn intersection(&self, theta: f32, from: bool, scaling: f32 ) -> (f32, f32) {
        let factor = if from { 1.0 } else { -1.0 };
        let x_inter = self.loc().x - factor * self.r() * theta.cos();
        let y_inter = self.loc().y + factor * self.r() * theta.sin();
        ( x_inter * scaling, y_inter * scaling )
    }
 
}