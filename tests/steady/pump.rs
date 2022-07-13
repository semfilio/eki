use eki::fluid::Fluid;
use eki::node::Node;
use eki::nodes::{ pressure::Pressure, connection::Connection };
use eki::edge::Edge;
use eki::edges::{ pump::Pump, pipe::Pipe };
use eki::graph::Graph;
use eki::solver::Solver;

#[test]
fn resistance() {
    let from = Node::Pressure( Pressure::new_elevation( 0, 0.0 ) );
    let to = Node::Pressure( Pressure::new_elevation( 1, 20.0 ) );
    let mut pump = Pump::new_params( from, to, vec![ 22.9, 10.7, -111.0 ], 
        ( 50.0 / (60.0 * 60.0), 50.0, 2950.0, 163.0e-3 ) );
    let head = pump.resistance( 0.23, 1.0, 1.0, 0 );
    assert_eq!( head, 22.9 + 10.7 * 0.23 - 111.0 * 0.23 * 0.23 );
    // Change the speed
    pump.speed = vec![ 2000.0 ];
    let head = pump.resistance( 0.23, 1.0, 1.0, 0 );
    let xi = ( 2950.0 / 2000.0 ) * ( 163.0e-3 / 163.0e-3 );
    let corrected_head = 22.9 * xi * xi + 10.7 * xi * 0.23 - 111.0 * 0.23 * 0.23;
    assert!( (head - corrected_head).abs() < 1.0e-8 );
}

#[test]
fn basic_pump() {
    let fluid = Fluid::new( 998.162, 1.1375e-6, 2.15e9 );
    let mut graph = Graph::new();

    let node_from = Node::Pressure( Pressure::new_elevation( 0, 0.0 ) );
    graph.add_node( node_from.clone() );

    let node_to = Node::Pressure( Pressure::new_elevation( 1, 20.0 ) );
    graph.add_node( node_to.clone() );

    let pump = Edge::Pump( Pump::new_params( node_from, node_to, 
        vec![ 
            46.0, -8460.59, 387449.0
            // (Q, dH)
            /*(0.00, 46.00),
            (13.32 / ( 60.0 * 60.0 ), 20.00),
            (36.80 / ( 60.0 * 60.0 ), 0.00)*/
        ], 
        ( 50.0 / (60.0 * 60.0), 50.0, 2950.0, 163.0e-3 )
    ));
    graph.add_edge( pump );

    let mut solver = Solver::default();
    let result = solver.solve_steady( &mut graph, &fluid, true );
    if let Err(residual) = result {
        println!( "residual = {}", residual );
    } 
    assert!( result.is_ok() && !result.is_err() );
    let volume_flow = *graph.edges()[0].steady_mass_flow() / fluid.density();
    assert!( (volume_flow - ( 13.32 / ( 60.0 * 60.0 ) )).abs() < 1.0e-8 );
}


/*#[test]
fn pump_and_pipe() {
    let fluid = Fluid::default();
    let mut graph = Graph::new();

    let node_from = Node::Pressure( Pressure::new( 0 ) );
    graph.add_node( node_from.clone() );

    let connection = Node::Connection( Connection::new( 1 ) );
    graph.add_node( connection.clone() );

    let node_to = Node::Pressure( Pressure::new_elevation( 2, 20.0 ) );
    graph.add_node( node_to.clone() );

    let pump = Edge::Pump( Pump::new_data( node_from, connection.clone(), 
        vec![ 
            (0.00 / ( 60.0 * 60.0 ), 46.00),
            (1.14 / ( 60.0 * 60.0 ), 45.98),
            (2.31 / ( 60.0 * 60.0 ), 45.89),
            (3.80 / ( 60.0 * 60.0 ), 45.76),
            (5.79 / ( 60.0 * 60.0 ), 45.50),
            (7.13 / ( 60.0 * 60.0 ), 45.19),
            (8.95 / ( 60.0 * 60.0 ), 44.62),
            (13.32 / ( 60.0 * 60.0 ), 42.59),
            (15.32 / ( 60.0 * 60.0 ), 41.36),
            (17.83 / ( 60.0 * 60.0 ), 39.38),
            (21.00 / ( 60.0 * 60.0 ), 36.50),
            (23.70 / ( 60.0 * 60.0 ), 33.70),
            (26.30 / ( 60.0 * 60.0 ), 30.60),
            (30.00 / ( 60.0 * 60.0 ), 23.00),
            (34.00 / ( 60.0 * 60.0 ), 11.00),
            (36.80 / ( 60.0 * 60.0 ), 0.00)
        ]
    ));
    graph.add_edge( pump );

    let mut pipe = Edge::Pipe( Pipe::new( connection, node_to ) );
    *pipe.length().unwrap() = 100.0;
    graph.add_edge( pipe );

    let mut solver = Solver::default();
    let result = solver.solve_steady( &mut graph, &fluid, true );
    if let Err(residual) = result {
        println!( "residual = {}", residual );
    } 
    assert!( result.is_ok() && !result.is_err() );

    let volume_flow = *graph.edges()[0].steady_mass_flow() / fluid.density();
    assert_eq!( volume_flow, 0.005979870051971544 );            //TODO get value from FD
    let rho_g = fluid.density() * solver.gravity();
    let h_from = graph.nodes()[0].steady_head( solver.gravity(), fluid.density() );
    assert_eq!( h_from, (101325.0 / rho_g) );
    let p_connection = *graph.nodes()[1].steady_pressure();
    let h_connection = graph.nodes()[1].steady_head( solver.gravity(), fluid.density() );
    assert_eq!( h_connection, p_connection / rho_g );
    assert_eq!( p_connection, 453585.47688527644 );             //TODO get value from FD
    let h_to = graph.nodes()[2].steady_head( solver.gravity(), fluid.density() );
    assert_eq!( h_to, (101325.0 / rho_g) + 20.0 );
}*/

#[test]
fn pipe_pump_pipe() {

}

#[test]
fn pipe_pump_valve_pipe() {
    
}

//TODO more pump testing
/*#[test]
fn pump_and_valve() {
    let fluid = Fluid::new( 998.162, 1.1375e-6, 2.15e9 );
    let mut graph = Graph::new();
    let node_from = Node::Pressure( Pressure::new_elevation( 0, 0.0 ) );
    graph.add_node( node_from.clone() );
        
    let connection1 = Node::Connection( Connection::new( 1 ) );
    // *connection1.steady_pressure() = 75043.0;
    graph.add_node( connection1.clone() );
    let connection2 = Node::Connection( Connection::new( 2 ) );
    // *connection2.steady_pressure() = 365164.0;
    graph.add_node( connection2.clone() );
    let connection3 = Node::Connection( Connection::new( 3 ) );
    // *connection3.steady_pressure() = 396508.0;
    graph.add_node( connection3.clone() );
    
    let node_to = Node::Pressure( Pressure::new_elevation( 4, 10.0 ) );
    graph.add_node( node_to.clone() );

    let mut pipe = Edge::Pipe( Pipe::new( node_from, connection1.clone() ) );
    *pipe.steady_mass_flow() = 0.002 * fluid.density();
    graph.add_edge( pipe );

    let mut pump = Edge::Pump( Pump::new_data( connection1, connection2.clone(), 
        vec![ 
            (0.00 / ( 60.0 * 60.0 ), 46.00),
            (1.14 / ( 60.0 * 60.0 ), 45.98),
            (2.31 / ( 60.0 * 60.0 ), 45.89),
            (3.80 / ( 60.0 * 60.0 ), 45.76),
            (5.79 / ( 60.0 * 60.0 ), 45.50),
            (7.13 / ( 60.0 * 60.0 ), 45.19),
            (8.95 / ( 60.0 * 60.0 ), 44.62),
            (13.32 / ( 60.0 * 60.0 ), 42.59),
            (15.32 / ( 60.0 * 60.0 ), 41.36),
            (17.83 / ( 60.0 * 60.0 ), 39.38),
            (21.00 / ( 60.0 * 60.0 ), 36.50),
            (23.70 / ( 60.0 * 60.0 ), 33.70),
            (26.30 / ( 60.0 * 60.0 ), 30.60),
            (30.00 / ( 60.0 * 60.0 ), 23.00),
            (34.00 / ( 60.0 * 60.0 ), 11.00),
            (36.80 / ( 60.0 * 60.0 ), 0.00)
        ]
    ));
    *pump.steady_mass_flow() = 0.002 * fluid.density();
    graph.add_edge( pump );

    let mut valve = Edge::Valve( Valve::new( connection2, connection3.clone() ) );
    *valve.k_values().unwrap() = vec![ 
        (0.000, 1.0e16),
        (5.000, 1200.0),
        (0.111, 400.),
        (0.222, 100.),
        (0.333, 40.),
        (0.444, 16.),
        (0.556, 7.0),
        (0.667, 3.3),
        (0.779, 1.6),
        (0.889, 0.48),
        (1.000, 0.05),
    ];
    *valve.steady_open_percent() = 1.0;
    *valve.steady_mass_flow() = 0.002 * fluid.density();
    graph.add_edge( valve );

    let mut pipe = Edge::Pipe( Pipe::new( connection3, node_to ) );
    *pipe.length().unwrap() = 100.0;
    *pipe.steady_mass_flow() = 0.002 * fluid.density();
    graph.add_edge( pipe );

    let mut solver = Solver::default();
    *solver.max_iter() = 30;
    let result = solver.solve_steady( &mut graph, &fluid, true );
    assert_eq!( *graph.edges()[3].length().unwrap(), 100.0 );
    if let Err(residual) = result {
        println!( "residual = {}", residual );
    } 
    assert!( result.is_ok() && !result.is_err() );

    let mass_flow = *graph.edges()[0].steady_mass_flow();
    //assert_eq!( mass_flow, 6.825793 );
}*/
