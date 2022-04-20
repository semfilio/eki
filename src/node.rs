use crate::nodes::{
    pressure::Pressure,
    flow::Flow,
    connection::Connection,
};

#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub enum Node {
    Pressure(Pressure),         // Assigned Boundary Pressure
    Flow(Flow),                 // Assigned Boundary Flow
    Connection(Connection),     // Connection Node
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
        }
    }
}

impl Node {

    pub fn id(&self) -> usize {
        match self {
            Node::Pressure(node) => node.id,
            Node::Flow(node) => node.id,
            Node::Connection(node) => node.id,
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

    pub fn elevation(&mut self) -> &mut f64 {
        match self {
            Node::Pressure(node) => &mut node.elevation,
            Node::Flow(node) => &mut node.elevation,
            Node::Connection(node) => &mut node.elevation,
        }
    }

    pub fn pressure(&mut self) -> &mut Vec<f64> {
        match self {
            Node::Pressure(node) => &mut node.pressure,
            Node::Flow(node) => &mut node.pressure,
            Node::Connection(node) => &mut node.pressure,
        }
    }

    pub fn consumption(&mut self) -> &mut Vec<f64> {
        match self {
            Node::Pressure(node) => &mut node.consumption,
            Node::Flow(node) => &mut node.consumption,
            Node::Connection(node) => &mut node.consumption,
        }
    }

    pub fn head(&mut self, g: f64, density: f64 ) -> Vec<f64> {
        let elevation = *self.elevation();
        let pressure = self.pressure();
        //let mut head = elevation;
        //head += (*pressure) / (g * density) ;
        let mut head = vec![0.0; pressure.len()];
        for (i, p) in pressure.iter().enumerate() {
            head[i] = elevation + ((*p) / (g * density)) ;
        }
        head
    }

    pub fn update_id(&mut self, id: usize ) {
        match self {
            Node::Pressure(node) => node.id = id,
            Node::Flow(node) => node.id = id,
            Node::Connection(node) => node.id = id,
        }
    }
 
}