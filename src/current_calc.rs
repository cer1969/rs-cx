// Cristian Echeverría

use crate::{
    k::{FormulaKind, TC_MIN, TC_MAX},
    conductor::Conductor
};

//----------------------------------------------------------------------------------------

pub struct CurrentCalc<'a> {
    pub conductor: &'a Conductor<'a>,       // Conductor instance
    pub altitude: f64,                      // Altitude [m] = 300.0
    pub air_velocity: f64,                  // Velocity of air stream [ft/seg] =   2.0
    pub sun_effect: f64,                    // Sun effect factor (0 to 1) = 1.0
    pub emissivity: f64,                    // Emissivity (0 to 1) = 0.5
    pub formula: FormulaKind,               // Define formula for current calculation = CF_IEEE
    pub delta_temp: f64                     // Temperature difference to determine equality [°C] = 0.0001
}

impl CurrentCalc<'_> {
    
    pub fn resistance(&self, tc: f64) -> Result<f64, String> {
        if tc < TC_MIN {
            return Err("CurrentCalc.Resistance: tc < TC_MIN".to_string());
        }
        if tc > TC_MAX {
            return Err("CurrentCalc.Resistance: tc > TC_MAX".to_string());
        }
        Ok(
            self.conductor.r25 * (1.0 + self.conductor.category.alpha*(tc-25.0))
        )
    }
}

//----------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate as cx;

    #[test]
    fn test_resistance() {
        
        let c1 = cx::Conductor {
            category: &cx::CC_AAAC,
            diameter: 25.17,
            area: 375.4,
            weight: 1.035,
            strength: 11625.0,
            r25: 0.08936,
            hcap: 0.05274
        };
        let cc1 = cx::CurrentCalc {
            conductor: &c1,
            altitude: 300.0,
            air_velocity: 2.0, 
            sun_effect: 1.0,
            emissivity: 0.5,
            formula: cx::FormulaKind::IEEE,
            delta_temp: 0.0001
        };
        let tc = 80.0;
        let r = cc1.resistance(tc).expect("TC incorrecto");
        let df = (r - 0.10607032).abs();

        assert!(df < 0.0001);
    }
}
