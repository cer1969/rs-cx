// Cristian Echeverría

#[derive(Debug)]
pub struct Category {
	pub modelas: f64,       // Modulus of elasticity [kg/mm2]
	pub coefexp: f64,       // Coefficient of Thermal Expansion [1/°C]
	pub creep: f64,         // Creep °C
	pub alpha: f64          // Temperature coefficient of resistance [1/°C]
}

//----------------------------------------------------------------------------------------

pub const CC_CU: Category = Category{
    modelas: 12000.0,
    coefexp: 0.0000169,
    creep: 0.0, 
    alpha: 0.00374
};
pub const CC_AAAC: Category = Category{
    modelas: 6450.0,
    coefexp: 0.0000230,
    creep: 20.0, 
    alpha: 0.00340
};
pub const CC_ACAR: Category = Category{
    modelas: 6450.0,
    coefexp: 0.0000250,
    creep: 20.0, 
    alpha: 0.00385
};
pub const CC_ACSR: Category = Category{
    modelas: 8000.0,
    coefexp: 0.0000191,
    creep: 20.0, 
    alpha: 0.00395
};
pub const CC_AAC: Category = Category{
    modelas: 5600.0,
    coefexp: 0.0000230,
    creep: 20.0, 
    alpha: 0.00395
};
pub const CC_CUWELD: Category = Category{
    modelas: 16200.0,
    coefexp: 0.0000130,
    creep: 0.0, 
    alpha: 0.00380
};
pub const CC_AASC: Category = CC_AAAC;
pub const CC_ALL: Category = CC_AAC;