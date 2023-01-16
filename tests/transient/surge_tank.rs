use eki::fluid::Fluid;
use eki::node::Node;
use eki::nodes::{ pressure::Pressure, connection::Connection, tank::Tank };
use eki::edge::Edge;
use eki::edges::{ pipe::Pipe, valve::Valve };
use eki::graph::Graph;
use eki::solver::Solver;
use eki::events::{TransientEvent, Time, Value};
use eki::utility;

#[test]
fn pipe_tank_valve() {
    let fluid = Fluid::default();
    let mut solver = Solver::default();
    let mut network = Graph::new();

    /* --- Pipe - Connection - Valve */

    // Nodes
    let mut node_0 = Node::Pressure( Pressure::new_with_value( 0, 121325.0 ) );
    assert_eq!( *node_0.pressure(), vec![ 121325.0 ] );

    let node_1 = Node::Connection( Connection::new( 1 ) );

    let mut node_2 = Node::Pressure( Pressure::new( 2 ) );
    assert_eq!( *node_2.pressure(), vec![ 101325.0 ] );
    
    network.add_node( node_0.clone() );
    network.add_node( node_1.clone() );
    network.add_node( node_2.clone() );

    // Pipe
    let pipe = Edge::Pipe( Pipe::new( node_0.clone(), node_1.clone()) );
    network.add_edge( pipe );

    // Valve
    let mut valve = Edge::Valve( Valve::new( node_1.clone(), node_2.clone() ) );
    let exponent = Value( 1.5 );  // Linear closing
    let event_time = Time( 0.0 );
    let closing_time = Time( 1.0 );
    let transient_event = TransientEvent::ValveClosure( exponent, event_time, closing_time );
    valve.add_event( transient_event );
    network.add_edge( valve );

    // Solve steady
    let steady_result = solver.solve_steady( &mut network, &fluid, true );
    assert!( steady_result.is_ok() && !steady_result.is_err() );

    let open_percent = network.edges()[1].open_percent().unwrap().clone();
    assert_eq!( open_percent, vec![ 1.0 ] );

    // Transient solve (pipe - connection - valve)
    let mut t = 0.0;
    let tmax = 2.0;

    *solver.dt() = 0.1;
    *solver.max_iter() = 30;
    while t < tmax {
        let result = solver.time_step( &mut network, &fluid );
        t += *solver.dt();
        match result {
            Ok( iter ) => {
                *solver.dt() *= if iter < 5 { 1.1 } else { 0.5 }; // Adaptive time-stepping
            },
            Err( residual ) => {
                println!("Residual = {:+.2e}", residual );
            }
        }
    }

    let mut connection_pressure = network.nodes()[1].pressure().clone();
    let max_pressure_no_tank = utility::max_value( &mut connection_pressure );
    assert!( max_pressure_no_tank > 200000.0 );

    /* --- Replace connection with surge tank --- */

    let mut new_network = Graph::new();
    let mut new_solver = Solver::default();

    // Nodes
    let mut node_0 = Node::Pressure( Pressure::new_with_value( 0, 121325.0 ) );
    assert_eq!( *node_0.pressure(), vec![ 121325.0 ] );

    let node_1 = Node::Tank( Tank::new_with_values( 
        1, 
        101325.0,           // Atmospheric pressure [Pa]
        fluid.density(),    // Fluid density [kg/m^3]
        solver.gravity(),   // Gravitational acceleration [m/s^2]
        0.5,                // Tank diameter [m]
        1.0,                // Initial fluid height [m]
        0.5,                // Minimum fluid height [m]
        1.5,                // Maximum fluid height [m]
    ));

    let mut node_2 = Node::Pressure( Pressure::new( 2 ) );
    assert_eq!( *node_2.pressure(), vec![ 101325.0 ] );
    
    new_network.add_node( node_0.clone() );
    new_network.add_node( node_1.clone() );
    new_network.add_node( node_2.clone() );

    // Pipe
    let pipe = Edge::Pipe( Pipe::new( node_0.clone(), node_1.clone()) );
    new_network.add_edge( pipe );

    // Valve
    let mut valve = Edge::Valve( Valve::new( node_1.clone(), node_2.clone() ) );
    let exponent = Value( 1.5 );  // Linear closing
    let event_time = Time( 0.0 );
    let closing_time = Time( 1.0 );
    let transient_event = TransientEvent::ValveClosure( exponent, event_time, closing_time );
    valve.add_event( transient_event );
    new_network.add_edge( valve );

    // Solve steady
    let steady_result = new_solver.solve_steady( &mut new_network, &fluid, true );
    assert!( steady_result.is_ok() && !steady_result.is_err() );

    // Transient solve (pipe - connection - valve)
    t = 0.0;

    *new_solver.dt() = 0.1;
    *new_solver.max_iter() = 30;
    while t < tmax {
        let result = new_solver.time_step( &mut new_network, &fluid );
        t += *new_solver.dt();
        match result {
            Ok( iter ) => {
                *new_solver.dt() *= if iter < 5 { 1.1 } else { 0.5 }; // Adaptive time-stepping TODO option for on/off
            },
            Err( residual ) => {
                println!("Residual = {:+.2e}", residual );
            }
        }
    }

    let mut surge_tank_pressure = new_network.nodes()[1].pressure().clone();
    let max_pressure_with_tank = utility::max_value( &mut surge_tank_pressure );
    //assert_eq!( max_pressure_no_tank, max_pressure_with_tank );
    assert!( max_pressure_with_tank < max_pressure_no_tank );
}

//TODO test against experimental data or other solutions