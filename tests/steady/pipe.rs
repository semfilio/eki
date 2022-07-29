use std::f64::consts::PI;
use eki::fluid::Fluid;
use eki::node::Node;
use eki::nodes::{ pressure::Pressure, flow::Flow, connection::Connection };
use eki::edge::Edge;
use eki::edges::pipe::Pipe;
use eki::graph::Graph;
use eki::solver::Solver;
use eki::utility;

#[test]
fn two_pipes() {
    let mut graph = Graph::new();
    let fluid = Fluid::new_basic( 999.7, 1.3063e-6, 2.15e9 ); // Water @ 10 degrees C
    let mut solver = Solver::default(); 

    let pressure1 = fluid.density() * solver.gravity() * 20.0;

    let node1 = Node::Pressure( Pressure::new_with_value( 1, pressure1 ) );
    let node2 = Node::Connection( Connection::new( 2 ) );
    let node3 = Node::Pressure( Pressure::new( 3 ) );

    graph.add_node( node1.clone() );
    graph.add_node( node2.clone() );
    graph.add_node( node3.clone() );

    let (t, y) = (5.0e-3, 2.0e11);
    let (l, d, r) = (100.0, 50.0e-3, 0.0);
    let pipe1 = Edge::Pipe( Pipe::new_params( node1.clone(), node2.clone(), l, d, r, t, y ) );
    let (l, d, r) = (200.0, 50.0e-3, 0.0);
    let pipe2 = Edge::Pipe( Pipe::new_params( node2.clone(), node3.clone(), l, d, r, t, y ) );

    graph.add_edge( pipe1 );
    graph.add_edge( pipe2 );

    let result = solver.solve_steady( &mut graph, &fluid, true );
    assert!( result.is_ok() && !result.is_err() );

    let h0 = graph.nodes()[0].steady_head( solver.gravity(), fluid.density() );
    assert!( (h0 - 20.0).abs() < 1.0e-8 );
    let h1 = graph.nodes()[1].steady_head( solver.gravity(), fluid.density() );
    assert!( (h1 - 16.77845838).abs() < 1.0e-8 );
    let h2 = graph.nodes()[2].steady_head( solver.gravity(), fluid.density() );
    assert!( (h2 - 10.33537514).abs() < 1.0e-8 );

    let q = *graph.edges()[0].steady_mass_flow() / fluid.density();
    assert!( (q - 0.00239623).abs() < 1.0e-8 );
}


#[test]
fn turbulent_water() {
    let mut graph = Graph::new();
    let fluid = Fluid::new_basic( 998.2, 1.0034e-6, 2.15e9 ); // Water at 68F = 20C
    let flow_in = 500.0 * 0.000063141414 * fluid.density(); // 500 usgpm
    let node_from = Node::Flow( Flow::new_with_value( 1, flow_in ) );
    graph.add_node( node_from.clone() );
    let out_pressure = 14.7 * 6894.75729; // 14.7 psi a
    let node_to = Node::Pressure( Pressure::new_with_value( 2, out_pressure ) );
    graph.add_node( node_to.clone() );

    let length = 1000.0 * 0.3048; // 1000 ft
    let diameter = 77.928e-3; // 3 inch nominal schedule 40 = 77.928 mm
    let (t, y) = (5.0e-3, 2.0e11);
    let roughness = 0.5e-4;
    let pipe = Edge::Pipe( 
        Pipe::new_params( node_from.clone(), node_to.clone(), 
            length, diameter, roughness, t, y ) 
    );
    graph.add_edge( pipe );

    let mut solver = Solver::default();
    let result = solver.solve_steady( &mut graph, &fluid, true );
    assert!( result.is_ok() && !result.is_err() );

    let volume_flow = *graph.edges()[0].steady_mass_flow() / fluid.density();
    let flow_usgpm = volume_flow / 0.000063141414; 
    assert!( ( flow_usgpm - 500.0 ).abs() < 1.0e-6 );

    let area = 0.25 * PI * diameter * diameter;
    let reynolds = volume_flow * diameter / ( area * fluid.kinematic_viscosity() );
    assert!( (reynolds - 514075.0 ).abs() < 1.0 );
    assert!( utility::relative_error( 514000.0, reynolds ) < 0.01 ); // < 1%
    let relative: f64 = roughness / diameter;
    let friction_factor = utility::friction_factor( relative, reynolds );
    assert!( (friction_factor - 0.018469).abs() < 1.0e-5 );
    assert!( utility::relative_error( 0.0184, friction_factor ) < 0.01 ); // < 1%
    
    let p_from = *graph.nodes()[0].steady_pressure();
    let p_to = *graph.nodes()[1].steady_pressure();
    let p_drop = p_from - p_to;
    let p_drop_psi = p_drop / 6894.75729; 
    assert!( utility::relative_error( 227.0, p_drop_psi ) < 0.01 ); // < 1%
    
    let head_loss = p_drop / ( solver.gravity() * fluid.density() );
    let head_loss_ft = head_loss / 0.3048;
    assert!( utility::relative_error( 526.0, head_loss_ft ) < 0.01 ); // < 1%

}

#[test]
fn turbulent_oil() {
    let mut graph = Graph::new();
    let specific_gravity = 0.9;
    let density = specific_gravity * 1000.0;
    let nu = 1.107748845e-5; 
    let bulk = 1.66e9;
    let fluid = Fluid::new_basic( density, nu, bulk ); // Oil at 68F = 20C
    let flow_in = 84.0 * 0.000063141414 * fluid.density(); // 84 usgpm
    let node_from = Node::Flow( Flow::new_with_value( 1, flow_in ) );
    graph.add_node( node_from.clone() );
    let out_pressure = 14.7 * 6894.75729; // 14.7 psi a
    let node_to = Node::Pressure( Pressure::new_with_value( 2, out_pressure ) );
    graph.add_node( node_to.clone() );

    let length = 1000.0 * 0.3048; // 1000 ft
    let diameter = 3.068 * 0.0254; // 3.068 in
    let (t, y) = (5.0e-3, 2.0e11);
    let roughness = 0.0018 * 0.0254; // 0.0018 in
    let pipe = Edge::Pipe( 
        Pipe::new_params( node_from.clone(), node_to.clone(), 
            length, diameter, roughness, t, y ) 
    );
    graph.add_edge( pipe );

    let mut solver = Solver::default();
    let result = solver.solve_steady( &mut graph, &fluid, true );
    assert!( result.is_ok() && !result.is_err() );

    let volume_flow = *graph.edges()[0].steady_mass_flow() / fluid.density();
    let flow_usgpm = volume_flow / 0.000063141414; 
    assert!( ( flow_usgpm - 84.0 ).abs() < 1.0e-6 );

    let area = 0.25 * PI * diameter * diameter;
    let reynolds = volume_flow * diameter / ( area * fluid.kinematic_viscosity() );
    assert!( (reynolds - 7823.0 ).abs() < 1.0 );
    assert!( utility::relative_error( 7826.0, reynolds ) < 0.01 ); // < 1%
    let relative: f64 = roughness / diameter;
    let friction_factor = utility::friction_factor( relative, reynolds );
    assert!( (friction_factor - 0.0337 ).abs() < 1.0e-4 );

    let p_from = *graph.nodes()[0].steady_pressure();
    let p_to = *graph.nodes()[1].steady_pressure();
    let p_drop = p_from - p_to;
    let p_drop_psi = p_drop / 6894.75729;
    assert!( utility::relative_error( 10.7, p_drop_psi ) < 0.01 ); // < 1% 
    
    let head_loss = p_drop / ( solver.gravity() * fluid.density() );
    let head_loss_ft = head_loss / 0.3048;
    assert!( utility::relative_error( 27.5, head_loss_ft ) < 0.01 ); // < 1% 
}