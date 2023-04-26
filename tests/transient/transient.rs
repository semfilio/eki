/*use eki::fluid::Fluid;
use eki::node::Node;
use eki::nodes::{ pressure::Pressure, connection::Connection, flow::Flow };
use eki::edge::Edge;
use eki::edges::{ pipe::Pipe };
use eki::graph::Graph;*/
use eki::solver::{Solver, SolverType};

mod streeter_and_wylie;
mod events;
mod surge_tank;
mod check_valve;

#[test]
fn initialise() {
    let mut solver = Solver::default();
    *solver.solver_type() = SolverType::Transient;
    assert_eq!( *solver.solver_type(), SolverType::Transient );
    *solver.theta() = 0.5;
    assert_eq!( *solver.theta(), 0.5 );
}

/*#[test]
fn time_step() {
    let mut solver = Solver::default();
    let mut network = Graph::new();
    
    let mut node_from = Node::Pressure( Pressure::new( 0 ) );
    *node_from.steady_pressure() = 121325.0;
    (*node_from.pressure()).push( 122325.0 );
    assert_eq!( *node_from.pressure(), vec![ 121325.0, 122325.0 ] );
    network.add_node( node_from.clone() );

    let mut connection = Node::Connection( Connection::new( 1 ) );
    assert_eq!( *connection.pressure(), vec![ 101325.0 ] );
    network.add_node( connection.clone() );

    let mut node_to = Node::Flow( Flow::new( 2 ) );
    (*node_to.consumption()).push( -0.05 );
    assert_eq!( *node_to.consumption(), vec![ -0.1, -0.05 ] );
    network.add_node( node_to.clone() );
    
    let edge = Edge::Pipe( Pipe::new( node_from, connection.clone() ) );
    network.add_edge( edge );
    let edge = Edge::Pipe( Pipe::new( connection, node_to.clone() ) );
    network.add_edge( edge );

    let fluid = Fluid::new( 997.0, 1.1375e-6, 2.15e9 );
    let g = *solver.g();
    let rho = fluid.density();
    let _result = solver.solve_steady( &mut network, &fluid, true );
    let (q_steady, h_steady) = network.steady_solution_qh( rho, g );
    assert!( (h_steady[0] * rho * g - 121325.0).abs() < 1e-8 );
    assert!( (h_steady[1] * rho * g - 121318.81).abs() < 1e-2 );
    assert!( (h_steady[2] * rho * g - 121312.63).abs() < 1e-2 );
    assert!( (q_steady[0] * rho - 0.1).abs() < 1e-8 );
    assert!( (q_steady[1] * rho - 0.1).abs() < 1e-8 );

    *solver.dt() = 0.1;
    
    let _result = solver.time_step( &mut network, &fluid );
    assert_eq!( solver.tnodes(), vec![ 0.0, 0.1 ] );
    let (_q_current, h_current) = network.current_solution_qh( rho, g, 1 );
    assert!( (h_current[0] * rho * g - 122325.0).abs() < 1e-8 );
    assert!( (h_current[1] * rho * g - 124600.46).abs() < 1e-2 );
    assert!( (h_current[2] * rho * g - 126892.88).abs() < 1e-2 );

    // Need to add boundary values for the next time step
    network.add_boundary_value( 0, 123325.0 );
    network.add_boundary_value( 2, 0.0 );
    let _result = solver.time_step( &mut network, &fluid );
    assert!( (solver.tnodes()[2] - 0.2).abs() < 1e-8 );
    let (_q_current, h_current) = network.current_solution_qh( rho, g, 2 );
    assert!( (h_current[0] * rho * g - 123325.0).abs() < 1e-8 );
    assert_eq!( *network.nodes()[2].consumption(), vec![ -0.1, -0.05, 0.0 ] );
}*/