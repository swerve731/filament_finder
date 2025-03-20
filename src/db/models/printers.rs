use super::*;

// ie. FDM, SLS, SLA
#[derive(Debug, Serialize, Deserialize)]
pub struct PrinterKindTable {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
}
