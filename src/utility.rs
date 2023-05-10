use ohsl::{ vector::Vec64, matrix::Mat64 };
use crate::graph::Graph;
use crate::fluid::Fluid;
use crate::node::Node;

pub fn max_value( values: &mut Vec<f64>) -> f64 {
    let max = values.iter_mut().max_by(|a, b| a.partial_cmp(b).unwrap());
    *max.unwrap()
}

pub fn min_value( values: &mut Vec<f64>) -> f64 {
    let min = values.iter_mut().min_by(|a, b| a.partial_cmp(b).unwrap());
    *min.unwrap()
}

pub fn friction_factor( relative: f64, reynolds: f64 ) -> f64 {
    if reynolds < 2100.0 {
        64.0 / reynolds 
    } else if reynolds > 3000.0 {
        //TODO Praks-Brkic ( temporary )
        let a = reynolds * relative / 8.0897;
        let b = reynolds.ln() - 0.779626;
        let x = a + b;
        let c = x.ln();
        let k = 0.8685972 * ( b - c + ( c / ( x - 0.5588 * c + 1.2079 ) ) );
        1.0 / ( k * k )
    } else {
        let k1 = (64.0/reynolds).powi( 12 );
        let c = 1.0 / ( (0.833* reynolds.powf(1.282) / reynolds.powf(1.007) ) + ( 0.27 * relative ) 
        + (110.0*relative/ reynolds ));
        let a = 0.8687 * (c.powi( 16 )).ln();
        let b = ( 13269.0 / reynolds ).powi( 16 );
        let k2 = (a+b).powf( -1.5 );
        (k1+k2).powf( 0.08333333333 )
    }
}

pub fn update_solution( qg: &mut Vec64, hg: &mut Vec64, correction: &Vec64 ) {
    let m = qg.size();
    for i in 0..m {
        qg[i] += correction[i];
    }
    for i in 0..hg.size() {
        hg[i] += correction[m+i];
    }
}

pub fn laminar_guess( net: &Graph, fluid: &Fluid, g: f64 ) -> (Vec64, Vec64) {
    let ( num_nodes, numel ) = ( net.num_nodes(), net.num_edges() );
    let mut k_matrix = Mat64::new( num_nodes, num_nodes, 0.0 );

    for i in 0..numel {
        let (ifrom, ito) = net.edges()[i].id();
        let ( a, b ) = ( net.index(ifrom), net.index(ito) );
        let k = net.edges()[i].k_laminar( fluid.kinematic_viscosity() );
        k_matrix[a][a] += k;
        k_matrix[b][b] += k;
        k_matrix[a][b] -= k;
        k_matrix[b][a] -= k;
    }

    let mut boundary_conditions = Vec::<Node>::new();
    for node in 0..num_nodes {
        if !net.nodes()[node].is_connection() {
            boundary_conditions.push( net.nodes()[node].clone() );
        }
    }

    let mut consumption = Vec64::new( num_nodes, 0.0 );
    for mut bc in boundary_conditions {
        let node = net.index( bc.id() );
        if bc.is_known_pressure() || bc.is_tank() {
            let pressure = *bc.steady_pressure();
            let elevation = *bc.elevation();
            let val = elevation + pressure / ( fluid.density() * g );
            for i in 0..num_nodes {                     // (a) - add contributions
                consumption[i] -= k_matrix[i][node] * val;
            }
            for i in 0..num_nodes {                     // (b) - zero row and column
                k_matrix[node][i] = 0.0;
                k_matrix[i][node] = 0.0;
            }
            k_matrix[node][node] = 1.0;            // (c) - make mat(j,j) = 1
            consumption[node] = val;               // (d) - make consumption[j] = Hj

        } else { 
            consumption[node] += *bc.steady_consumption() / fluid.density();
        }
    }
    let head = k_matrix.solve_basic( consumption.clone() ); // Solve to find the heads at each node
    let mut flow_rate = Vec64::new( numel, 0.0 ); // Flow rate in each pipe (assuming friction factor = 0.1)
    for i in 0..numel {
        let (ifrom, ito) = net.edges()[i].id();
        let ( a, b ) = ( net.index(ifrom), net.index(ito) );
        let ( h_initial, h_final ) = ( head[a], head[b] );
        flow_rate[i] = net.edges()[i].darcy_approx( h_initial - h_final, g ) * ( h_initial - h_final );
        if flow_rate[i].is_nan() {
            flow_rate[i] = 0.0001;
        }
    }
    ( flow_rate, head )
}

pub fn relative_error( x: f64, x_approx: f64 ) -> f64 {
    let rel = 1.0 - ( x_approx / x );
    rel.abs()
}

pub fn interpolate( x: f64, x_data: &Vec<f64>, y_data: &Vec<f64> ) -> f64 {
    assert_eq!( x_data.len(), y_data.len() );
    let n = x_data.len();
    let mut i = 1;
    while x > x_data[i] && i < n-1 {
        i += 1;
    }
    let x_1 = x_data[i-1];
    let x_2 = x_data[i];
    let y_1 = y_data[i-1];
    let y_2 = y_data[i];
    let slope = ( y_2 - y_1 ) / ( x_2 - x_1 );
    y_1 + slope * ( x - x_1 )
}

pub fn split_into_two_vectors( data: &Vec<(f64, f64)> ) -> (Vec<f64>, Vec<f64>) {
    let mut x = Vec::<f64>::new();
    let mut y = Vec::<f64>::new();
    for i in 0..data.len() {
        x.push( data[i].0 );
        y.push( data[i].1 );
    }
    ( x, y )
}