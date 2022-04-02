use eki::node::Node;
use eki::nodes::{ pressure::Pressure, flow::Flow };
use eki::edge::Edge;
use eki::edges::{ pipe::Pipe };
use eki::graph::Graph;
use eki::solver::{Solver, SolverType};

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

