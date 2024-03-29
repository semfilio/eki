use std::f64::consts::PI;

use eki::node::Node;
use eki::nodes::{ 
    pressure::Pressure, 
    flow::Flow, 
    connection::Connection,
    tank::Tank, 
};

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

#[test]
fn tank() {
    let mut node = Node::Tank( Tank::new( 4, 101325.0, 1000.0, 9.80665 ) );
    assert_eq!( *node.elevation(), 0.0 );
    assert_eq!( *node.pressure(), vec![ 106228.325 ] );
    assert_eq!( *node.consumption(), vec![ 0.0 ] );
    assert_eq!( node.id(), 4 );
    assert_eq!( node.is_tank(), true );
}

/* TRIAL TESTS - SEM */

#[test] //Trial-Sem
fn flow_with_values() {
    let mut node = Node::Flow( Flow::new_with_value( 0, -2.0 ) );
    assert_eq!( *node.consumption(), vec![ -2.0 ] );
    assert_eq!( node.id(), 0 );
    assert_eq!( node.is_known_flow(), true );
    assert_eq!( node.is_tank(), false );
    assert_eq!( node.area(), 0.0 );
    assert_eq!( node.diameter(), None );
    assert_eq!( *node.pressure(), vec![ 101325.0 ] );
}

#[test] //Trial-Sem
fn tank_with_values() {
    let mut node = Node::Tank( Tank::new_with_values( 4, 101325.0, 1200.0, 9.80665, 2.0, 1.0, 0.3, 2.5 ) );
    assert_eq!( node.area(), PI / 4.0 * 2.0 * 2.0 );
    assert_eq!( *node.pressure(), vec![ 101325.0 + 1200.0 * 9.80665 * 1.0 ] );
    assert_eq!( *node.consumption(), vec![ 0.0 ] );
    assert_eq!( node.id(), 4 );
    assert_eq!( node.is_tank(), true );
}
