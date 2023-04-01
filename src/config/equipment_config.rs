#[derive(serde::Deserialize, Debug)]
pub struct EquipmentConfig {
    pub id: u32,
    pub name: String,
    pub equipment_type: String,
    pub capacity: String,
}
