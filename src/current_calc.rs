// Cristian Echeverría

use check::Check;
use crate::{
    k::{F64Result, FormulaKind, TC_MIN, TC_MAX},
    conductor::Conductor
};

//----------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct CurrentCalc<'a> {
    pub conductor: &'a Conductor<'a>,       // Conductor instance
    pub altitude: f64,                      // Altitude [m] = 300.0
    pub air_velocity: f64,                  // Velocity of air stream [ft/seg] =   2.0
    pub sun_effect: f64,                    // Sun effect factor (0 to 1) = 1.0
    pub emissivity: f64,                    // Emissivity (0 to 1) = 0.5
    pub formula: FormulaKind,               // Define formula for current calculation = CF_IEEE
    pub delta_temp: f64                     // Temperature difference to determine equality [°C] = 0.0001
}

impl<'a> CurrentCalc<'a> {

    pub fn new_base(conductor: &'a Conductor<'a>) -> Self {
        Self {
            conductor,
            altitude: 300.0,
            air_velocity: 2.0,
            sun_effect: 1.0,
            emissivity: 0.5,
            formula: FormulaKind::IEEE,
            delta_temp: 0.0001
        }
    }

    pub fn validate(&self) -> Result<bool, String> {
        Check{n: self.conductor.diameter}.gt(0.0, "CurrentCalc.validate: conductor.diameter <= 0")?;
        Check{n: self.conductor.r25}.gt(0.0, "CurrentCalc.validate: conductor.r25 <= 0")?;
        Check{n: self.conductor.category.alpha}.gt(0.0, "CurrentCalc.validate: conductor.category.alpha <= 0")?;
        Check{n: self.conductor.category.alpha}.lt(1.0, "CurrentCalc.validate: conductor.category.alpha >= 1")?;
        Check{n: self.altitude}.ge(0.0, "CurrentCalc.validate: altitude < 0")?;
        Check{n: self.air_velocity}.ge(0.0, "CurrentCalc.validate: air_velocity < 0")?;
        Check{n: self.sun_effect}.ge(0.0, "CurrentCalc.validate: sun_effect < 0")?;
        Check{n: self.sun_effect}.le(1.0, "CurrentCalc.validate: sun_effect > 1")?;
        Check{n: self.emissivity}.ge(0.0, "CurrentCalc.validate: emissivity < 0")?;
        Check{n: self.emissivity}.le(1.0, "CurrentCalc.validate: emissivity > 1")?;
        Check{n: self.delta_temp}.gt(0.0, "CurrentCalc.validate: delta_temp <= 0")?;
        
        Ok(true)
    }

    pub fn resistance(&self, tc: f64) -> F64Result {
        Check{n: tc}.ge(TC_MIN, "CurrentCalc.resistance: tc < TC_MIN")?;
        Check{n: tc}.le(TC_MAX, "CurrentCalc.resistance: tc > TC_MAX")?;
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
        // let cc1 = cx::CurrentCalc {
        //     conductor: &c1,
        //     altitude: 300.0,
        //     air_velocity: 2.0, 
        //     sun_effect: 1.0,
        //     emissivity: 0.5,
        //     formula: cx::FormulaKind::IEEE,
        //     delta_temp: 0.0001
        // };
        let cc1 = cx::CurrentCalc::new_base(&c1);
        cc1.validate().expect("Argumentos incorrectos");
        let tc = 80.0;
        let r = cc1.resistance(tc).expect("TC incorrecto");
        let df = (r - 0.10607032).abs();

        assert!(df < 0.0001);
    }
}
