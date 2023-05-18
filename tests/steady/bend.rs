use eki::node::Node;
use eki::nodes::{ pressure::Pressure, connection::Connection };
use eki::edge::Edge;
use eki::edges::{ bend::Bend, pipe::Pipe };
use eki::graph::Graph;
use eki::fluid::Fluid;
use eki::solver::Solver;
use eki::utility;

#[test]
fn create_bend() {
    let mut graph = Graph::new();
    let node_from = Node::Pressure( Pressure::new_with_value( 0, 121325.0 ) );
    graph.add_node( node_from.clone() );
    let node_to = Node::Pressure( Pressure::new( 1 ) );
    graph.add_node( node_to.clone() );
    let radius = 52.5e-3;                // 52.5mm bend radius
    let diameter = 52.5e-3;              // 52.5mm 
    let angle = (90.0_f64).to_radians(); // 90 degree bend angle
    let roughness = 0.05e-3;             // 0.05mm
    let thickness = 5.0e-3;              // 5mm pipe
    let youngs_modulus = 2.0e11;         // Steel pipe
    let bend = Bend::new_params( 
        node_from, node_to, radius, diameter, angle, roughness, thickness, youngs_modulus 
    );
    assert_eq!( bend.radius, radius );
    assert_eq!( bend.length(), radius * angle );
    let edge = Edge::Bend( bend );
    graph.add_edge( edge );
}

#[test]
fn pipe_bend_pipe() {
    let mut graph = Graph::new();
    let node_from = Node::Pressure( Pressure::new_with_value( 1, 121325.0 ) );
    graph.add_node( node_from.clone() );
    let node_c1 = Node::Connection( Connection::new( 2 ) );
    graph.add_node( node_c1.clone() );
    let node_c2 = Node::Connection( Connection::new( 3 ) );
    graph.add_node( node_c2.clone() );
    let node_to = Node::Pressure( Pressure::new( 4 ) );
    graph.add_node( node_to.clone() );

    let edge = Edge::Pipe( Pipe::new( node_from, node_c1.clone() ) );
    graph.add_edge( edge );

    let edge = Edge::Bend( Bend::new( node_c1, node_c2.clone() ) );
    graph.add_edge( edge );

    let edge = Edge::Pipe( Pipe::new( node_c2, node_to ) );
    graph.add_edge( edge );

    let fluid = Fluid::new_basic( 999.7, 1.3063e-6, 2.15e9 ); // Water @ 10 degrees C
    let mut solver = Solver::default();
    let result = solver.solve_steady( &mut graph, &fluid, true );
    assert!( result.is_ok() && !result.is_err() );

    let mass_flow = *graph.edges()[1].steady_mass_flow();
    //assert_eq!( mass_flow, 4.59 );
    assert!( utility::relative_error( 4.576737, mass_flow ) < 0.01 ); // < 1% FD
}


/* TRIAL TESTS - SEM */

#[test] //Trial-Sem
fn bend_resistance() {
    let mut graph = Graph::new();
    let node_from = Node::Pressure( Pressure::new_with_value( 0, 121325.0 ) );
    graph.add_node( node_from.clone() );
    let node_to = Node::Pressure( Pressure::new( 1 ) );
    graph.add_node( node_to.clone() );
    let radius = 52.5e-3;                // 52.5mm bend radius
    let diameter = 52.5e-3;              // 52.5mm 
    let angle = (90.0_f64).to_radians(); // 90 degree bend angle
    let roughness = 0.05e-3;             // 0.05mm
    let thickness = 5.0e-3;              // 5mm pipe
    let youngs_modulus = 2.0e11;         // Steel pipe
    let bend = Bend::new_params( 
        node_from, node_to, radius, diameter, angle, roughness, thickness, youngs_modulus 
    );
    assert_eq!( bend.radius, radius );
    assert_eq!( bend.length(), radius * angle );
    let edge = Edge::Bend( bend.clone() );
    graph.add_edge( edge );
    

    // zero flow resistance
    let q = 0.0;
    let dh = 0.5;
    let nu = 0.001/1000.;
    let g = 9.81;
    assert_eq!(bend.resistance(q, dh, nu, g), 0.0);
    

    // with flow resistance
    let q = 0.1;
    let f = bend.friction_factor(q, nu);
    let rd = radius / diameter;
    let s = ( 0.5 * bend.angle ).sin();
    let pow = rd.powf( 4. * bend.angle / 3.1415 );    
    assert_eq!(bend.k(q, nu), f * bend.angle * rd + ( 0.1 + 2.4 * f ) * s + ( 6.6 * f * ( s.sqrt() + s ) / pow ));

    let k = f * bend.angle * rd + ( 0.1 + 2.4 * f ) * s + ( 6.6 * f * ( s.sqrt() + s ) / pow );
    assert_eq!(bend.resistance(q, dh, nu, g), - ( k * q * q.abs() / ( 2. * bend.area() ) ) + g * bend.area() * dh);


}
