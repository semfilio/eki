use eki::node::Node;
use eki::nodes::{ pressure::Pressure, flow::Flow, connection::Connection };

#[test]
fn default() {
    let mut node = Node::default();
    assert_eq!( *node.elevation(), 0.0 );
    assert_eq!( *node.pressure(), vec![ 101325.0 ] );
    assert_eq!( *node.consumption(), vec![ 0.0 ] );
    let z = node.elevation();
    *z = 3.14; 
    assert_eq!( *node.elevation(), 3.14 );
    assert_eq!( node.id(), 0 );
}

#[test]
fn head() {
    let mut node = Node::default();
    let g = 9.81;
    let density = 1000.0;
    let head = node.head(g, density)[0];
    assert_eq!( head, 101325.0 / (g * density) );
    let z = node.elevation();
    *z = 3.14; 
    let head = node.head(g, density)[0];
    assert_eq!( head, 3.14 + 101325.0 / (g * density) );
}

#[test]
fn flow() {
    let mut node = Node::Flow( Flow::new( 1 ) );
    assert_eq!( *node.consumption(), vec![ -0.1 ] );
    assert_eq!( node.id(), 1 );
    assert_eq!( node.is_known_flow(), true );
}

#[test]
fn pressure() {
    let mut node = Node::Pressure( Pressure::new( 2 ) );
    assert_eq!( *node.pressure(), vec![ 101325.0 ] );
    assert_eq!( node.id(), 2 );
    assert_eq!( node.is_known_pressure(), true );
}

#[test]
fn connection() {
    let mut node = Node::Connection( Connection::new( 3 ) );
    assert_eq!( *node.elevation(), 0.0 );
    assert_eq!( *node.pressure(), vec![ 101325.0 ] );
    assert_eq!( *node.consumption(), vec![ 0.0 ] );
    assert_eq!( node.id(), 3 );
    assert_eq!( node.is_connection(), true );
}