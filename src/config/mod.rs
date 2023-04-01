mod equipment_config;
pub use equipment_config::EquipmentConfig;
mod factory_config;
pub use factory_config::FactoryConfig;
mod recipe_config;
pub use recipe_config::RecipeConfig;

use crate::capacity::Capacity;
use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::beer::Beer;
    use crate::equipment::Equipment;
    use crate::volume::Volume;

    fn load_configuration_from_json() -> Config {
        let mut test_config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_config_path.push("contrib/LoonsLanding.json");
        let path_str = test_config_path.to_str().unwrap();
        Config::read_config(path_str.to_string()).unwrap()
    }

    #[test]
    fn it_can_load_a_configuration_from_json() {
        let config = load_configuration_from_json();
        assert_eq!("Loons Landing Brewery", config.factory.name);
        assert_eq!(8, config.factory.equipment.len());

        // Check to make sure that we have at least one fermentor named "Big Bertha"
        for equip_config in &config.factory.equipment {
            if equip_config.name == "Big Bertha" {
                let equipment = Equipment::from(equip_config);
                assert_eq!(equipment.name, "Big Bertha");
                assert_eq!(equipment.capacity, Capacity::G14);
                assert!(equipment.can_hold(&Volume::GallonUS(12.0)));
            }
        }

        // Check to make sure there is at least one recipe called "Damned Squirrel, Mk. II"
        for recipe_config in &config.factory.recipes {
            if recipe_config.name == "Damned Squirrel Mk. II" {
                let _beer = Beer::from((&config.factory, recipe_config));
            }
        }
    }

    #[test]
    fn max_mash_tun_capacity_should_be_15g() {
        let config = load_configuration_from_json();
        assert_eq!(
            config.factory.max_mash_tun_capacity().unwrap(),
            Capacity::G15
        );
    }
}
