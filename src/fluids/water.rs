use crate::utility;

#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub struct Water {
    pub temperature: f64,       // Temperature [K]
    temp_data: Vec<f64>,        // Temperature data [K]
    rho_data: Vec<f64>,         // Density data [kg/m^3]
    sound_data: Vec<f64>,       // Sound speed data [m/s]
    viscosity_data: Vec<f64>,   // Dynamic viscosity data [Pa s]
}

impl Default for Water {
    fn default() -> Self { // Assume the fluid is water at 15 degrees C & 1 bar
        Water {
            temperature: 273.15 + 15.0,
            temp_data: temp_data(),
            rho_data: rho_data(),
            sound_data: sound_data(),
            viscosity_data: viscosity_data(),
        }
    }
}

impl Water {
    pub fn new( temperature: f64 ) -> Self {
        Water { 
            temperature,
            temp_data: temp_data(),
            rho_data: rho_data(),
            sound_data: sound_data(),
            viscosity_data: viscosity_data(),
        }
    }

    pub fn reset_parameters(&mut self) {
        self.temperature = 273.15 + 15.0;
    }

    pub fn density(&self) -> f64 {
        utility::interpolate( self.temperature, &self.temp_data, &self.rho_data )
    }

    pub fn kinematic_viscosity(&self) -> f64 {
        let mu = self.viscosity();
        let rho = self.density();
        mu / rho
    }

    pub fn viscosity(&self) -> f64 {
        utility::interpolate( self.temperature, &self.temp_data, &self.viscosity_data )
    }

    pub fn bulk_modulus(&self) -> f64 {
        let rho = self.density();
        let c = self.sound_speed();
        rho * c * c
    }

    pub fn sound_speed(&self) -> f64 {
        utility::interpolate( self.temperature, &self.temp_data, &self.sound_data )
    }

    pub fn max_temperature(&self) -> f64 {
        self.temp_data.last().unwrap().clone()
    }

    pub fn min_temperature(&self) -> f64 {
        self.temp_data.first().unwrap().clone()
    }
}

/* --- Data from Pipe Flow ( Rennels ) */

fn temp_data() -> Vec<f64> {
    vec![
        273.15,
        275.15,
        277.15,
        279.15,
        281.15,
        283.15,
        285.15,
        287.15,
        289.15,
        291.15,
        293.15,
        295.15,
        297.15,
        299.15,
        301.15,
        303.15,
        305.15,
        307.15,
        309.15,
        311.15,
        313.15,
        315.15,
        317.15,
        319.15,
        321.15,
        323.15,
        325.15,
        327.15,
        329.15,
        331.15,
        333.15,
        335.15,
        337.15,
        339.15,
        341.15,
        343.15,
        345.15,
        347.15,
        349.15,
        351.15,
        353.15,
        355.15,
        357.15,
        359.15,
        361.15,
        363.15,
        365.15,
        367.15,
        369.15,
        371.15,
        373.15,
    ]
}

fn rho_data() -> Vec<f64> {
    vec![
        999.84,
        999.94,
        999.97,
        999.94,
        999.85,
        999.70,
        999.50,
        999.25,
        998.95,
        998.60,
        998.21,
        997.78,
        997.30,
        996.79,
        996.24,
        995.65,
        995.03,
        994.37,
        993.68,
        992.96,
        992.21,
        991.43,
        990.62,
        989.79,
        988.92,
        988.03,
        987.12,
        986.17,
        985.21,
        984.22,
        983.20,
        982.16,
        981.10,
        980.02,
        978.91,
        977.78,
        976.63,
        975.46,
        974.27,
        973.05,
        971.82,
        970.57,
        969.29,
        968.00,
        966.68,
        965.35,
        964.00,
        962.63,
        961.24,
        959.83,
        958.40,
    ]
}

fn sound_data() -> Vec<f64> {
    vec![
        1403.,
        1413.,
        1422.,
        1431.,
        1439.,
        1447.,
        1455.,
        1462.,
        1468.,
        1475.,
        1481.,
        1487.,
        1492.,
        1497.,
        1502.,
        1507.,
        1512.,
        1516.,
        1520.,
        1523.,
        1527.,
        1530.,
        1533.,
        1536.,
        1539.,
        1541.,
        1543.,
        1545.,
        1547.,
        1549.,
        1550.,
        1552.,
        1553.,
        1553.,
        1554.,
        1555.,
        1555.,
        1555.,
        1555.,
        1555.,
        1554.,
        1554.,
        1553.,
        1552.,
        1551.,
        1550.,
        1549.,
        1548.,
        1546.,
        1545.,
        1543.,
    ]
}

fn viscosity_data() -> Vec<f64> {
    vec![
        1.793e-3,
        1.675e-3,
        1.568e-3,
        1.472e-3,
        1.386e-3,
        1.307e-3,
        1.235e-3,
        1.169e-3,
        1.109e-3,
        1.053e-3,
        1.002e-3,
        9.549e-4,
        9.112e-4,
        8.706e-4,
        8.328e-4,
        7.976e-4,
        7.648e-4,
        7.341e-4,
        7.054e-4,
        6.784e-4,
        6.531e-4,
        6.293e-4,
        6.069e-4,
        5.858e-4,
        5.658e-4,
        5.469e-4,
        5.291e-4,
        5.122e-4,
        4.962e-4,
        4.809e-4,
        4.665e-4,
        4.527e-4,
        4.396e-4,
        4.272e-4,
        4.153e-4,
        4.040e-4,
        3.932e-4,
        3.828e-4,
        3.729e-4,
        3.635e-4,
        3.544e-4,
        3.457e-4,
        3.374e-4,
        3.295e-4,
        3.218e-4,
        3.145e-4,
        3.074e-4,
        3.006e-4,
        2.941e-4,
        2.878e-4,
        2.818e-4,
    ]
}