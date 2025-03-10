use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Common attributes for all materials
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Material {
    pub id: Uuid,
    pub name: String,
    pub manufacturer: String,
    pub description: String,
    pub unit_size_in_grams: usize,
    pub cost_per_unit_usd: f64,
    pub density_g_per_cm3: f64,
    pub shore_hardness: Option<ShoreHardness>,
    pub printer_kind: PrinterKind,
    pub material_family: MaterialFamily,
    pub special_properties: MaterialProperties,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "printer_kind", rename_all = "lowercase")]
pub enum PrinterKind {
    Fdm,
    Sla,
}

/// Enum for shore hardness scales
todo!(" make it so this can be added to database using either to string or display or smthn")
#[derive(Debug, Serialize, Deserialize)]
pub enum ShoreHardness {
    A(u8), // Shore A scale (0-100)
    D(u8), // Shore D scale (0-100)
}

/// Additional properties applicable to all materials
#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialProperties {
    pub additives: Vec<Additive>,
    pub special_properties: Vec<SpecialProperty>,
    // Add other common properties as needed
}

/// Material families with associated specific properties
#[derive(Debug, Serialize, Deserialize)]
pub enum MaterialFamily {
    Pla,
    Abs,
    Petg,
    Nylon,
    Tpu,
    Resin, // Add other material families as needed
}

/// Enum for possible additives in materials
#[derive(Debug, Serialize, Deserialize)]
pub enum Additive {
    CarbonFiber,
    GlassFiber,
    WoodFiber,
    MetalPowder,
    CeramicPowder,
    PhosphorescentPigment,
    ThermoplasticElastomer,
}

pub enum Finish {
    Silk,
    Matte,
    GlowInTheDark,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SpecialProperty {
    Foaming,
    HighSpeedPrinting,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_creation() {
        // Arrange
        let _material = Material {
            id: Uuid::new_v4(),
            name: "VarioShore TPU".to_string(),
            manufacturer: "ColorFabb".to_string(),
            description: "Foaming TPU with variable density.".to_string(),
            unit_size_in_grams: 1000,
            cost_per_unit_usd: 56.67,
            density_g_per_cm3: 1.25,
            shore_hardness: Some(ShoreHardness::A(92)),
            printer_kind: PrinterKind::Fdm,
            material_family: MaterialFamily::Tpu,
            special_properties: MaterialProperties {
                additives: vec![Additive::CarbonFiber],
                special_properties: vec![SpecialProperty::Foaming],
            },
        };
    }
}
