use ohsl::{vector::Vec64, matrix::Mat64};
use std::collections::HashMap;

use crate::node::Node;
use crate::edge::Edge;
use crate::fluid::Fluid;

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub id_to_index: HashMap<usize, usize>
}

impl Graph {
    pub fn new() -> Self {
        let nodes: Vec<Node> = Vec::new();
        let edges: Vec<Edge> = Vec::new();
        let id_to_index: HashMap<usize, usize> = HashMap::new();
        Graph { nodes, edges, id_to_index }
    }

    pub fn nodes(&self) -> Vec<Node> {
        self.nodes.clone()
    }

    pub fn edges(&self) -> Vec<Edge> {
        self.edges.clone()
    }

    pub fn count_links(&self, node: Node) -> usize {
        let mut count = 0;
        for edge in self.edges.clone() {
            if edge.from().id() == node.id() {
                count += 1;
            }
            if edge.to().id() == node.id() {
                count += 1;
            }
        }
        count
    }

    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }
    
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn taken_ids(&self) -> Vec<usize> {
        let mut taken_ids: Vec<usize> = Vec::new();
        for node in self.nodes() {
            taken_ids.push( node.id() );
        }
        taken_ids
    }

    pub fn create_id_to_index(&mut self) {
        self.id_to_index.clear();
        for id in self.taken_ids() {
            self.id_to_index.insert(id, self.id_to_index.len());
        }
    }

    pub fn index(&self, id: usize) -> usize {
        self.id_to_index.get(&id).copied().unwrap()
    }

    pub fn k_matrix(&self) -> Mat64 {
        let (m, n) = ( self.num_edges(), self.num_nodes() );
        let mut mat = Mat64::new( m, n, 0.0 );
        for i in 0..m {
            let (ifrom, ito) = self.edges[i].id();
            mat[i][self.index(ifrom)] = 1.0;
            mat[i][self.index(ito)] = -1.0;
        }
        mat
    }

    // Return the incidence matrix K^T
    pub fn incidence_matrix(&self) -> Mat64 {
        let mut mat = self.k_matrix();
        mat.transpose_in_place();
        mat
    }

    pub fn kplus_matrix(&self) -> Mat64 {
        let (m, n) = ( self.num_edges(), self.num_nodes() );
        let mut mat = Mat64::new( m, n, 0.0 );
        for i in 0..m {
            let ifrom = self.edges[i].from().id();
            mat[i][self.index(ifrom)] = 1.0;
        }
        mat
    }

    pub fn kminus_matrix(&self) -> Mat64 {
        let (m, n) = ( self.num_edges(), self.num_nodes() );
        let mut mat = Mat64::new( m, n, 0.0 );
        for i in 0..m {
            let ito = self.edges[i].to().id();
            mat[i][self.index(ito)] = 1.0;
        }
        mat
    }

    // Return the matrix M = diag( g Aj Lj / 2 aj^2 ) 
    pub fn m_diag(&mut self, fluid: &Fluid) -> Vec64 {
        let mut m = Vec64::new( self.num_edges(), 0.0 );
        for i in 0..self.num_edges() {
            let a: f64 = self.edges[i].wave_speed( fluid );
            let area = self.edges[i].area();
            //TODO don't hard code g?
            if let Some(length) = self.edges[i].length() {
                m[i] = 0.5 * 9.806 * area * (*length) / ( a * a );
            } else {
                m[i] = 0.5 * 9.806 * area / ( a * a );
            }
        }
        m
    }

    fn mult_m_diag(&mut self, mut kmat: Mat64, fluid: &Fluid ) -> Mat64{
        let m = self.num_edges();
        let m_diag = self.m_diag( fluid );
        for j in 0..m {
            kmat[j] = m_diag[j] * kmat[j].clone();
        }
        kmat
    }

    // Return the diagonal matrix D = K+^T M K+ + K-^T M K- 
    pub fn d_diag(&mut self, fluid: &Fluid ) -> Vec64 {
        let mut d = Vec64::new( self.num_nodes(), 0.0 );
        let ( mut kplus, mut kminus ) = ( self.kplus_matrix(), self.kminus_matrix() );
        let ( kplust, kminust ) = ( kplus.transpose(), kminus.transpose() );
        kplus = self.mult_m_diag( kplus, fluid );
        let plus = kplust * kplus;
        kminus = self.mult_m_diag( kminus, fluid );
        let minus = kminust * kminus;
        for i in 0..self.num_nodes() {
            d[i] = plus[i][i] + minus[i][i];
        }
        d
    }

    // Return the diagonal matrix B = diag( L_j / g A_j )
    pub fn b_diag(&mut self) -> Vec64 {
        let mut b = Vec64::new( self.num_edges(), 0.0 );
        for j in 0..self.num_edges() {
            let area = self.edges[j].area();
            // TODO don't hard code g
            if let Some(length) = self.edges[j].length() {
                b[j] = (*length) / ( 9.806 * area );
            } else {
                b[j] = 1.0 / ( 9.806 * area );
            }
        }
        b
    }

    // Return the vector of nodal consumptions (steady)
    pub fn consumption(&mut self) -> Vec64 {
        let n = self.num_nodes();
        let mut consumption = Vec64::new( n, 0.0 );
        for i in 0..n {
            if self.nodes[i].is_known_flow() {
                /*if let Some( steady_consumption ) = self.nodes[i].consumption() {
                    consumption[i] = steady_consumption;
                }*/
                consumption[i] = *self.nodes[i].consumption();
            }
        }
        consumption
    }

    // Return the vector of nodal consumptions and specified time step (transient)
    /*pub fn transient_consumption(&mut self, step: usize ) -> Vec64 {
        let n = self.num_nodes();
        let mut consumption = Vec64::new( n, 0.0 );
        for i in 0..n {
            if self.nodes[i].is_known_flow() {
                //TODO deal with option properly
                consumption[i] = self.nodes[i].consumption().unwrap()[step];
            }
        }
        consumption
    }*/

    // Put the calculated steady solution into the network
    pub fn set_steady_solution(&mut self, q_guess: Vec64, h_guess: Vec64, rho: f64, g: f64 ) {
        let (m, n) = ( self.num_edges(), self.num_nodes() );
        for j in 0..m {
            /*if let Some(mass_flow) = self.edges[j].mass_flow() {
                mass_flow[0] = q_guess[j] * rho;
            }*/
            *self.edges[j].mass_flow() = q_guess[j] * rho;
        }
        for i in 0..n {
            //let elevation = *self.nodes[i].elevation().unwrap();
            /*if let Some(pressure) = self.nodes[i].pressure() {
                pressure[0] = (h_guess[i] - elevation) * rho * g;
            }*/
            let elevation = *self.nodes[i].elevation();
            *self.nodes[i].pressure() = (h_guess[i] - elevation) * rho * g;
        }
    }

    // Return the steady solution as two vectors
    pub fn steady_solution(&mut self, rho: f64, g: f64) -> (Vec64, Vec64) {
        let (m, n) = ( self.num_edges(), self.num_nodes() );
        let mut q_guess = Vec64::new( m, 0.0 );
        let mut h_guess = Vec64::new( n, 0.0 );
        for j in 0..m {
            /*if let Some(mass_flow) = self.edges[j].mass_flow() {
                q_guess[j] = mass_flow[0] / rho;
            }*/
            q_guess[j] = *self.edges[j].mass_flow() / rho;
        }
        for i in 0..n {
            //let elevation = *self.nodes[i].elevation().unwrap();
            /*if let Some(pressure) = self.nodes[i].pressure() {
                h_guess[i] = (pressure[0] / (rho * g)) + elevation;
            }*/
            let elevation = *self.nodes[i].elevation();
            h_guess[i] = (*self.nodes[i].pressure() / (rho * g)) + elevation;
        }
        (q_guess, h_guess)
    }

    // Initialise the transient solution using the steady solution
    /*pub fn initialise_transient(&mut self, tnodes: Vec<f64> ) {
        let n = tnodes.len();
        for mut node in self.nodes() {
            let length = if let Some(pressure) = node.pressure() { pressure.len() } else { n };
            if n != length {
                node.create_transient_values( &tnodes );
                self.update_node( node );
            }
        }
        for mut edge in self.edges() {
            let length = if let Some(mass_flow) = edge.mass_flow() { mass_flow.len() } else { n };
            if n != length {
                edge.create_transient_values( &tnodes );
                self.update_edge( edge );
            }
        }
    }*/

    // Put the calculated transient solution into the network
    /*pub fn set_transient_solution(&mut self, q_guess: Vec64, h_guess: Vec64, fluid: &Fluid, g: f64, step: usize ) {
        let (m, n) = ( self.num_edges(), self.num_nodes() );
        for j in 0..m {
            let mass_flow = self.edges[j].mass_flow().unwrap();
            mass_flow[step+1] = q_guess[j] * fluid.density();
        }
        for i in 0..n {
            if !self.nodes[i].is_known_pressure() {
                let elevation = *self.nodes[i].elevation().unwrap();
                let pressure = self.nodes[i].pressure().unwrap();
                pressure[step+1] = (h_guess[i] - elevation) * fluid.density() * g;

            }
        }
    }*/

    pub fn create_guess(&self, fluid: &Fluid, g: f64 ) -> (Vec64, Vec64) {
        let ( num_nodes, numel ) = ( self.num_nodes(), self.num_edges() );
        let mut k_matrix = Mat64::new( num_nodes, num_nodes, 0.0 );

        for i in 0..numel {
            let (ifrom, ito) = self.edges[i].id();
            let ( a, b ) = ( self.index(ifrom), self.index(ito) );
            let coefficient = self.edges[i].k_laminar( fluid.kinematic_viscosity() );
            k_matrix[a][a] += coefficient;
            k_matrix[b][b] += coefficient;
            k_matrix[a][b] -= coefficient;
            k_matrix[b][a] -= coefficient;
        }

        let mut boundary_conditions = Vec::<Node>::new();
        for node in 0..num_nodes {
            if !self.nodes[node].is_connection() {
                boundary_conditions.push( self.nodes[node].clone() );
            }
        }

        let mut consumption = Vec64::new( num_nodes, 0.0 );
        for mut bc in boundary_conditions {
            let node = self.index( bc.id() );
            if bc.is_known_pressure() {
                //let pressure = bc.pressure().unwrap()[0]; 
                //let elevation = *bc.elevation().unwrap();
                let pressure = *bc.pressure(); 
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
                //consumption[node] += bc.consumption().unwrap()[0] / fluid.density();
                consumption[node] += *bc.consumption() / fluid.density();
            }
        }
        let head = k_matrix.solve_basic( consumption.clone() ); // Solve to find the heads at each node
        let mut flow_rate = Vec64::new( numel, 0.0 ); // Flow rate in each pipe (assuming friction factor = 0.1)
        for i in 0..numel {
            let (ifrom, ito) = self.edges[i].id();
            let ( a, b ) = ( self.index(ifrom), self.index(ito) );
            let ( h_initial, h_final ) = ( head[a], head[b] );
            flow_rate[i] = self.edges[i].darcy_approx( h_initial - h_final, g ) * ( h_initial - h_final );
            if flow_rate[i].is_nan() {
                flow_rate[i] = 0.0001;
            }
        }
        ( flow_rate, head )
    }

    /* ----- UI functions ----- */




    // TODO check doesn't already exist
    pub fn add_node(&mut self, node: Node ) {
        self.nodes.push( node );
    }

    // TODO check doesn't already exist
    pub fn add_edge(&mut self, edge: Edge ) {
        self.edges.push( edge );
    }

}