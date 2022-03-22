#[derive(serde::Deserialize, serde::Serialize)]
pub struct Fluid {
    pub rho: Option<f64>,       // Density [kg/m^3]
    pub nu: Option<f64>,        // Kinematic viscosity [m^2/s]
    pub bulk: Option<f64>,      // Bulk modulus of elasticity [Pa]
}

impl Default for Fluid {
    fn default() -> Self {
        Fluid {
            rho: Some( 997.0 ),
            nu: Some( 1.1375e-6 ),
            bulk: Some( 2.15e9 ),
        }
    }
}

impl Fluid {
    pub fn reset_parameters(&mut self) {
        self.rho = Some( 997.0 );
        self.nu = Some( 1.1375e-6 );
        self.bulk = Some( 2.15e9 );
    }

    pub fn density(&self) -> f64 {
        self.rho.unwrap_or( 997.0 ) // Assume the fluid is water
    }

    pub fn kinematic_viscosity(&self) -> f64 {
        self.nu.unwrap_or( 1.1375e-6 ) // Assume the fluid is water
    }

    pub fn bulk_modulus(&self) -> f64 {
        self.bulk.unwrap_or( 2.15e9 ) // Assume the fluid is water
    }
}