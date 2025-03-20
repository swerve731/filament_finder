use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialPropertiesTable {
    pub id: Uuid,
    pub material_id: Uuid,
    pub additive_ids: Option<Vec<Uuid>>,
    pub special_properties: Option<Vec<Uuid>>, // References special_properties table
    pub density_g_per_cm3: f64,
    pub shore_hardness_id: Option<Uuid>, // Fixed from shore_hardness_id
    pub finish_id: Uuid,
    pub created_at: NaiveDateTime,
    pub melting_point_celcius: f64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "shore_hardness_class", rename_all = "lowercase")] // Maps to PostgreSQL enum type
pub enum ShoreHardnessClass {
    A,
    D,
    C,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShoreHardnessTable {
    pub id: Uuid,
    pub material_id: Uuid,
    pub class: ShoreHardnessClass,
    pub hardness: u8, // 0-100 ie Class(hardness) = D(55)

    //optional range values for things like foaming materials
    pub min_hardness: Option<u8>,
    pub max_hardness: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialPropertyTable {
    pub id: Uuid,
    pub name: String, // foaming, fast printing,carbon fiber, wood fibers, metal fibers
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinishKindTable {
    pub id: Uuid,
    pub name: String, // matte, silk, gloss
    pub created_at: NaiveDateTime,
}
