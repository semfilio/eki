use ohsl::{ vector::Vec64, matrix::Mat64 };
use crate::graph::Graph;
use crate::fluid::Fluid;
use crate::node::Node;

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

/*pub fn laminar_guess( network: &mut Graph, fluid: &Fluid, g: f64 ) -> (Vec64, Vec64) {
    let nu: f64 = fluid.kinematic_viscosity();
    let (n, m) = ( network.num_nodes(), network.num_edges() );
    let size = n + m;

    let k = network.k_matrix();
    let kt = network.incidence_matrix();

    let mut b = Vec64::new( size, 0.0 );
    let mut mat = Mat64::new( size, size, 0.0 );

    let mut q_rand = Vec64::random( m );
    for j in 0..m {
        q_rand[j] *= 0.01;
    }

    // Continuity equation at each node
    let mut continuity_residual = network.steady_consumption_q( fluid.density() );
    continuity_residual -= kt.clone() * q_rand.clone();
    for i in 0..n {
        for j in 0..m {
            mat[i][j] = kt[i][j];
        }
        b[i] = continuity_residual[i];
    }
    // Fill the resistance Jacobian matrix in bottom left corner
    //let q_zero = Vec64::new( m, 0.05 );
    for j in 0..m {
        let ( r, drdq ) = network.edges[j].r_drdq( q_rand[j], nu, g, 0 );
        //println!( "r = {}", r );
        //println!( "drdq = {}", drdq );
        mat[n + j][j] = -drdq;
        b[n + j] = r;
    }
    // Fill the K matrix in bottom right corner
    for i in 0..m {
        for j in 0..n {
            mat[n+i][m+j] = - k[i][j];
        }
    }
    // Insert boundary conditions 
    for i in 0..n {
        if network.nodes[i].is_known_pressure() {
            // Clear row
            for k in 0..n+m {
                mat[i][k] = 0.0;
                b[i] = 0.0;
            }
            mat[i][m+i] = 1.0;
            let head = network.nodes[i].steady_head( g, fluid.density() );
            b[i] = head;
        }
    }

    //println!("mat = {}", mat);
    //println!("b = {}", b);
    let correction = mat.solve_basic( b.clone() );

    let mut q_laminar = Vec64::new( m, 0.0 );
    let mut h_laminar = Vec64::new( n, 0.0 );

    update_solution( &mut q_laminar, &mut h_laminar, &correction );

    ( q_laminar, h_laminar )
}*/

pub fn laminar_guess( net: &Graph, fluid: &Fluid, g: f64 ) -> (Vec64, Vec64) {
    let ( num_nodes, numel ) = ( net.num_nodes(), net.num_edges() );
    let mut k_matrix = Mat64::new( num_nodes, num_nodes, 0.0 );

    for i in 0..numel {
        let (ifrom, ito) = net.edges()[i].id();
        let ( a, b ) = ( net.index(ifrom), net.index(ito) );
        let coefficient = net.edges()[i].k_laminar( fluid.kinematic_viscosity() );
        k_matrix[a][a] += coefficient;
        k_matrix[b][b] += coefficient;
        k_matrix[a][b] -= coefficient;
        k_matrix[b][a] -= coefficient;
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
        if bc.is_known_pressure() {
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