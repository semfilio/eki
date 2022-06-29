use ohsl::{vector::Vec64, matrix::Mat64};
use crate::graph::Graph;
use crate::fluid::Fluid;
use crate::utility;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Solver {
    solver_type: SolverType,    // Type of solver Steady or Transient
    solved_steady: bool,        // True if the steady problem has been solved 
    solved_transient: bool,     // True if the transient problem has been solved
    max_iter: usize,            // Maximum number of iterations
    tolerance: f64,             // Tolerance for convergence
    tmax: f64,                  // Maximum simulation time [s]
    dt: f64,                    // Time step [s]
    g: f64,                     // Acceleration due to gravity [m/s^2]
    tnodes: Vec<f64>,           // Time vector for transient solver [s]
    theta: f64,                 // Numerical scheme parameter
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub enum SolverType {
    Steady,
    Transient,
}

impl std::fmt::Debug for SolverType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolverType::Steady => write!(f, "Steady"),
            SolverType::Transient => write!(f, "Transient"),
        }
    }
}

impl Default for Solver {
    fn default() -> Self {
        Solver {
            solver_type: SolverType::Steady,
            solved_steady: false,
            solved_transient: false,
            max_iter: 20,
            tolerance: 1.0e-8,
            tmax: 5.0,
            dt: 0.1, //TODO do we need this?
            g: 9.80665,
            tnodes: vec![0.0],
            theta: 1.0, // 0 = explicit, 1 = implicit, 0.5 = Crank-Nicolson
        }
    }
}

impl Solver {

    pub fn tnodes(&mut self) -> Vec<f64> {
        self.tnodes.clone()
    }

    pub fn reset_tnodes(&mut self) {
        self.tnodes = vec![0.0]
    }

    pub fn theta(&mut self) -> &mut f64 {
        &mut self.theta
    }

    pub fn reset(&mut self) {
        self.solved_steady = false;
        self.solved_transient = false;
        self.reset_parameters();
    }

    pub fn reset_parameters(&mut self) {
        self.max_iter = 20;
        self.tolerance = 1.0e-8;
        self.tmax = 5.0;
        self.dt = 0.1;
        self.g = 9.80665;
    }

    pub fn max_iter(&mut self) -> &mut usize {
        &mut self.max_iter
    }

    pub fn tolerance(&mut self) -> &mut f64 {
        &mut self.tolerance
    }

    pub fn dt(&mut self) -> &mut f64 {
        &mut self.dt
    }

    pub fn tmax(&mut self) -> &mut f64 {
        &mut self.tmax
    }

    pub fn g(&mut self) -> &mut f64 {
        &mut self.g
    }

    pub fn gravity(&self) -> f64 {
        self.g
    }

    pub fn solver_type(&mut self) -> &mut SolverType {
        &mut self.solver_type
    }

    pub fn solved(&self) -> (bool, bool) {
        (self.solved_steady, self.solved_transient )
    }

    pub fn is_transient(&self) -> bool {
        self.solver_type == SolverType::Transient
    }

    pub fn solve_steady(&mut self, network: &mut Graph, fluid: &Fluid, create_guess: bool ) 
        -> Result<usize,f64> 
    {
        let nu: f64 = fluid.kinematic_viscosity();
        let (n, m) = ( network.num_nodes(), network.num_edges() );
        let size = n + m;
        if size == 0 { return Err(1.0); }
        if m == 0 { return Err(1.0); }

        network.create_id_to_index();

        let k = network.k_matrix();
        let kt = network.incidence_matrix();

        let ( mut q_guess, mut h_guess ): ( ohsl::Vec64, ohsl::Vec64 );
        if create_guess {
            ( q_guess, h_guess ) = utility::laminar_guess( network, fluid, self.g );
            //println!( "q_guess = {}", q_guess );
            //println!( "h_guess = {}", h_guess );
        } else {
            ( q_guess, h_guess ) = network.steady_solution_qh( fluid.density(), self.g );
        }

        let mut iter: usize = 0;
        let mut max_residual = 1.0;
        while iter < self.max_iter && max_residual > self.tolerance {
            let mut b = Vec64::new( size, 0.0 );
            let mut mat = Mat64::new( size, size, 0.0 );
            // Continuity equation at each node
            let mut continuity_residual = network.steady_consumption_q( fluid.density() );
            continuity_residual -= kt.clone() * q_guess.clone();
            for i in 0..n {
                for j in 0..m {
                    mat[i][j] = kt[i][j];
                }
                b[i] = continuity_residual[i];
            }
            // Fill the resistance Jacobian matrix in bottom left corner
            let khg = k.clone() * h_guess.clone();
            for j in 0..m {
                let ( r, drdq ) = network.edges[j].r_drdq( q_guess[j], nu, self.g, 0 );
                mat[n + j][j] = -drdq;
                b[n + j] = r + khg[j];
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
                    let head = network.nodes[i].steady_head( self.g, fluid.density() );
                    b[i] = head - h_guess[i];
                }
            }

            //println!( "mat = {}", mat );
            let correction = mat.solve_basic( b.clone() );

            //println!( "q_guess = {}", q_guess );
            //println!( "h_guess = {}", h_guess );
            //println!( "correction = {}", correction );
            utility::update_solution( &mut q_guess, &mut h_guess, &correction );
            
            max_residual = correction.norm_inf();
            iter += 1;
        }

        if iter < self.max_iter && !max_residual.is_nan() {
            network.set_steady_solution( q_guess, h_guess, fluid.density(), self.g );
            self.solved_steady = true;
            Ok(iter)
        } else {
            self.solved_steady = false;
            Err(max_residual)
        }
    }

    pub fn time_step(&mut self, network: &mut Graph, fluid: &Fluid ) -> Result<usize,f64> {
        let step = self.tnodes.len() - 1;
        //println!("Time step {}", step);
        let ( qn, hn ) = network.current_solution_qh( fluid.density(), self.g, step ); 
        let ( mut qg, mut hg ) = ( qn.clone(), hn.clone() );
        let dt = *self.dt();
        let invdt = 1.0 / dt;

        // Create extra values in vectors using events
        let time = self.tnodes[step] + dt;
        for node in network.mut_nodes() {
            node.add_transient_value( time );
        }
        for edge in network.mut_edges() {
            edge.add_transient_value( time );
        }

        
        
        let (n, m) = ( network.num_nodes(), network.num_edges() );
        let size = n + m;
        if size == 0 { return Err(1.0); }
        if m == 0 { return Err(1.0); }
        if !self.solved_steady { return Err(1.0); }
        

        let kt = network.incidence_matrix();
        let k = network.k_matrix();
        let d_diag = network.d_diag( fluid );
        let b_diag = network.b_diag();

        let mut iter: usize = 0;
        let mut max_residual: f64 = 1.0;
        // Iterate to convergence 
        while iter < self.max_iter && max_residual > self.tolerance {
            // Assemble the matrix problem
            let mut b = Vec64::new( size, 0.0 );
            let mut mat = Mat64::new( size, size, 0.0 );

            let qbar = self.theta * qg.clone() + ( 1.0 - self.theta ) * qn.clone();
            let hbar = self.theta * hg.clone() + ( 1.0 - self.theta ) * hn.clone();

            // Continuity equation at each node
            let mut continuity_residual = network.consumption_q( step + 1, fluid.density() );
            continuity_residual -= kt.clone() * qbar.clone();
            let mut hdiff = hg.clone() - hn.clone();
            for i in 0..n {
                hdiff[i] *= d_diag[i];
            }
            continuity_residual -= invdt * hdiff;
            for i in 0..n {
                for j in 0..m {
                    mat[i][j] = self.theta * kt[i][j];
                }
                mat[i][m+i] = invdt * d_diag[i];
                b[i] = continuity_residual[i];
            }

            // Fill the resistance Jacobian matrix in bottom left corner
            let khbar = k.clone() * hbar.clone();
            for j in 0..m {
                let ( r, drdq ) = network.edges[j].r_drdq( qbar[j], fluid.kinematic_viscosity(), self.g, step + 1 );
                mat[n + j][j] = invdt * b_diag[j] - drdq;
                b[n + j] = r - invdt * b_diag[j] * ( qg[j] - qn[j] ) + khbar[j];
            }

            // Fill the K matrix in bottom right corner
            for i in 0..m {
                for j in 0..n {
                    mat[n+i][m+j] = - self.theta * k[i][j];
                }
            }

            // Insert boundary conditions 
            for i in 0..n {
                if network.nodes[i].is_known_pressure() {
                    // Clear row
                    for k in 0..size {
                        mat[i][k] = 0.0;
                        b[i] = 0.0;
                    }
                    mat[i][m+i] = self.theta;
                    let head = network.nodes[i].head( self.g, fluid.density() );
                    b[i] = head[ step + 1 ] - hbar[i];
                }
            }

            let correction = mat.solve_basic( b.clone() );
            utility::update_solution( &mut qg, &mut hg, &correction );
            max_residual = correction.norm_inf();
            iter += 1;
        }
        
        

        if iter < self.max_iter && !max_residual.is_nan() {
            let t = *self.tnodes.last().unwrap();
            self.tnodes.push( t + dt );
            //println!("qg = {:?}", qg);
            //println!("hg = {:?}", hg);
            //println!("iter = {}", iter);
            network.push_transient_solution( qg, hg, fluid, *self.g() );
            self.solved_transient = true;
            Ok( iter )
        } else {
            self.solved_transient = false;
            Err( max_residual )
        }

    }
}