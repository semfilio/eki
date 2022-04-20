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

    pub fn mut_nodes(&mut self) -> &mut Vec<Node> {
        &mut self.nodes
    }

    pub fn edges(&self) -> Vec<Edge> {
        self.edges.clone()
    }

    pub fn mut_edges(&mut self) -> &mut Vec<Edge> {
        &mut self.edges
    }

    /*pub fn count_links(&self, node: Node) -> usize {
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
    }*/

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

    // Return the vector of nodal consumptions (steady) [mdot]
    pub fn steady_consumption(&mut self) -> Vec64 {
        let n = self.num_nodes();
        let mut consumption = Vec64::new( n, 0.0 );
        for i in 0..n {
            if self.nodes[i].is_known_flow() {
                consumption[i] = (*self.nodes[i].consumption())[0];
            }
        }
        consumption
    }

    // Return the vector of nodal consumptions (steady) [Q]
    pub fn steady_consumption_q(&mut self, density: f64 ) -> Vec64 {
        let mut consumption = self.steady_consumption();
        for i in 0..self.num_nodes() {
            consumption[i] /= density;
        }
        consumption
    }

    // Return the vector of nodal consumptions and specified time step (transient) [mdot]
    pub fn consumption(&mut self, step: usize ) -> Vec64 {
        let n = self.num_nodes();
        let mut consumption = Vec64::new( n, 0.0 );
        for i in 0..n {
            if self.nodes[i].is_known_flow() {
                consumption[i] = (*self.nodes[i].consumption())[step];
            }
        }
        consumption
    }

    // Return the vector of nodal consumptions and specified time step (transient) [mdot]
    pub fn consumption_q(&mut self, step: usize, density: f64 ) -> Vec64 {
        let mut consumption = self.consumption( step);
        for i in 0..self.num_nodes() {
            consumption[i] /= density;
        }
        consumption
    }

    // Put the calculated steady solution into the network
    pub fn set_steady_solution(&mut self, q_guess: Vec64, h_guess: Vec64, rho: f64, g: f64 ) {
        let (m, n) = ( self.num_edges(), self.num_nodes() );
        for j in 0..m {
            *self.edges[j].steady_mass_flow() = q_guess[j] * rho;
        }
        for i in 0..n {
            let elevation = *self.nodes[i].elevation();
            *self.nodes[i].steady_pressure() = (h_guess[i] - elevation) * rho * g;
        }
    }

    // Return the steady solution as two vectors
    pub fn steady_solution_qh(&mut self, rho: f64, g: f64) -> (Vec64, Vec64) {
        let (m, n) = ( self.num_edges(), self.num_nodes() );
        let mut q_guess = Vec64::new( m, 0.0 );
        let mut h_guess = Vec64::new( n, 0.0 );
        for j in 0..m {
            q_guess[j] = *self.edges[j].steady_mass_flow() / rho;
        }
        for i in 0..n {
            let elevation = *self.nodes[i].elevation();
            h_guess[i] = (*self.nodes[i].steady_pressure() / (rho * g)) + elevation;
        }
        (q_guess, h_guess)
    }

    // Return the current solution as two vectors
    pub fn current_solution_qh(&mut self, rho: f64, g: f64, step: usize ) -> (Vec64, Vec64) {
        let (m, n) = ( self.num_edges(), self.num_nodes() );
        let mut q_guess = Vec64::new( m, 0.0 );
        let mut h_guess = Vec64::new( n, 0.0 );
        for j in 0..m {
            //q_guess[j] = self.edges[j].current_mass_flow() / rho;
            q_guess[j] = self.edges[j].mass_flow()[step] / rho;
        }
        for i in 0..n {
            let elevation = *self.nodes[i].elevation();
            h_guess[i] = (self.nodes[i].pressure()[step] / (rho * g)) + elevation;
        }
        (q_guess, h_guess)
    }

    // Put the calculated transient solution into the network
    pub fn push_transient_solution(&mut self, q_guess: Vec64, h_guess: Vec64, fluid: &Fluid, g: f64 ) {
        let (m, n) = ( self.num_edges(), self.num_nodes() );
        for j in 0..m {
            let mass_flow = self.edges[j].mass_flow();
            mass_flow.push( q_guess[j] * fluid.density() );
        }
        for i in 0..n {
            if !self.nodes[i].is_known_pressure() {
                let elevation = *self.nodes[i].elevation();
                let pressure = self.nodes[i].pressure();
                pressure.push( (h_guess[i] - elevation) * fluid.density() * g );
            }
        }
    }

    pub fn add_boundary_value( &mut self, id: usize, value: f64 ) {
        let index = self.index(id);
        self.nodes[index].add_boundary_value( value );
    }

    // TODO check doesn't already exist
    pub fn add_node(&mut self, node: Node ) {
        self.nodes.push( node );
    }

    // TODO check doesn't already exist
    pub fn add_edge(&mut self, edge: Edge ) {
        self.edges.push( edge );
    }

}