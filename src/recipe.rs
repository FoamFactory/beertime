use std::collections::HashMap;
use std::str::FromStr;

use crate::steps::Steps;
use crate::capacity::Capacity;
use crate::config::{FactoryConfig, RecipeConfig};
use crate::volume::Volume;

/**
 * A Recipe contains a mapping of `BatchSize`s to pairs of `(Volume, Steps)`, where
 * each pair contains a _yield_ volume and a set of `Steps` (also called Phases) that allow the
 * beer in question to be constructed.
 */
#[derive(Debug, PartialEq)]
pub struct Recipe {
    pub map: HashMap<Capacity, (Volume, Steps)>,
}

impl Recipe {
    pub fn blank() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn new(system_capacity: Capacity, batch_size: Volume, steps: Steps) -> Self {
        println!("Batchsize volume is: {}", system_capacity.volume());
        println!("Yield is: {}", batch_size);
        assert!(system_capacity.volume().ge(&batch_size));
        let mut recipe = Recipe::blank();
        recipe.map.insert(system_capacity, (batch_size, steps));

        recipe
    }

    pub fn store(&mut self, system_capacity: Capacity, batch_size: Volume, steps: Steps) {
        assert_eq!(self.get(&system_capacity), None);
        self.map.insert(system_capacity, (batch_size, steps));
    }

    pub fn get(&self, system_capacity: &Capacity) -> Option<&(Volume, Steps)> {
        self.map.get(system_capacity)
    }
}

impl From<(&FactoryConfig, &RecipeConfig)> for Recipe {
    fn from(config_pair: (&FactoryConfig, &RecipeConfig)) -> Self {
        let (factory_config_ref, recipe_config_ref) = config_pair;
        let system_capacity = match Capacity::from_str(&factory_config_ref.capacity) {
            Ok(c) => c,
            Err(_) => panic!("{} does not appear to be a valid system capacity", &factory_config_ref.capacity),
        };

        Recipe::new(system_capacity,
                    Volume::GallonUS(10 as f32),
                    Steps::new(None, None, None, None, None, None))
    }
}
