use eki::fluid::Fluid;
use eki::node::Node;
use eki::nodes::{ pressure::Pressure };
use eki::edge::Edge;
use eki::edges::{ pump::Pump };
use eki::graph::Graph;
use eki::solver::Solver;

#[test]
fn interpolate_head_data() {
    let from = Node::Pressure( Pressure::new_elevation( 0, 0.0 ) );
    let to = Node::Pressure( Pressure::new_elevation( 1, 20.0 ) );
    let pump = Pump::new( from, to );
    assert_eq!( pump.f_h( 0.0 ), -0.55 );
    let theta = (30.0_f64).to_radians();
    let h = pump.f_h( theta );
    assert_eq!( h, 0.06 );
    let theta = (102.5_f64).to_radians();
    let h = pump.f_h( theta );
    assert!( (h - 1.325).abs() < 1.0e-10 );
    let theta = (357.5_f64).to_radians();
    let h = pump.f_h( theta );
    assert!( (h - (-0.58)).abs() < 1.0e-10 );
}

#[test]
fn resistance() {
    let from = Node::Pressure( Pressure::new_elevation( 0, 0.0 ) );
    let to = Node::Pressure( Pressure::new_elevation( 1, 100.0 ) );
    let mut pump = Pump::new( from, to );
    let flow = 300.0 / ( 60.0 * 60.0 );         // 300 m^3/hour
    pump.speed[0] = 5650.0 / 3.0_f64.sqrt();    // 3262.03 rpm
    let dh = 0.0;
    let r = pump.resistance( flow, dh, 0.0, 9.81, 0 ) / ( 9.81 * pump.area()); 
    assert!( (r - 6.6).abs() < 1.0e-10 );  
}

#[test]
fn basic_steady_pump() {
    let fluid = Fluid::new_basic( 998.162, 1.1375e-6, 2.15e9 );
    let mut graph = Graph::new();

    let elevation = 100.0;
    let node_from = Node::Pressure( Pressure::new_elevation( 0, 0.0 ) );
    graph.add_node( node_from.clone() );
    let node_to = Node::Pressure( Pressure::new_elevation( 1, elevation ) );
    graph.add_node( node_to.clone() );

    let steady_speed: f64 = 5000.0;
    let new_pump = Pump::new( node_from, node_to );
    let mut pump = Edge::Pump( new_pump.clone() );
    if let Some(speed) = pump.speed() {
        speed[0] = steady_speed;
    }
    graph.add_edge( pump );

    let mut solver = Solver::default();
    let result = solver.solve_steady( &mut graph, &fluid, true );
    if let Err(residual) = result {
        println!( "residual = {}", residual );
    } 
    assert!( result.is_ok() && !result.is_err() );
    let volume_flow = *graph.edges()[0].steady_mass_flow() / fluid.density();
    
    let q = volume_flow / new_pump.q_rated;
    let n: f64 = steady_speed / new_pump.n_rated;
    let mut theta = n.atan2( q );
    if theta < 0.0 { theta += 2.0 * std::f64::consts::PI; }
    let fh = new_pump.f_h( theta );
    let h = ( n * n + q * q ) * fh;
    println!( "h = {}", h );
    println!( "elevation = {}", elevation / new_pump.h_rated );
    assert!( ( h - (elevation / new_pump.h_rated) ).abs() < 1.0e-10 );
    
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
