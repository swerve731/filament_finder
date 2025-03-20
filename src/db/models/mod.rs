pub mod manufacturer;
pub mod material;
pub mod material_family;
pub mod material_properties;
pub mod printers;
pub use chrono::NaiveDateTime;

pub use manufacturer::*;
pub use material::*;
pub use material_family::*;
pub use printers::*;
pub use serde::{Deserialize, Serialize};
pub use sqlx::FromRow;
pub use uuid::Uuid;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Material {
//     pub id: Uuid,
//     pub name: String,
//     pub manufacturer: Manufacturer,
//     pub description: String,
//     pub unit_size_in_grams: usize,
//     pub cost_per_unit_usd: f64,
//     pub density_g_per_cm3: f64,
//     pub shore_hardness: Option<ShoreHardnessClass>,
//     pub printer_kind: PrinterKind,
//     pub material_family: MaterialFamily,
//     pub special_properties: MaterialProperties,
//     pub created_at: NaiveDateTime,
// }

// #[derive(Debug, Serialize, Deserialize, FromRow)]
// pub struct MaterialTableModel {
//     pub id: Uuid,
//     pub name: String,
//     pub manufacturer_id: Uuid,
//     pub description: String,
//     pub unit_data: Vec<Uuid>,
//     pub printer_kind_id: Uuid,
//     pub material_family_id: Uuid,
//     pub special_properties: MaterialProperties,
//     pub created_at: NaiveDateTime,
// }

// ie. one spool is 1kg and cost 25$
// if the company sells 2 sizes with different prices this would work too

//ie. PLA, TPU, PETG

// Define the enum with SQLx support

// #[derive(Debug, Serialize, Deserialize)]
// pub struct MaterialProperties {
//     pub id: Uuid,
//     pub material_id: Uuid,
//     pub additive_ids: Option<Vec<Uuid>>,
//     pub special_properties: Option<Vec<Uuid>>,
//     pub density_g_per_cm3: f64,
//     pub shore_hardness_id: Option<Uuid>,
//     pub finish_id: Uuid,
//     pub created_at: NaiveDateTime,
//     pub melting_point_celcius: f64,
// }

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_material_creation() {
        // Arrange
    }
}
