//! # Eki
//!
//! `eki` is a solver for steady and transient flow in fluid networks.

pub mod fluid;
pub mod node;
pub mod nodes;
pub mod edge;
pub mod edges;
pub mod graph;
pub mod solver;
pub mod utility;

//Re-exports
pub use self::fluid::Fluid;
pub use self::node::Node;
pub use self::nodes::{
    pressure::Pressure, flow::Flow, connection::Connection
};

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

}
