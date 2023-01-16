use eki::fluid::Fluid;
use eki::fluids::{
    water::Water,
};
use eki::node::Node;
use eki::nodes::{ pressure::Pressure, tank::Tank };
use eki::edge::Edge;
use eki::edges::pipe::Pipe;
use eki::graph::Graph;
use eki::solver::Solver;

// A tank can act as a connection for steady flow.
#[test]
fn pipe_tank_pipe() {
    let mut graph = Graph::new();
    let fluid = Fluid::Water( Water::new( 273.15 + 10.0 ) );    // Water @ 10 degrees C
    let mut solver = Solver::default(); 

    let pressure1 = fluid.density() * solver.gravity() * 20.0; // 20m of head

    let node1 = Node::Pressure( Pressure::new_with_value( 1, pressure1 ) );
    // A tank between the two pipes
    let node2 = Node::Tank( Tank::new( 2, 101325.0, fluid.density(), solver.gravity() ) );
    let node3 = Node::Pressure( Pressure::new( 3 ) );

    graph.add_node( node1.clone() );
    graph.add_node( node2.clone() );
    graph.add_node( node3.clone() );

    let (t, y) = (5.0e-3, 2.0e11);
    let (l, d, r) = (100.0, 50.0e-3, 0.0);
    let pipe1 = Edge::Pipe( Pipe::new_params( node1.clone(), node2.clone(), l, d, r, t, y ) );
    let (l, d, r) = (200.0, 50.0e-3, 0.0);
    let pipe2 = Edge::Pipe( Pipe::new_params( node2.clone(), node3.clone(), l, d, r, t, y ) );

    graph.add_edge( pipe1 );
    graph.add_edge( pipe2 );

    let result = solver.solve_steady( &mut graph, &fluid, true );
    assert!( result.is_ok() && !result.is_err() );

    let h0 = graph.nodes()[0].steady_head( solver.gravity(), fluid.density() );
    assert!( (h0 - 20.0).abs() < 1.0e-8 );
    let h1 = graph.nodes()[1].steady_head( solver.gravity(), fluid.density() );
    assert!( (h1 - 16.77845838).abs() < 1.0e-8 );
    let h2 = graph.nodes()[2].steady_head( solver.gravity(), fluid.density() );
    assert!( (h2 - 10.33537514).abs() < 1.0e-8 );

    let q = *graph.edges()[0].steady_mass_flow() / fluid.density();
    assert!( (q - 0.00239598).abs() < 1.0e-8 );
}