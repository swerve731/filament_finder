use super::*;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MaterialUnitDataTable {
    pub id: Uuid,
    pub material_id: Uuid,
    pub unit_size_in_grams: usize,
    pub cost_per_unit_usd: f64,
    // ie. fdm printer filament can come in 1.75mm, 2.25mm etc...
    pub filament_diameter: Option<f64>,
}
