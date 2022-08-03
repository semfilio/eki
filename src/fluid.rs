use crate::fluids::{
    basic_fluid::BasicFluid,
    water::Water,
};

#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub enum Fluid {
    BasicFluid(BasicFluid),  
    Water(Water),
}

impl std::fmt::Display for Fluid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fluid::BasicFluid(_fluid) => write!(f, "Basic Fluid"),
            Fluid::Water(_fluid) => write!(f, "Water"),
        }
    }
}

impl Default for Fluid {
    fn default() -> Self {
        Fluid::BasicFluid( BasicFluid::default() )
    }
}

impl Fluid {
    pub fn text(&self) -> String {
        match self {
            Fluid::BasicFluid(_) => "Basic Fluid".to_string(),
            Fluid::Water(_) => "Water".to_string(),
        }
    }

    pub fn new_basic( rho: f64, nu: f64, bulk: f64 ) -> Self {
        Fluid::BasicFluid( BasicFluid::new(rho, nu, bulk) )
    }

    pub fn rho(&mut self) -> Option<&mut f64> {
        match self {
            Fluid::BasicFluid(fluid) => Some(&mut fluid.rho),
            Fluid::Water(_fluid) => None,
        }
    }

    pub fn nu(&mut self) -> Option<&mut f64> {
        match self {
            Fluid::BasicFluid(fluid) => Some(&mut fluid.nu),
            Fluid::Water(_fluid) => None,
        }
    }

    pub fn bulk(&mut self) -> Option<&mut f64> {
        match self {
            Fluid::BasicFluid(fluid) => Some(&mut fluid.bulk),
            Fluid::Water(_fluid) => None,
        }
    }

    pub fn temperature(&mut self) -> Option<&mut f64> {
        match self {
            Fluid::BasicFluid(_fluid) => None,
            Fluid::Water(fluid) => Some(&mut fluid.temperature),
        }
    }

    pub fn max_temperature(&self) -> Option<f64> {
        match self {
            Fluid::BasicFluid(_fluid) => None,
            Fluid::Water(fluid) => Some( fluid.max_temperature() ),
        }
    }

    pub fn min_temperature(&self) -> Option<f64> {
        match self {
            Fluid::BasicFluid(_fluid) => None,
            Fluid::Water(fluid) => Some( fluid.min_temperature() ),
        }
    }
    
    pub fn reset_parameters(&mut self) {
        match self {
            Fluid::BasicFluid(fluid) => fluid.reset_parameters(),
            Fluid::Water(fluid) => fluid.reset_parameters(),
        }
    }
    
    pub fn density(&self) -> f64 {
        match self {
            Fluid::BasicFluid(fluid) => fluid.density(),
            Fluid::Water(fluid) => fluid.density(),
        }
    }
    
    pub fn kinematic_viscosity(&self) -> f64 {
        match self {
            Fluid::BasicFluid(fluid) => fluid.kinematic_viscosity(),
            Fluid::Water(fluid) => fluid.kinematic_viscosity(),
        }
    }
    
    pub fn bulk_modulus(&self) -> f64 {
        match self {
            Fluid::BasicFluid(fluid) => fluid.bulk_modulus(),
            Fluid::Water(fluid) => fluid.bulk_modulus(),
        }
    }
}

/*#[derive(serde::Deserialize, serde::Serialize)]
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
}*/