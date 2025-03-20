use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialFamilyTable {
    pub id: Uuid,
    pub name: String,
    pub printer_kind_id: Uuid,
    pub created_at: NaiveDateTime,
}
