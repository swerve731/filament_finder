use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    pub id: Uuid,
    pub name: String,
    pub manufacturer: Manufacturer, // Replace with Manufacturer struct if available
    pub description: String,
    pub unit_size_in_grams: usize,
    pub cost_per_unit_usd: f64,
    pub density_g_per_cm3: f64,
    pub shore_hardness: Option<ShoreHardness>,
    pub printer_kind: PrinterKind,
    pub material_family: MaterialFamily,
    pub special_properties: MaterialProperties,
}

/// Common attributes for all materials
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MaterialTableModel {
    pub id: Uuid,
    pub name: String,
    pub manufacturer_id: Uuid,
    pub description: String,
    pub unit_size_in_grams: usize,
    pub cost_per_unit_usd: f64,
    pub density_g_per_cm3: f64,
    pub printer_kind_id: Uuid,
    pub material_family_id: Uuid,
    pub special_properties: MaterialProperties,
}

impl From<Material> for MaterialTableModel {
    fn from(material: Material) -> Self {
        MaterialTableModel {
            id: material.id,
            name: material.name,
            manufacturer_id: material.manufacturer.id, // Placeholder - Assuming Manufacturer struct will have an ID
            description: material.description,
            unit_size_in_grams: material.unit_size_in_grams,
            cost_per_unit_usd: material.cost_per_unit_usd,
            density_g_per_cm3: material.density_g_per_cm3,
            printer_kind_id: material.printer_kind.id,
            material_family_id: material.material_family.id,
            special_properties: material.special_properties,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrinterKind {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialProperties {
    pub id: Uuid,
    pub material_id: Uuid,
    pub additive_ids: Option<Vec<Uuid>>,
    pub special_properties: Option<Vec<Uuid>>,
    pub shore_hardness_id: Option<Uuid>,
    pub finish_id: Uuid, // Add other common properties as needed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialProperty {
    pub id: Uuid,
    pub name: String, //foaming
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ShoreHardnessClass {
    A,
    D,
}
/// Enum for shore hardness scales
#[derive(Debug, Serialize, Deserialize)]
pub struct ShoreHardness {
    pub id: Uuid,
    pub class: ShoreHardnessClass,
    pub hardness: u8, // Shore A scale (0-100)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialFamily {
    pub id: Uuid,
    pub name: String,
    pub printer_kind: PrinterKind,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinishKind {
    pub id: Uuid,
    pub name: String, //matte, silk, etc..
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_creation() {
        // Arrange
    }
}
