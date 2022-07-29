use eki::fluid::Fluid;
use eki::fluids::{
    water::Water,
};

#[test]
fn fluid() {
    let mut fluid = Fluid::default();
    assert_eq!(fluid.density(), 999.1); 
    assert_eq!(fluid.kinematic_viscosity(), 1.1385e-6);
    assert_eq!(fluid.bulk_modulus(), 2.15e9);
    if let Some(rho) = fluid.rho() {
        *rho = 1000.0;
    }
    if let Some(nu) = fluid.nu() {
        *nu = 1.0e-6;
    }
    if let Some(bulk) = fluid.bulk() {
        *bulk = 2.0e9;
    }
    assert_eq!(fluid.density(), 1000.0);
    assert_eq!(fluid.kinematic_viscosity(), 1.0e-6);
    assert_eq!(fluid.bulk_modulus(), 2.0e9);
}

#[test]
fn water() {
    let mut fluid = Fluid::Water( Water::new( 273.15 + 20.0 ) );    // Water @ 20 degrees C
    assert_eq!(fluid.density(), 998.21 );
    assert_eq!(fluid.kinematic_viscosity(), 1.002e-3 / 998.21 );
    assert_eq!(fluid.bulk_modulus(), 998.21 * 1481. * 1481. );
    if let Some(temp) = fluid.temperature() {
        *temp = 273.15 + 47.0;                                      // Water @ 47 degrees C
    }
    assert_eq!(fluid.density(), 0.5 * ( 989.79 + 988.92 ) );
    assert_eq!(fluid.min_temperature().unwrap(), 273.15 );
    assert_eq!(fluid.max_temperature().unwrap(), 273.15 + 100.0 );
}