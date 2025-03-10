#[derive(Debug, Serialize, Deserialize)]
pub struct Manufacturer {
    pub id: Uuid,
    pub name: String,
}
