use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct Config {
    pub factory: FactoryConfig,
}

impl Config {
    pub fn read_config(file_path: String) -> std::io::Result<Config> {
        let content = std::fs::read_to_string(file_path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }
}

#[derive(serde::Deserialize)]
pub struct FactoryConfig {
    pub name: String,
    pub equipment: Vec<EquipmentConfig>
}

#[derive(serde::Deserialize)]
pub struct EquipmentConfig {
    pub id: u32,
    pub name: String,
    pub equipment_type: String,
    pub capacity: String,
}

#[cfg(test)]
mod tests {
    use crate::equipment::Equipment;
    use crate::equipment_group::EquipmentGroup;
    use crate::volume::Volume;
    use super::*;

    #[test]
    fn it_can_load_a_configuration_from_json() {
        let mut test_config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_config_path.push("contrib/LoonsLanding.json");
        let path_str = test_config_path.to_str().unwrap();
        let config = Config::read_config(path_str.to_string()).unwrap();
        assert_eq!("Loons Landing Brewery", config.factory.name);
        assert_eq!(9, config.factory.equipment.len());

        // Check to make sure that we have at least one fermentor named "Big Bertha"
        for equip_config in config.factory.equipment {
            if equip_config.name == "Big Bertha" {
                let equipment = Equipment::from(equip_config);
                assert_eq!(equipment.name, "Big Bertha");
                assert_eq!(equipment.volume, Volume::GallonUS(14.0));
                assert!(equipment.can_hold(&Volume::GallonUS(12.0)));
            }
        }
    }
}
