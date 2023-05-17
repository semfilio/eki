use eki::fluid::Fluid;
use eki::node::Node;
use eki::nodes::{ pressure::Pressure };
use eki::edge::Edge;
use eki::edges::{ open_pipe::OpenPipe };
use eki::graph::Graph;
use eki::solver::Solver;

#[test]
fn steady_open_pipe() {
    let mut graph = Graph::new();
    let fluid = Fluid::new_basic( 997.0, 1.1375e-6, 2.15e9 );
    let mut solver = Solver::default();
    let rho_g = fluid.density() * solver.gravity();
    
    let p_from = rho_g * 20.0; // H_0 = 20m
    let node_from = Node::Pressure( Pressure::new_with_value( 0, p_from ) );
    graph.add_node( node_from.clone() );
    let node_to = Node::Pressure( Pressure::new( 1 ) );
    graph.add_node( node_to.clone() );
    let mut open_pipe = Edge::OpenPipe( OpenPipe::new( node_from, node_to ) );
    *open_pipe.k().unwrap() = 1.5;                   // K = 1.5
    *open_pipe.diameter().unwrap() = 50.0e-3;        // D = 50mm
    graph.add_edge( open_pipe );
    
    
    let result = solver.solve_steady( &mut graph, &fluid, true );
    assert!( result.is_ok() && !result.is_err() );
    
    let h_from = graph.nodes()[0].steady_head( solver.gravity(), fluid.density() );
    assert_eq!( h_from, 20.0 );
    let h_to = graph.nodes()[1].steady_head( solver.gravity(), fluid.density() );
    assert_eq!( h_to, 101325.0 / rho_g );
    
    let mass_flow = (*graph.edges()[0].mass_flow())[0];
    let q = mass_flow / fluid.density();
    assert!( (q - 0.022041 ).abs() < 1.0e-6 );
}