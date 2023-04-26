use eki::fluid::Fluid;
use eki::node::Node;
use eki::nodes::{ pressure::Pressure };
use eki::edge::Edge;
use eki::edges::{ safety_valve::SafetyValve };
use eki::graph::Graph;
use eki::solver::Solver;

#[test]
fn transient_safety_valve() {
    let mut graph = Graph::new();
    let fluid = Fluid::new_basic( 997.0, 1.1375e-6, 2.15e9 );
    let dt = 0.1;
    let tmax = 1.0;
    let n = (tmax / dt) as usize;
    let mut solver = Solver::default();
    *solver.dt() = dt;
    
    let atmospheric_pressure = 101325.0;
    let mut node_from = Node::Pressure( Pressure::new( 0 ) );
    *node_from.pressure() = vec![ 
        atmospheric_pressure,
        atmospheric_pressure,
        atmospheric_pressure,
        atmospheric_pressure,
        atmospheric_pressure + 10000.0,
        atmospheric_pressure + 20000.0, // Pressure difference is above the set dp
        atmospheric_pressure + 30000.0,
        atmospheric_pressure + 40000.0,
        atmospheric_pressure + 50000.0,
        atmospheric_pressure + 60000.0,
    ];
    graph.add_node( node_from.clone() );
    let mut node_to = Node::Pressure( Pressure::new( 1 ) );
    *node_to.pressure() = vec![ atmospheric_pressure; n ];
    graph.add_node( node_to.clone() );
    let set_pressure = 10000.0; 
    let mut safety_valve = Edge::SafetyValve( SafetyValve::new( node_from, node_to, set_pressure ) );
    *safety_valve.invk_values().unwrap() = vec![ 
        (0.0, 0.0),
        (1.0, 1. / 0.25),
    ];
    *safety_valve.diameter() = 50.0e-3;        // D = 50mm
    assert_eq!( *safety_valve.steady_open_percent(), 0.0); // Should be closed initially.
    graph.add_edge( safety_valve );

    let result = solver.solve_steady( &mut graph, &fluid, true );
    assert!( result.is_ok() && !result.is_err() );

    // Flow should be zero because the safety valve is closed initially.
    let mass_flow = (*graph.edges()[0].mass_flow())[0];
    let q = mass_flow / fluid.density();
    assert!( q.abs() < 1.0e-8 );

    // Transient simulation
    println!( "n = {}", n);
    for step in 0..n {
        let result = solver.time_step( &mut graph, &fluid );
        let mass_flow = (*graph.edges()[0].mass_flow())[step];
        let q = mass_flow / fluid.density();
        println!( "step = {}, result = {:?}, q = {:.2e}", step, result, q );
        assert!( result.is_ok() && !result.is_err() );
    }

    // Check that the check valve closes
    if let Some( open_percent ) = graph.edges()[0].open_percent() {
        assert_eq!( open_percent[0], 0.0 );
        assert_eq!( open_percent[1], 0.0 );
        assert_eq!( open_percent[2], 0.0 );
        assert_eq!( open_percent[3], 0.0 );
        assert_eq!( open_percent[4], 0.0 );
        assert_eq!( open_percent[5], 0.0 );
        assert_eq!( open_percent[6], 1.0 );
        assert_eq!( open_percent[7], 1.0 );
        assert_eq!( open_percent[8], 1.0 );
        assert_eq!( open_percent[9], 1.0 );
        assert_eq!( open_percent[10], 1.0 );
    }
    



}