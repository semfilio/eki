use eki::fluid::Fluid;
use eki::fluids::{
    water::Water
};
use eki::node::Node;
use eki::nodes::{ pressure::Pressure };
use eki::edge::Edge;
use eki::edges::{ 
    size_change::SizeChange,
};
use eki::graph::Graph;
use eki::solver::Solver;

#[test]
fn contraction() {
    let mut graph = Graph::new();
    let fluid = Fluid::Water( Water::new( 273.15 + 20.0 ) );    // Water @ 20 degrees C
    let mut solver = Solver::default(); 

    let node_from = Node::Pressure( Pressure::new_elevation( 1, 20.0 ) );
    let node_to = Node::Pressure( Pressure::new( 2 ) );

    graph.add_node( node_from.clone() );
    graph.add_node( node_to.clone() );

    let beta = 0.5; // < 1 = Contraction
    let size_change = Edge::SizeChange( SizeChange::new_params( 
        node_from.clone(), node_to.clone(), 100.0e-3, beta
    ) );
    graph.add_edge( size_change );

    let result = solver.solve_steady( &mut graph, &fluid, true );
    assert!( result.is_ok() && !result.is_err() );

    let q = *graph.edges()[0].steady_mass_flow() / fluid.density();
    assert!( (q - 0.2298781702).abs() < 1.0e-8 );
}

#[test]
fn expansion() {
    let mut graph = Graph::new();
    let fluid = Fluid::Water( Water::new( 273.15 + 20.0 ) );    // Water @ 20 degrees C
    let mut solver = Solver::default(); 

    let node_from = Node::Pressure( Pressure::new_elevation( 1, 20.0 ) );
    let node_to = Node::Pressure( Pressure::new( 2 ) );

    graph.add_node( node_from.clone() );
    graph.add_node( node_to.clone() );

    let beta = 2.0; // > 1 = Expansion
    let size_change = Edge::SizeChange( SizeChange::new_params( 
        node_from.clone(), node_to.clone(), 100.0e-3, beta
    ) );
    graph.add_edge( size_change );

    let result = solver.solve_steady( &mut graph, &fluid, true );
    assert!( result.is_ok() && !result.is_err() );

    let q = *graph.edges()[0].steady_mass_flow() / fluid.density();
    assert!( (q - 0.2074048708).abs() < 1.0e-8 );
}

#[test]
fn contraction_reverse_flow() {
    let mut graph = Graph::new();
    let fluid = Fluid::Water( Water::new( 273.15 + 20.0 ) );    // Water @ 20 degrees C
    let mut solver = Solver::default(); 

    let node_from = Node::Pressure( Pressure::new( 1 ) );
    let node_to = Node::Pressure( Pressure::new_elevation( 2, 20.0 ) );

    graph.add_node( node_from.clone() );
    graph.add_node( node_to.clone() );

    let beta = 0.5; // < 1 = Contraction
    let size_change = Edge::SizeChange( SizeChange::new_params( 
        node_from.clone(), node_to.clone(), 100.0e-3, beta
    ) );
    graph.add_edge( size_change );

    let result = solver.solve_steady( &mut graph, &fluid, true );
    assert!( result.is_ok() && !result.is_err() );

    let q = *graph.edges()[0].steady_mass_flow() / fluid.density();
    assert!( (q + 0.05185121771).abs() < 1.0e-8 );
}

#[test]
fn expansion_reverse_flow() {
    let mut graph = Graph::new();
    let fluid = Fluid::Water( Water::new( 273.15 + 20.0 ) );    // Water @ 20 degrees C
    let mut solver = Solver::default(); 

    let node_from = Node::Pressure( Pressure::new( 1 ) );
    let node_to = Node::Pressure( Pressure::new_elevation( 2, 20.0 ) );

    graph.add_node( node_from.clone() );
    graph.add_node( node_to.clone() );

    let beta = 2.0; // > 1 = Expansion
    let size_change = Edge::SizeChange( SizeChange::new_params( 
        node_from.clone(), node_to.clone(), 100.0e-3, beta
    ) );
    graph.add_edge( size_change );

    let result = solver.solve_steady( &mut graph, &fluid, true );
    assert!( result.is_ok() && !result.is_err() );

    let q = *graph.edges()[0].steady_mass_flow() / fluid.density();
    assert!( (q + 0.9195126808).abs() < 1.0e-8 );
}