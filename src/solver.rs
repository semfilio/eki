use ohsl::{vector::Vec64, matrix::Mat64};
use crate::graph::Graph;
use crate::fluid::Fluid;

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
            dt: 0.1,
            g: 9.80665,
        }
    }
}

impl Solver {

    //TODO
    pub fn tnodes(&self) -> Vec<f64> {
        let num_time_steps: usize = ( self.tmax / self.dt ).ceil() as usize;
        let mut tnodes = vec![ 0.0 ];
        for n in 0..num_time_steps {
            tnodes.push( tnodes[n] + self.dt );
        }
        tnodes
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

    pub fn solver_type(&mut self) -> &mut SolverType {
        &mut self.solver_type
    }

    pub fn solved(&self) -> (bool, bool) {
        (self.solved_steady, self.solved_transient )
    }

    pub fn is_transient(&self) -> bool {
        self.solver_type == SolverType::Transient
    }

    pub fn solve_steady(&mut self, network: &mut Graph, fluid: &Fluid) -> Result<usize,f64> {
        let nu: f64 = fluid.kinematic_viscosity();
        let (n, m) = ( network.num_nodes(), network.num_edges() );
        let size = n + m;

        if size == 0 { return Err(1.0); }
        if m == 0 { return Err(1.0); }

        network.create_id_to_index();

        let k = network.k_matrix();
        let kt = network.incidence_matrix();

        let ( mut q_guess, mut h_guess ): ( ohsl::Vec64, ohsl::Vec64 );

        //TODO what about when elements are added to the network?
        //TODO could add random perutbation to known solution
        if self.solved_steady {
            ( q_guess, h_guess ) = network.steady_solution( fluid.density(), self.g );
            println!("Using previous solution.");
        } else {
            ( q_guess, h_guess ) = network.create_guess( fluid, self.g );
            println!("Creating a new guess.");
        }

        let mut iter: usize = 0;
        let mut max_residual = 1.0;
        while iter < self.max_iter && max_residual > self.tolerance {
            // Assemble the matrix problem
            let mut b = Vec64::new( size, 0.0 );
            let mut mat = Mat64::new( size, size, 0.0 );
            // Continuity equation at each node
            let mut continuity_residual = network.consumption();
            // Convert to volume flow rate
            for i in 0..n {
                continuity_residual[i] /= fluid.density();
            }
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
                let ( r, drdq ) = network.edges[j].r_drdq( q_guess[j], nu, self.g );
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
                    let head = network.nodes[i].head( self.g, fluid.density() );
                    b[i] = head - h_guess[i];
                }
            }


            //println!( "mat = {}", mat );
            //println!( "b = {}", b );
            // Solve the system of equations
            let correction = mat.solve_basic( b.clone() );
            //println!( "correction = {}", correction );
            // Update the solution
            for i in 0..m {
                q_guess[i] += correction[i];
            }
            for i in 0..n {
                h_guess[i] += correction[m+i];
            }
            // Check convergence & iterate
            max_residual = correction.norm_inf();
            //println!( "iter = {}\t residual = {:+.2e}", iter, max_residual );
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

    pub fn solve_transient(&mut self, network: &mut Graph, fluid: &Fluid ) -> Result<(),(f64,f64)> {
        //TODO network.initialise_transient( self.tnodes() );
        let t_max = *self.tmax();
        //println!( " * tmax = {}", t_max );
        let dt = *self.dt();
        //println!( " * dt = {}", dt );
        let num_time_steps: usize = ( t_max / dt ).ceil() as usize;
        //println!( " * # Time steps = {}", num_time_steps );
        let mut tnodes = vec![ 0.0 ];
        let ( mut qn, mut hn ) = network.steady_solution( fluid.density(), self.g );
        //println!( " * qn = {}", qn );
        //println!( " * hn = {}", hn );
        let mut converged = false;
        let mut error = ( 0.0, 0.0 ); 

        // Time stepping
        for n in 0..num_time_steps {
            tnodes.push( tnodes[n] + dt );
            let time = tnodes[n+1];
            // Single time step
            let result = self.time_step( network, n, &qn, &hn, fluid );
            match result {
                Ok( ( qg, hg ) ) => { 
                    /*println!( " * Time step {}: {:.2e}", n, time );
                    println!( " * q = {:?}", qg );
                    println!( " * h = {:?}", hg );*/
                    //TODO network.set_transient_solution( qg.clone(), hg.clone(), fluid, self.g(), n );
                    qn = qg;
                    hn = hg;
                    converged = true;
                },
                Err(residual) => { 
                    error = ( time, residual );
                    converged = false;
                    break; 
                },
            }
        }
        //self.tnodes = Some( tnodes );
        if converged {
            self.solved_transient = true;
            Ok(())
        } else {
            self.solved_transient = false;
            Err( error )
        }
    }

    fn time_step(&mut self, network: &mut Graph, step: usize, qn: &Vec64, hn: &Vec64, fluid: &Fluid ) -> Result<(Vec64,Vec64),f64> {
        let ( mut qg, mut hg ) = ( qn.clone(), hn.clone() );
        let (n, m) = ( network.num_nodes(), network.num_edges() );
        let size = n + m;
        if size == 0 { return Err(1.0); }
        if m == 0 { return Err(1.0); }
        let theta = 1.0; //TODO allow user specified theta
        let invdt = 1.0 / self.dt;

        let kt = network.incidence_matrix();
        let k = network.k_matrix();
        let d_diag = network.d_diag( fluid );
        let b_diag = network.b_diag();

        let mut iter: usize = 0;
        let mut max_residual = 1.0;
        // Iterate to convergence 
        while iter < self.max_iter && max_residual > self.tolerance {
            // Assemble the matrix problem
            let mut b = Vec64::new( size, 0.0 );
            let mut mat = Mat64::new( size, size, 0.0 );

            let qbar = theta * qg.clone() + ( 1.0 - theta ) * qn.clone();
            let hbar = theta * hg.clone() + ( 1.0 - theta ) * hn.clone();

            // Continuity equation at each node
            //let mut continuity_residual = network.transient_consumption( step + 1 );
            let mut continuity_residual = network.consumption();
            // Convert to volume flow rate
            for i in 0..n {
                continuity_residual[i] /= fluid.density();
            }
            continuity_residual -= kt.clone() * qbar.clone();
            let mut hdiff = hg.clone() - hn.clone();
            for i in 0..n {
                hdiff[i] *= d_diag[i];
            }
            continuity_residual -= invdt * hdiff;
            for i in 0..n {
                for j in 0..m {
                    mat[i][j] = theta * kt[i][j];
                }
                mat[i][m+i] = invdt * d_diag[i];
                b[i] = continuity_residual[i];
            }

            // Fill the resistance Jacobian matrix in bottom left corner
            let khbar = k.clone() * hbar.clone();
            for j in 0..m {
                let ( r, drdq ) = network.edges[j].r_drdq( qbar[j], fluid.kinematic_viscosity(), self.g );
                mat[n + j][j] = invdt * b_diag[j] - drdq;
                b[n + j] = r - invdt * b_diag[j] * ( qg[j] - qn[j] ) + khbar[j];
            }

            // Fill the K matrix in bottom right corner
            for i in 0..m {
                for j in 0..n {
                    mat[n+i][m+j] = - theta * k[i][j];
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
                    mat[i][m+i] = theta;
                    let head = network.nodes[i].head( self.g, fluid.density() );
                    b[i] = head - hbar[i];
                }
            }

            //println!( "mat = {}", mat );
            //println!( "b = {}", b );
            // Solve the system of equations
            let correction = mat.solve_basic( b.clone() );
            //println!( "correction = {}", correction );

            // Update the solution
            for i in 0..m {
                qg[i] += correction[i];
            }
            for i in 0..n {
                hg[i] += correction[m+i];
            }
            
            // Check convergence & iterate
            max_residual = correction.norm_inf();
            //println!( "iter = {}\t residual = {:+.2e}", iter, max_residual );
            iter += 1;
        }

        if iter < self.max_iter && !max_residual.is_nan() {
            Ok( ( qg, hg ) )
        } else {
            Err( max_residual )
        }

    }
}