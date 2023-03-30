use std::path::PathBuf;
use std::str::FromStr;

use crate::capacity::Capacity;
use crate::equipment::Equipment;
use crate::equipment_group::EquipmentGroup;
use crate::factory::Factory;
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
    pub equipment: Vec<EquipmentConfig>,
    pub recipes: Vec<RecipeConfig>,
    pub capacity: String,
}

impl FactoryConfig {
    pub fn max_mash_tun_capacity(&self) -> Option<Capacity> {
        let mut mash_tuns: Vec<Equipment> = vec![];
        for equipment_config in &self.equipment {
            let eq_group = match EquipmentGroup::from_str(&equipment_config.equipment_type) {
                Ok(s) => s,
                Err(_) => panic!(
                    "{} does not appear to be a valid equipment group",
                    equipment_config.equipment_type
                ),
            };

            if eq_group == EquipmentGroup::MashTun {
                println!(
                    "Saw mash tun equipment group {} with capacity config: {}",
                    equipment_config.name, equipment_config.capacity
                );
                mash_tuns.push(Equipment::from(equipment_config));
            }
        }

        let max_cap = mash_tuns
            .iter()
            .map(|eq| {
                println!("Saw mash tun {} with capacity: {}", eq.name, eq.capacity);
                eq.capacity.clone()
            })
            .min();

        max_cap
        // *mash_tuns.iter().min().unwrap();
    }
    // pub fn system_capacity(&self) -> Capacity {
    //     // We need a mash tun, lauter tun, kettle, and at least one fermentor for the entire process.
    //     let mut mash_tun_size: Option<Capacity> = None;
    //     let mut lauter_tun_size: Option<Capacity> = None;
    //     let mut kettle_size: Option<Capacity> = None;
    //     let mut fermentor_size: Option<Capacity> = None;
    //
    //     for equipment_config in &self.equipment {
    //         let eq_group = match EquipmentGroup::from_str(&equipment_config.equipment_type) {
    //             Ok(s) => s,
    //             Err(_) => panic!("{} does not appear to be a valid equipment group", &equipment_config.equipment_type)
    //         };
    //
    //         if eq_group == EquipmentGroup::MashTun {
    //             if mash_tun_size.is_none() || mash_tun_size.unwrap() > Capacity::from_str(&equipment_config.capacity).unwrap() {
    //                 mash_tun_size = Some(Capacity::from_str(&equipment_config.capacity).unwrap())
    //             }
    //         } else if eq_group == EquipmentGroup::LauterTun {
    //             if lauter_tun_size.is_none() || lauter_tun_size.unwrap() > Capacity::from_str(&equipment_config.capacity).unwrap() {
    //                 lauter_tun_size = Some(Capacity::from_str(&equipment_config.capacity).unwrap())
    //             }
    //         } else if eq_group == EquipmentGroup::Kettle {
    //             if kettle_size.is_none() || kettle_size.unwrap() > Capacity::from_str(&equipment_config.capacity).unwrap() {
    //                 kettle_size = Some(Capacity::from_str(&equipment_config.capacity).unwrap())
    //             }
    //         } else if eq_group == EquipmentGroup::Fermentor {
    //             if fermentor_size.is_none() || fermentor_size.unwrap() > Capacity::from_str(&equipment_config.capacity).unwrap() {
    //                 fermentor_size = Some(Capacity::from_str(&equipment_config.capacity).unwrap())
    //             }
    //         }
    //     }
    //
    //     // Find the minimum capacity
    //     let mut system_capacity: Capacity = Capacity::UNKNOWN;
    //     if mash_tun_size.is_some() && lauter_tun_size.is_some() && kettle_size.is_some() && fermentor_size.is_some() {
    //         let mt_cap = mash_tun_size.unwrap();
    //         let lt_cap = lauter_tun_size.unwrap();
    //         let bk_cap = kettle_size.unwrap();
    //         let ferm_cap = fermentor_size.unwrap();
    //         system_capacity = mt_cap.min(lt_cap).min(bk_cap).min(ferm_cap);
    //     } else {
    //         panic!("There does not appear to be enough equipment to compute a minimum system size for factory '{}'", self.name);
    //     }
    //
    //     system_capacity
    // }
}

#[derive(serde::Deserialize)]
pub struct EquipmentConfig {
    pub id: u32,
    pub name: String,
    pub equipment_type: String,
    pub capacity: String,
}

#[derive(serde::Deserialize)]
pub struct RecipeConfig {
    pub name: String,
    pub batch_size: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::beer::Beer;
    use crate::equipment::Equipment;
    use crate::equipment_group::EquipmentGroup;
    use crate::recipe::Recipe;
    use crate::volume::Volume;
    use crate::volume::Volume::GallonUS;

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
                let beer = Beer::from((&config.factory, recipe_config));
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
