use eki::fluid::Fluid;

#[test]
fn fluid() {
    let mut fluid = Fluid::default();
    assert_eq!(fluid.density(), 997.0); 
    assert_eq!(fluid.kinematic_viscosity(), 1.1375e-6);
    assert_eq!(fluid.bulk_modulus(), 2.15e9);
    fluid.rho = Some(1000.0);
    fluid.nu = Some(1.0e-6);
    fluid.bulk = Some(2.0e9);
    assert_eq!(fluid.density(), 1000.0);
    assert_eq!(fluid.kinematic_viscosity(), 1.0e-6);
    assert_eq!(fluid.bulk_modulus(), 2.0e9);
}