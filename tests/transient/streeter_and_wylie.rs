use eki::fluid::Fluid;
use eki::node::Node;
use eki::nodes::{ pressure::Pressure, connection::Connection };
use eki::edge::Edge;
use eki::edges::{ pipe::Pipe, valve::Valve };
use eki::graph::Graph;
use eki::solver::Solver;
use eki::utility;

use std::f64::consts::PI;

#[test]
fn example3_1() {

    let dt = 0.1;
    let tmax = 4.3;
    let n = (tmax / dt) as usize;


    let mut solver = Solver::default();
    *solver.g() = 9.806;
    //*solver.max_iter() = 100;
    *solver.theta() = 0.9;
    let mut network = Graph::new();
    let fluid = Fluid::new( 997.0, 1.1375e-6, 2.15e9 );

    let reservoir_head = 150.0;
    let reservoir_pressure = fluid.density() * solver.gravity() * reservoir_head;
    let mut reservoir = Node::Pressure( Pressure::new_with_value( 1, reservoir_pressure  ) );
    //(*reservoir.pressure()).push( reservoir_pressure );
    *reservoir.pressure() = vec![reservoir_pressure; n]; // Initialise transient pressure values (constant)
    network.add_node( reservoir.clone() );

    let node2 = Node::Connection( Connection::new( 2 ) );
    network.add_node( node2.clone() );

    let node3 = Node::Connection( Connection::new( 3 ) );
    network.add_node( node3.clone() );

    let node4 = Node::Connection( Connection::new( 4 ) );
    network.add_node( node4.clone() );

    let node5 = Node::Connection( Connection::new( 5 ) );
    network.add_node( node5.clone() );

    let valve_node = Node::Connection( Connection::new( 6 ) );
    network.add_node( valve_node.clone() );

    let outlet_head = 101325.0 / ( fluid.density() * solver.gravity() );
    let dh = 143.49 - outlet_head;
    println!( "dH: {}", dh );
    let outlet_pressure = fluid.density() * solver.gravity() * outlet_head;
    let mut outlet = Node::Pressure( Pressure::new_with_value( 7, outlet_pressure ) );
    //(*outlet.pressure()).push( outlet_pressure );
    *outlet.pressure() = vec![outlet_pressure; n];
    network.add_node( outlet.clone() );

    // Choose parameters to fit the example (which use a fixed friction factor = 0.018)
    let (thickness, youngs) = (1.0803e-2, 2.0e11);
    let (length, diameter, roughness) = (120.0, 0.5, 0.3155006e-3);
    let pipe = Edge::Pipe( 
        Pipe::new_params( reservoir, node2.clone(), length, diameter, roughness, thickness, youngs ) 
    );
    network.add_edge( pipe.clone() ); 
    //*pipe.steady_mass_flow() = 0.477 * fluid.density(); 
    /*assert!( ( pipe.wave_speed( &fluid ) - 1200.0 ) < 1.0e-2 );
    let rel = roughness / diameter;
    let reynolds = 0.477 * diameter / ( pipe.area() * fluid.kinematic_viscosity() );
    let friction_factor = utility::friction_factor( rel, reynolds );
    assert!( (friction_factor - 0.018).abs() < 1.0e-3 );*/
    //assert_eq!( friction_factor, 0.018 );
    

    let pipe = Edge::Pipe( 
        Pipe::new_params( node2.clone(), node3.clone(), length, diameter, roughness, thickness, youngs ) 
    );
    network.add_edge( pipe );
    let pipe = Edge::Pipe( 
        Pipe::new_params( node3.clone(), node4.clone(), length, diameter, roughness, thickness, youngs ) 
    );
    network.add_edge( pipe );
    let pipe = Edge::Pipe( 
        Pipe::new_params( node4.clone(), node5.clone(), length, diameter, roughness, thickness, youngs ) 
    );
    network.add_edge( pipe );
    let pipe = Edge::Pipe( 
        Pipe::new_params( node5.clone(), valve_node.clone(), length, diameter, roughness, thickness, youngs ) 
    );
    network.add_edge( pipe );

    //TODO create a scheme for splitting pipes and pushing nodes / edges to network

    let area = PI * 0.5 * 0.5 / 4.;
    //let min_k = 2. * solver.gravity() * area * area * dh / ( 0.477 * 0.477 );
    //let min_k = area * area / ( 0.009 * 0.009 );
    let cd_av = 0.477 / ( 2.0 * solver.gravity() * dh ).sqrt();
    let min_k = area * area / ( cd_av * cd_av );
    let mut valve = Edge::Valve( Valve::new( valve_node, outlet ) );
    *valve.k_values().unwrap() = vec![ 
        (0.0, 1.0e16),
        (0.1, min_k / (0.1*0.1)),
        (0.2, min_k / (0.2*0.2)),
        (0.3, min_k / (0.3*0.3)),
        (0.4, min_k / (0.4*0.4)),
        (0.5, min_k / (0.5*0.5)),
        (0.6, min_k / (0.6*0.6)),
        (0.7, min_k / (0.7*0.7)),
        (0.8, min_k / (0.8*0.8)),
        (0.9, min_k / (0.9*0.9)),
        (1.0, min_k),
    ];
    *valve.steady_open_percent() = 1.0;
    //(*valve.open_percent().unwrap()).push( 0.929428641 );
    //let open_percent = vec![];
    let mut t = dt;
    let tc: f64 = 2.1;
    for i in 1..n {
        if t <= tc {
            let tau = ( 1.0 - ( t / tc ) ).powf( 1.5 );
            (*valve.open_percent().unwrap()).push( tau );
        } else {
            (*valve.open_percent().unwrap()).push( 0.0 );
        }
        t += dt;
    }
    //println!( "open_percent: {:?}", valve.open_percent().unwrap() );
    //assert_eq!( *valve.open_percent().unwrap(), vec![ 1.0, 0.929428641 ] );
    *valve.diameter() = 0.5;
    network.add_edge( valve );

    let steady_result = solver.solve_steady( &mut network, &fluid, true );
    assert!( steady_result.is_ok() && !steady_result.is_err() );
    if let Ok(iter) = steady_result {
        assert!( iter < 10 );
    }

    let valve_pressure = *network.nodes()[5].steady_pressure();
    let valve_head = valve_pressure / ( fluid.density() * solver.gravity() );
    assert!( (valve_head - 143.49).abs() < 1.0e-2 );
    
    
    let mass_flow = *network.edges()[0].steady_mass_flow();
    let volume_flow = mass_flow / fluid.density();
    assert!( ( volume_flow - 0.477 ).abs() < 1.0e-3 );

    


    let streeter = vec![
        143.49, 154.28, 165.79, 178.08, 191.11, 204.93, 
    ];

    for step in 1..6 {
        let result = solver.time_step( &mut network, &fluid, dt );
        assert!( result.is_ok() && !result.is_err() );
        let valve_pressure = (*network.nodes()[5].pressure())[step];
        let valve_head = valve_pressure / ( fluid.density() * solver.gravity() );
        let relative_error = ( streeter[step] - valve_head ) / streeter[step];
        assert!( relative_error.abs() < 0.01 );
    }
    
}