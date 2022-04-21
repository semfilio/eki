#[derive(serde::Deserialize, serde::Serialize)]
pub struct Fluid {
    pub rho: f64,       // Density [kg/m^3]
    pub nu: f64,        // Kinematic viscosity [m^2/s]
    pub bulk: f64,      // Bulk modulus of elasticity [Pa]
}

impl Default for Fluid {
    fn default() -> Self { // Assume the fluid is water at 15 degrees C & 1 bar
        Fluid {
            rho: 999.1,
            nu: 1.1385e-6,
            bulk: 2.15e9,
        }
    }
}

impl Fluid {
    pub fn new(rho: f64, nu: f64, bulk: f64) -> Self {
        Fluid { rho, nu, bulk }
    }

    pub fn reset_parameters(&mut self) {
        self.rho = 999.1;
        self.nu = 1.1385e-6;
        self.bulk = 2.15e9;
    }

    pub fn density(&self) -> f64 {
        self.rho
    }

    pub fn kinematic_viscosity(&self) -> f64 {
        self.nu
    }

    pub fn bulk_modulus(&self) -> f64 {
        self.bulk
    }
}