// Cristian Echeverría


pub type F64Result = Result<f64, String>;

#[derive(Debug)]
pub enum FormulaKind {
    CLASSIC,                    // Formula original con saltos. Debería eliminarla??
    IEEE                        // Formula IEEE
}

// Minimum & Maximum values for ambient temperature [°C].
// World lowest -82.2°C Vostok Antartica 21/07/1983
// World highest 58.2°C Libia 13/09/1922
pub const TA_MIN: f64 = -90.0;
pub const TA_MAX: f64 = 90.0;

// Minimum & Maximum values for conductor temperature [°C].
pub const TC_MIN: f64 = -90.0;
pub const TC_MAX: f64 = 2000.0;

// Maximum conductor tension [kg]
pub const TENSION_MAX: f64 = 50000.0;