// Tests for the generic component 
use eki::fluid::Fluid;
use eki::node::Node;
use eki::nodes::{ pressure::Pressure };
use eki::edge::Edge;
use eki::edges::generic::Generic;
use eki::graph::Graph;
use eki::solver::Solver;

#[test]
fn generic_component_test() {
    let mut graph = Graph::new();
    let fluid = Fluid::new_basic( 997.0, 1.1375e-6, 2.15e9 );
    let mut solver = Solver::default();
    let rho_g = fluid.density() * solver.gravity();

    // If Q = 1.5, (A, B, C) = (1.0, 2.0, 3.0) and (m, n) = (1.5, 2.7) then dH = 13.63959298
    
    let p_from = rho_g * 13.63959298 + 101325.0 ; 
    let node_from = Node::Pressure( Pressure::new_with_value( 0, p_from ) );
    graph.add_node( node_from.clone() );
    let node_to = Node::Pressure( Pressure::new( 1 ) );
    graph.add_node( node_to.clone() );

    let coefficients = ( 1.0, 2.0, 3.0 );
    let exponents = ( 1.5, 2.7 );
    let generic = Edge::Generic( Generic::new_params( 
        node_from, 
        node_to,
        coefficients,
        exponents,
    ) );
    graph.add_edge( generic );

    let result = solver.solve_steady( &mut graph, &fluid, true );
    assert!( result.is_ok() && !result.is_err() );
    if let Ok(iter) = result {
        assert_eq!( iter, 9 ); // Takes many iterations to converge (need a better laminar guess)
    }
    
    let mass_flow = (*graph.edges()[0].mass_flow())[0];
    let q = mass_flow / fluid.density();
    println!( "q: {}", q );
    assert!( (q - 1.5).abs() < 1.0e-6 );
}