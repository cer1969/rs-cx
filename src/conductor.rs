// Cristian Echeverría

use crate::category::Category;

//----------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Conductor<'a> {
	pub category: &'a Category,     // Category instance
	pub diameter: f64,          // Diameter [mm]
	pub area: f64,              // Cross section area [mm2]
	pub weight: f64,            // Weight per unit [kg/m]
	pub strength: f64,          // Rated strength [kg]
	pub r25: f64,               // Resistance at 25°C [Ohm/km]
	pub hcap: f64               // Heat capacity [kcal/(ft*°C)]
}