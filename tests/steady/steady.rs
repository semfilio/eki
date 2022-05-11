use eki::fluid::Fluid;
use eki::node::Node;
use eki::nodes::{ pressure::Pressure };
use eki::edge::Edge;
use eki::edges::{ pipe::Pipe, valve::Valve };
use eki::graph::Graph;
use eki::solver::{Solver, SolverType};

mod three_reservoirs;
mod pipe;

#[test]
fn default() {
    let mut solver = Solver::default();
    assert_eq!( *solver.max_iter(), 20 );
    assert_eq!( *solver.tolerance(), 1e-8 );
    assert_eq!( *solver.tmax(), 5.0 );
    assert_eq!( *solver.dt(), 0.1 );
    assert_eq!( *solver.g(), 9.80665 );
    let ( steady, transient ) = solver.solved();
    assert_eq!( steady, false ); 
    assert_eq!( transient, false );
    assert_eq!( *solver.solver_type(), SolverType::Steady ); 
}

#[test]
fn edit_parameters() {
    let mut solver = Solver::default();
    *solver.max_iter() = 30;
    assert_eq!( *solver.max_iter(), 30 );
    *solver.tolerance() = 1e-6;
    assert_eq!( *solver.tolerance(), 1e-6 );
    *solver.tmax() = 10.0;
    assert_eq!( *solver.tmax(), 10.0 );
    *solver.dt() = 0.5;
    assert_eq!( *solver.dt(), 0.5 );
    *solver.g() = 9.81;
    assert_eq!( *solver.g(), 9.81 );
    *solver.solver_type() = SolverType::Transient;
    assert_eq!( *solver.solver_type(), SolverType::Transient );
}

#[test]
fn reset() {
    let mut solver = Solver::default();
    *solver.max_iter() = 17;
    solver.reset();
    assert_eq!( *solver.max_iter(), 20 );
}

#[test]
fn single_pipe() {
    let mut graph = Graph::new();
    let node_from = Node::Pressure( Pressure::new_with_value( 0, 121325.0 ) );
    graph.add_node( node_from.clone() );
    let node_to = Node::Pressure( Pressure::new( 1 ) );
    graph.add_node( node_to.clone() );
    let edge = Edge::Pipe( Pipe::new( node_from, node_to ) );
    graph.add_edge( edge );
    let fluid = Fluid::new( 997.0, 1.1375e-6, 2.15e9 );
    let mut solver = Solver::default();
    let result = solver.solve_steady( &mut graph, &fluid, true );
    assert!( result.is_ok() && !result.is_err() );
    if let Ok(iter) = result {
        assert_eq!( iter, 5 );
    }
    let mass_flow = *graph.edges()[0].steady_mass_flow();
    assert!( ( mass_flow - 6.7865862 ).abs() < 1.0e-6 );
}

#[test]
fn initial_guess() {
    let mut graph = Graph::new();
    let node_from = Node::Pressure( Pressure::new_with_value( 0, 121325.0 ) );
    graph.add_node( node_from.clone() );
    
    let node_to = Node::Pressure( Pressure::new( 1 ) );
    graph.add_node( node_to.clone() );
    
    let edge = Edge::Pipe( Pipe::new( node_from, node_to ) );
    graph.add_edge( edge );
    let fluid = Fluid::new( 997.0, 1.1375e-6, 2.15e9 );
    let mut solver = Solver::default();
    let create_guess = true;
    let result = solver.solve_steady( &mut graph, &fluid, create_guess );
    let mut iterations: usize = 0;
    if let Ok(iter) = result {
        assert_eq!( iter, 5 );
        iterations = iter;
    }

    // Modify network but use previous solution as initial guess
    let create_guess = false;
    // Change steady pressure of from node
    *graph.mut_nodes()[0].steady_pressure() = 122325.0;
    let result = solver.solve_steady( &mut graph, &fluid, create_guess );
    assert!( result.is_ok() && !result.is_err() );
    if let Ok(iter) = result {
        assert!( iter < iterations );
    }
    let mass_flow = *graph.edges()[0].steady_mass_flow();
    assert!( ( mass_flow - 6.960918 ).abs() < 1.0e-6 );
    // Change the length of the pipe
    *graph.mut_edges()[0].length().unwrap() = 11.0;
    let result = solver.solve_steady( &mut graph, &fluid, create_guess );
    assert!( result.is_ok() && !result.is_err() );
    if let Ok(iter) = result {
        assert!( iter < iterations );
    }
    let mass_flow = *graph.edges()[0].steady_mass_flow();
    assert!( ( mass_flow - 6.6243271 ).abs() < 1.0e-6 );
}

#[test]
fn steady_valve() {
    let mut graph = Graph::new();
    let node_from = Node::Pressure( Pressure::new_with_value( 0, 111325.0 ) );
    graph.add_node( node_from.clone() );
    let node_to = Node::Pressure( Pressure::new( 1 ) );
    graph.add_node( node_to.clone() );
    let mut valve = Edge::Valve( Valve::new( node_from, node_to ) );
    *valve.k_values().unwrap() = vec![ 
        (0.0, 1.0e16),
        (0.5, 7.0),
        (1.0, 0.25),
    ];
    *valve.steady_open_percent() = 0.5;
    graph.add_edge( valve );
    let fluid = Fluid::new( 997.0, 1.1375e-6, 2.15e9 );
    let mut solver = Solver::default();
    let result = solver.solve_steady( &mut graph, &fluid, true );
    assert!( result.is_ok() && !result.is_err() );
    let mass_flow = (*graph.edges()[0].mass_flow())[0];
    assert!( ( mass_flow - 3.6536088 ).abs() < 1.0e-6 );
}