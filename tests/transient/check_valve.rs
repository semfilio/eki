use eki::fluid::Fluid;
use eki::node::Node;
use eki::nodes::{ pressure::Pressure };
use eki::edge::Edge;
use eki::edges::{ check_valve::CheckValve };
use eki::graph::Graph;
use eki::solver::Solver;

#[test]
fn transient_check_valve() {
    let mut graph = Graph::new();
    let fluid = Fluid::new_basic( 997.0, 1.1375e-6, 2.15e9 );
    let dt = 0.1;
    let tmax = 1.0;
    let n = (tmax / dt) as usize;

    let mut solver = Solver::default();
    *solver.dt() = dt;
    *solver.max_iter() = 50; // Need more iterations because sudden closing
    let rho_g = fluid.density() * solver.gravity();
    
    let atmospheric_pressure = 101325.0;
    let p_from = rho_g * 5.0 + atmospheric_pressure; // H_0 = 5m
    let mut node_from = Node::Pressure( Pressure::new( 0 ) );
    *node_from.pressure() = vec![ p_from; n];
    graph.add_node( node_from.clone() );
    let mut node_to = Node::Pressure( Pressure::new( 1 ) );
    *node_to.pressure() = vec![ // Pressure increase over time to create a negative pressure gradient
        atmospheric_pressure,
        rho_g * 1.0 + atmospheric_pressure,
        rho_g * 2.0 + atmospheric_pressure,
        rho_g * 3.0 + atmospheric_pressure,
        rho_g * 4.0 + atmospheric_pressure,
        rho_g * 5.0 + atmospheric_pressure,
        rho_g * 6.0 + atmospheric_pressure,
        rho_g * 7.0 + atmospheric_pressure,
        rho_g * 8.0 + atmospheric_pressure,
        rho_g * 9.0 + atmospheric_pressure,
        rho_g * 10.0 + atmospheric_pressure,
    ];
    graph.add_node( node_to.clone() );
    let mut check_valve = Edge::CheckValve( CheckValve::new( node_from, node_to ) );
    *check_valve.invk_values().unwrap() = vec![ 
        (0.0, 0.0),
        (1.0, 1. / 0.25),
    ];
    *check_valve.diameter().unwrap() = 50.0e-3;        // D = 50mm
    assert_eq!( *check_valve.steady_open_percent(), 1.0); // Should be open initially.
    graph.add_edge( check_valve );

    let result = solver.solve_steady( &mut graph, &fluid, true );
    assert!( result.is_ok() && !result.is_err() );

    // Flow should be positive because the check valve is open initially.
    let mass_flow = (*graph.edges()[0].mass_flow())[0];
    let q = mass_flow / fluid.density();
    assert!( q > 0.0 );

    // Transient simulation
    println!( "n = {}", n);
    for step in 0..n {
        let result = solver.time_step( &mut graph, &fluid );
        let mass_flow = (*graph.edges()[0].mass_flow())[step];
        let q = mass_flow / fluid.density();
        println!( "step = {}, result = {:?}, q = {}", step, result, q );
        assert!( result.is_ok() && !result.is_err() );
    }

    // Check that the check valve closes
    if let Some( open_percent ) = graph.edges()[0].open_percent() {
        assert_eq!( open_percent[0], 1.0 );
        assert_eq!( open_percent[1], 1.0 );
        assert_eq!( open_percent[2], 1.0 );
        assert_eq!( open_percent[3], 1.0 );
        assert_eq!( open_percent[4], 1.0 );
        assert_eq!( open_percent[5], 1.0 );
        assert_eq!( open_percent[6], 0.0 );
        assert_eq!( open_percent[7], 0.0 );
        assert_eq!( open_percent[8], 0.0 );
        assert_eq!( open_percent[9], 0.0 );
        assert_eq!( open_percent[10], 0.0 );
    }
    



}