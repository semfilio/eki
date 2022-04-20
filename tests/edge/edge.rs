use eki::node::Node;
use eki::nodes::{ pressure::Pressure, flow::Flow, connection::Connection };
use eki::edge::Edge;
use eki::edges::{ pipe::Pipe, valve::Valve };

#[test]
fn pipe() {
    let node_from = Node::Pressure( Pressure::new( 0 ) );
    let node_to = Node::Flow( Flow::new( 1 ) );
    let mut edge = Edge::Pipe( Pipe::new( node_from, node_to ) );
    *edge.steady_mass_flow() = 1.0;
    assert_eq!( *edge.mass_flow(), vec![ 1.0 ] );
    assert_eq!( edge.id(), (0,1) );
    assert_eq!( *edge.length().unwrap(), 10.0 );
    assert_eq!( *edge.diameter(), 52.5e-3 );
    assert_eq!( *edge.roughness().unwrap(), 0.05e-3 );
    assert_eq!( *edge.thickness(), 0.005 );
    assert_eq!( *edge.youngs_modulus(), 2.0e11 );
}

#[test]
fn valve() {
    let node_from = Node::Pressure( Pressure::new( 1 ) );
    let node_to = Node::Connection( Connection::new( 2 ) );
    let mut edge = Edge::Valve( Valve::new( node_from, node_to ) );
    assert_eq!( edge.id(), (1,2) );
    assert_eq!( *edge.diameter(), 52.5e-3 );
    assert_eq!( *edge.thickness(), 0.005 );
    assert_eq!( *edge.youngs_modulus(), 2.0e11 );
    assert_eq!( *edge.open_percent().unwrap(), vec![ 1.0 ] );
    assert_eq!( edge.pressure_loss_coefficient().unwrap(), 0.25 );
}

#[test]
fn resistance() {
    let node_from = Node::Pressure( Pressure::new( 1 ) );
    let node_to = Node::Connection( Connection::new( 2 ) );
    let edge = Edge::Pipe( Pipe::new( node_from, node_to ) );
    let q = 0.01;
    let r = edge.resistance( q, 1.1375e-6, 9.81 );
    assert_eq!( r, -4.299969928559725 );
}