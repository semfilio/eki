use eki::fluid::Fluid;
use eki::node::Node;
use eki::nodes::{ pressure::Pressure, connection::Connection };
use eki::edge::Edge;
use eki::edges::{ pipe::Pipe, valve::Valve, pump::Pump };
use eki::graph::Graph;
use eki::solver::Solver;
use eki::events::{TransientEvent, Time, Value};

#[test]
fn add_transient_values() {
    let fluid = Fluid::default();
    let mut solver = Solver::default();
    let mut network = Graph::new();

    let mut node_from = Node::Pressure( Pressure::new_with_value( 0, 121325.0 ) );
    assert_eq!( *node_from.pressure(), vec![ 121325.0 ] );
    // Add a transient event 
    let time = Time( 0.5 );
    let pressure = Value( 131325.0 ); 
    let transient_event = TransientEvent::InstantaneousChange(pressure, time);
    node_from.add_event( transient_event );
    network.add_node( node_from.clone() );

    let mut connection = Node::Connection( Connection::new( 1 ) );
    assert_eq!( *connection.pressure(), vec![ 101325.0 ] );
    network.add_node( connection.clone() );

    let mut node_to = Node::Pressure( Pressure::new( 2 ) );
    assert_eq!( *node_to.pressure(), vec![ 101325.0 ] );
    network.add_node( node_to.clone() );
    
    let pipe = Edge::Pipe( Pipe::new( node_from, connection.clone() ) );
    network.add_edge( pipe );
    let pipe = Edge::Pipe( Pipe::new( connection, node_to.clone() ) );
    network.add_edge( pipe );

    let steady_result = solver.solve_steady( &mut network, &fluid, true );
    assert!( steady_result.is_ok() && !steady_result.is_err() );

    // Take a time step but add transient values first (Before event)
    let dt = 0.3; // Less than event time
    *solver.dt() = dt;
    let _result = solver.time_step( &mut network, &fluid );

    let pressure_from = network.nodes()[0].pressure().clone();
    assert_eq!( pressure_from.len(), 2 );
    assert!( (pressure_from[1] - 121325.0).abs() < 1.0e-8 );

    let pressure_connection = network.nodes()[1].pressure().clone();
    assert_eq!( pressure_connection.len(), 2 );
    assert!( (pressure_connection[1] - 111325.0).abs() < 1.0e-8 );

    let pressure_to = network.nodes()[2].pressure().clone();
    assert_eq!( pressure_to.len(), 2 );
    assert!( (pressure_to[1] - 101325.0).abs() < 1.0e-8 );

    let mass_flow = network.edges()[0].mass_flow().clone();
    assert_eq!( mass_flow.len(), 2 );
    assert!( (mass_flow[1] - 4.72854748).abs() < 1.0e-8 );

    // Take another time step (After event)
    *solver.dt() = dt;
    let _result = solver.time_step( &mut network, &fluid );

    let pressure_from = network.nodes()[0].pressure().clone();
    assert_eq!( pressure_from.len(), 3 );
    assert!( (pressure_from[2] - 131325.0).abs() < 1.0e-8 );

    let pressure_connection = network.nodes()[1].pressure().clone();
    assert_eq!( pressure_connection.len(), 3 );
    assert!( pressure_connection[2] > 111325.0 );

    let pressure_to = network.nodes()[2].pressure().clone();
    assert_eq!( pressure_to.len(), 3 );
    assert!( (pressure_to[2] - 101325.0).abs() < 1.0e-8 );

    let mass_flow = network.edges()[0].mass_flow().clone();
    assert_eq!( mass_flow.len(), 3 );
    assert!( mass_flow[2] > 4.72854748 );
}

#[test]
fn valve_closure() {
    let fluid = Fluid::default();
    let mut solver = Solver::default();
    let mut network = Graph::new();

    let mut node_from = Node::Pressure( Pressure::new_with_value( 1, 121325.0 ) );
    assert_eq!( *node_from.pressure(), vec![ 121325.0 ] );
    network.add_node( node_from.clone() );

    let mut node_to = Node::Pressure( Pressure::new( 2 ) );
    assert_eq!( *node_to.pressure(), vec![ 101325.0 ] );
    network.add_node( node_to.clone() );

    let mut valve = Edge::Valve( Valve::new( node_from, node_to ) );
    *valve.k_values().unwrap() = vec![ 
        (0.0, 1.0e16),
        (0.5, 7.0),
        (1.0, 0.25),
    ];
    *valve.steady_open_percent() = 1.0; // k = 0.25
    *valve.diameter() = 50.0e-3;        // D = 50mm

    let exponent = Value( 1.0 );  // Linear closing
    let event_time = Time( 0.0 );
    let closing_time = Time( 1.0 );
    let transient_event = TransientEvent::ValveClosure( exponent, event_time, closing_time );
    valve.add_event( transient_event );
    network.add_edge( valve );

    let steady_result = solver.solve_steady( &mut network, &fluid, true );
    assert!( steady_result.is_ok() && !steady_result.is_err() );

    let open_percent = network.edges()[0].open_percent().unwrap().clone();
    assert_eq!( open_percent, vec![ 1.0 ] );

    let steady_mass_flow = *network.edges()[0].steady_mass_flow();

    // Time step
    *solver.dt() = 0.5;
    *solver.max_iter() = 30;
    let _result = solver.time_step( &mut network, &fluid );

    let open_percent = network.edges()[0].open_percent().unwrap().clone();
    assert_eq!( open_percent, vec![ 1.0, 0.5 ] );

    let mass_flow = network.edges()[0].mass_flow().clone();
    assert!( mass_flow[1] < steady_mass_flow );

    // Another step
    let result = solver.time_step( &mut network, &fluid );

    match result {
        Ok(iterations) => {
            assert!( iterations < 21 );
            // Takes 20 iterations (we should be using a smaller time step)
        },
        Err(_) => {}
    }
    
    let open_percent = network.edges()[0].open_percent().unwrap().clone();
    assert_eq!( open_percent, vec![ 1.0, 0.5, 0.0 ] );

    let mass_flow = network.edges()[0].mass_flow().clone();
    assert!( mass_flow[2] < mass_flow[1] );
    assert!( mass_flow[2].abs() < 1.0e-3 )

}

#[test]
fn pump_linear_shutdown() {
    let fluid = Fluid::default();
    let mut network = Graph::new();

    let mut node_from = Node::Pressure( Pressure::new( 1 ) );
    assert_eq!( *node_from.pressure(), vec![ 101325.0 ] );
    network.add_node( node_from.clone() );

    let node_to = Node::Pressure( Pressure::new_elevation( 2, 10.0 ) );
    network.add_node( node_to.clone() );

    let new_pump = Pump::new( node_from, node_to );
    let mut pump = Edge::Pump( new_pump.clone() );
    let event_time = Time( 0.0 );
    let shutdown_time = Time( 10.0 );
    let transient_event = TransientEvent::PumpLinearShutdown( event_time.clone(), shutdown_time.clone() );
    pump.add_event( transient_event );
    network.add_edge( pump );

    let mut solver = Solver::default();
    let result = solver.solve_steady( &mut network, &fluid, true );
    if let Err(residual) = result {
        println!( "residual = {}", residual );
    } 
    assert!( result.is_ok() && !result.is_err() );

    let volume_flow = *network.edges()[0].steady_mass_flow() / fluid.density();
    let q = volume_flow / new_pump.q_rated;
    println!( "q = {}", q );
    let n: f64 = 1.0; // Using the rated speed
    let mut theta = n.atan2( q );
    if theta < 0.0 { theta += 2.0 * std::f64::consts::PI; }
    let fh = new_pump.f_h( theta );
    let h = ( n * n + q * q ) * fh;
    assert!( ( h - 10.0 / new_pump.h_rated ) < 1.0e-10 );
    let steady_mass_flow = *network.edges()[0].steady_mass_flow();

    let speed = network.edges()[0].speed().unwrap().clone();
    assert_eq!( speed, vec![ 11300. ] );

    let n = 50;

    // A time step
    *solver.dt() = 1.0 / (n as f64);
    *solver.max_iter() = 50;
    let result = solver.time_step( &mut network, &fluid );
    match result {
        Ok( iter ) => {
            let speed = network.edges()[0].speed().unwrap().clone();
            let mass_flow = network.edges()[0].mass_flow().clone();
            println!( "iterations = {}, dt = {}, speed = {}, mass_flow = {}", iter, *solver.dt(), *speed.last().unwrap(), *mass_flow.last().unwrap() );
            *solver.dt() *= if iter < 5 { 1.0 } else { 0.5 }; // Adaptive time-stepping
        },
        Err( residual ) => {
            println!("Residual = {:+.2e}", residual );
        }
    } 

    let speed = network.edges()[0].speed().unwrap().clone();
    assert_eq!( speed, vec![ 11300., 11300. * ( 1.0 - ( ( *solver.dt() - event_time.0 ) / shutdown_time.0 ) ) ] );

    let mass_flow = network.edges()[0].mass_flow().clone();
    assert!( mass_flow[1] < steady_mass_flow );

    let mut time = *solver.tnodes().last().unwrap();

    // Some more time steps
    while time < event_time.0 + shutdown_time.0 + 1.0 {
        let result = solver.time_step( &mut network, &fluid );
        match result {
            Ok( iter ) => {
                let speed = network.edges()[0].speed().unwrap().clone();
                let mass_flow = network.edges()[0].mass_flow().clone();
                println!( "iterations = {}, t = {}, speed = {}, mass_flow = {}", iter, time, *speed.last().unwrap(), *mass_flow.last().unwrap() );
                *solver.dt() *= if iter < 5 { 1.1 } else { 0.5 }; // Adaptive time-stepping
                time += *solver.dt();
            },
            Err( residual ) => {
                println!("Residual = {:+.2e}", residual );
                break;
            }
        }
    }

    let time = *solver.tnodes().last().unwrap();
    println!( "time = {:?}", time );
    let speed = network.edges()[0].speed().unwrap().clone();
    assert_eq!( *speed.last().unwrap(), 0.0 ); 
}