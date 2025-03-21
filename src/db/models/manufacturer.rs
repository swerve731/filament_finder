use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ManufacturerTable {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
}
