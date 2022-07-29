use eki::fluid::Fluid;
use eki::fluids::{
    water::Water,
};
use eki::node::Node;
use eki::nodes::{ pressure::Pressure, connection::Connection, flow::Flow };
use eki::edge::Edge;
use eki::edges::pipe::Pipe;
use eki::graph::Graph;
use eki::solver::Solver;
use eki::utility;

#[test]
fn solve() {
    let fluid = Fluid::Water( Water::new( 273.15 + 10.0 ) );    // Water @ 10 degrees C
    //let fluid = Fluid::new_basic( 999.7, 1.3063e-6, 2.15e9 ); // Water @ 10 degrees C
    let mut graph = Graph::new();

    let node1 = Node::Pressure( Pressure::new_elevation( 1, 85.0 ) );
    let node2 = Node::Pressure( Pressure::new_elevation( 2, 100.0 ) );
    let node3 = Node::Pressure( Pressure::new_elevation( 3, 60.0 ) );
    let node4 = Node::Connection( Connection::new_elevation( 4, 0.0 ) );
    let node5 = Node::Flow( Flow::new_with_value( 5, -0.06 * fluid.density() ) );

    graph.add_node( node1.clone() );
    graph.add_node( node2.clone() );
    graph.add_node( node3.clone() );
    graph.add_node( node4.clone() );
    graph.add_node( node5.clone() );

    let (t, y) = (5.0e-3, 2.0e11);
    let (l, d, r) = (1500.0, 250.0e-3, 0.5e-3);
    let pipe1 = Edge::Pipe( Pipe::new_params( node1.clone(), node4.clone(), l, d, r, t, y ) );
    let (l, d, r) = (2000.0, 300.0e-3, 0.5e-3);
    let pipe2 = Edge::Pipe( Pipe::new_params( node2.clone(), node4.clone(), l, d, r, t, y ) );
    let (l, d, r) = (3000.0, 250.0e-3, 0.5e-3);
    let pipe3 = Edge::Pipe( Pipe::new_params( node3.clone(), node4.clone(), l, d, r, t, y ) );
    let pipe4 = Edge::Pipe( Pipe::new( node4.clone(), node5.clone() ) );

    graph.add_edge( pipe1 );
    graph.add_edge( pipe2 );
    graph.add_edge( pipe3 );
    graph.add_edge( pipe4 );
 
    let mut solver = Solver::default(); 
    let result = solver.solve_steady( &mut graph, &fluid, true );
    assert!( result.is_ok() && !result.is_err() );

    // Flow rate from highest reservoir
    let volume_flow = *graph.edges()[1].steady_mass_flow() / fluid.density();
    assert!( utility::relative_error( 0.1023, volume_flow ) < 0.005 ); // < 0.5% error (published)
    assert!( ( volume_flow - 0.1022 ).abs() < 1.0e-4 ); // FD

    // Flow rate from middle reservoir
    let volume_flow = *graph.edges()[0].steady_mass_flow() / fluid.density();
    assert!( utility::relative_error( 0.02, volume_flow ) < 0.005 ); // < 0.5% error (published)
    assert!( ( volume_flow - 0.02 ).abs() < 1.0e-2 ); // FD

    // Flow rate from lowest reservoir
    let volume_flow = *graph.edges()[2].steady_mass_flow() / fluid.density();
    assert!( utility::relative_error( -0.0622, volume_flow ) < 0.005 ); // < 0.5% error (published)
    assert!( ( volume_flow + 0.0621 ).abs() < 1.0e-4 ); // FD

    // External flow demand
    let volume_flow = *graph.edges()[3].steady_mass_flow() / fluid.density();
    assert_eq!( volume_flow, 0.06 );
}
