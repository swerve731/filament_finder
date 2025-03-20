use super::*;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MaterialTableModel {
    pub id: Uuid,
    pub name: String,
    pub manufacturer_id: Uuid,
    pub description: String,
    pub printer_kind_id: Uuid,
    pub material_family_id: Uuid,
    pub material_properties_id: Uuid, // Reference to material_properties table
    pub created_at: NaiveDateTime,
    pub last_updatated: NaiveDateTime,
}
