use eki::fluid::Fluid;
use eki::node::Node;
use eki::nodes::{ pressure::Pressure };
use eki::edge::Edge;
use eki::edges::{ check_valve::CheckValve };
use eki::graph::Graph;
use eki::solver::Solver;

#[test]
fn steady_check_valve() {
    let mut graph = Graph::new();
    let fluid = Fluid::new_basic( 997.0, 1.1375e-6, 2.15e9 );
    let mut solver = Solver::default();
    let rho_g = fluid.density() * solver.gravity();
    
    let p_from = rho_g * 20.0; // H_0 = 20m
    let node_from = Node::Pressure( Pressure::new_with_value( 0, p_from ) );
    graph.add_node( node_from.clone() );
    let node_to = Node::Pressure( Pressure::new( 1 ) );
    graph.add_node( node_to.clone() );
    let mut check_valve = Edge::CheckValve( CheckValve::new( node_from, node_to ) );
    *check_valve.invk_values().unwrap() = vec![ 
        (0.0, 0.0),
        (1.0, 1. / 0.25),
    ];
    *check_valve.diameter().unwrap() = 50.0e-3;        // D = 50mm
    assert_eq!( *check_valve.steady_open_percent(), 1.0); // Should be open initially.
    graph.add_edge( check_valve );
    
    
    let result = solver.solve_steady( &mut graph, &fluid, true );
    assert!( result.is_ok() && !result.is_err() );

    // Flow should be positive because the check valve is open initially.
    let mass_flow = (*graph.edges()[0].mass_flow())[0];
    let q = mass_flow / fluid.density();
    assert!( q > 0.0 );
}