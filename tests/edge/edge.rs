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
    assert_eq!( *edge.diameter().unwrap(), 52.5e-3 );
    assert_eq!( *edge.roughness().unwrap(), 0.05e-3 );
    assert_eq!( *edge.thickness().unwrap(), 0.005 );
    assert_eq!( *edge.youngs_modulus().unwrap(), 2.0e11 );
    assert_eq!( edge.from().id(), 0 );
    assert_eq!( edge.to().id(), 1 );
}

#[test]
fn valve() {
    let node_from = Node::Pressure( Pressure::new( 1 ) );
    let node_to = Node::Connection( Connection::new( 2 ) );
    let mut edge = Edge::Valve( Valve::new( node_from, node_to ) );
    assert_eq!( edge.id(), (1,2) );
    assert_eq!( *edge.diameter().unwrap(), 52.5e-3 );
    assert_eq!( *edge.thickness().unwrap(), 0.005 );
    assert_eq!( *edge.youngs_modulus().unwrap(), 2.0e11 );
    assert_eq!( *edge.open_percent().unwrap(), vec![ 1.0 ] );
    assert_eq!( edge.pressure_loss_coefficient( 0 ).unwrap(), 0.25 );
}

#[test]
fn resistance() {
    let node_from = Node::Pressure( Pressure::new( 1 ) );
    let node_to = Node::Connection( Connection::new( 2 ) );
    let mut edge = Edge::Pipe( Pipe::new( node_from, node_to ) );
    let q = 0.01;
    let dh = 0.0;
    let r = edge.resistance( q, dh, 1.1375e-6, 9.81, 0 );
    let lga = *(edge.length().unwrap()) / ( 9.81 * edge.area() );
    assert_eq!( r * lga, -4.299969928559724 );
}



/* TRIAL TESTS - SEM */

#[test] //Trial-Sem
fn pipe_2() {
    let node_from = Node::Pressure( Pressure::new( 3 ) );
    let node_to = Node::Flow( Flow::new( 5 ) );
    let mut edge = Edge::Pipe( Pipe::new_params( node_from, node_to, 25.0, 103.5e-3, 0.01e-3, 0.01, 2.0e11 ) );
    *edge.steady_mass_flow() = 5.0;
    assert_eq!( *edge.mass_flow(), vec![ 5.0 ] );
    assert_eq!( edge.id(), (3,5) );
    assert_eq!( *edge.length().unwrap(), 25.0 );
    assert_eq!( *edge.diameter().unwrap(), 103.5e-3 );
    assert_eq!( *edge.roughness().unwrap(), 0.01e-3 );
    assert_eq!( *edge.thickness().unwrap(), 0.01 );
    assert_eq!( *edge.youngs_modulus().unwrap(), 2.0e11 );
    assert_eq!( edge.from().id(), 3 );
    assert_eq!( edge.to().id(), 5 );
}

#[test] //Trial-Sem
fn valve_2() {
    let node_from = Node::Pressure( Pressure::new_with_value( 1, 25.0 ) );
    let node_to = Node::Pressure( Pressure::new( 2 ) );
    let mut edge = Edge::Valve( Valve::new( node_from, node_to ) );
    assert_eq!( edge.id(), (1,2) );
    assert_eq!( *edge.diameter().unwrap(), 52.5e-3 );
    assert_eq!( *edge.thickness().unwrap(), 0.005 );
    assert_eq!( *edge.youngs_modulus().unwrap(), 2.0e11 );
    assert_eq!( *edge.open_percent().unwrap(), vec![ 1.0 ] );
    assert_eq!( edge.pressure_loss_coefficient( 0 ).unwrap(), 0.25 );
}


#[test] //Trial-Sem
fn resistance_pipe_zero_flow() {
    let node_from = Node::Flow( Flow::new( 9 ) );
    let node_to = Node::Connection( Connection::new( 10 ) );
    let mut edge = Edge::Pipe( Pipe::new_params( node_from, node_to, 25.0, 103.5e-3, 0.01e-3, 0.01, 2.0e11 ) );
    let q = 0.0;
    let dh = 0.0;
    let r = edge.resistance( q, dh, 1.1375e-6, 9.81, 0 );
    let lga = *(edge.length().unwrap()) / ( 9.81 * edge.area() );
    assert_eq!( r * lga, 0.0);
    assert_eq!( edge.id(), (9,10) );
}
