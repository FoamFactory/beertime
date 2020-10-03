use std::collections::HashMap;

use chrono::Duration;

use crate::batchneed::BatchNeed;
use crate::beer::Beer;
use crate::equipment::Equipment;
use crate::equipment_group::EquipmentGroup;
use crate::step_group::StepGroup;
use crate::steps::StepIterator;
use crate::system::System;
use crate::volume::Volume;

#[derive(Debug, PartialEq)]
pub struct Factory {
    pub name: String,
    pub equipments: HashMap<String, Equipment>,
    pub beers: HashMap<String, Beer>,
}

impl Factory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            equipments: HashMap::new(),
            beers: HashMap::new(),
        }
    }

    pub fn calculate_batches(&self, wishlist: Vec<(&'static str, Volume)>) -> Vec<BatchNeed> {
        let mut batches_needed = Vec::with_capacity(wishlist.len());
        for (beer_name, quantity) in wishlist {
            let beer = self
                .beers
                .get(beer_name)
                .expect(&format!("Unknow beer: {}", beer_name));
            //FIXME: we now take the first recipy that is registered,
            //       this should be done by the solver
            let (system, (r#yield, steps)) = beer
                .recipy
                .map
                .iter()
                .nth(0)
                .expect(&format!("Unknown recipy for beer: {}", beer_name));
            let counts = quantity.full_batches(r#yield);
            for _i in 0..counts {
                let batch = BatchNeed::new(beer, system, r#yield.clone());
                batches_needed.push(batch);
            }
        }

        batches_needed
    }

    pub fn calculate_bottleneck_step(
        &self,
        batches: Vec<BatchNeed>,
    ) -> Vec<(System, StepGroup, Duration)> {
        let mut temp: HashMap<(System, StepGroup), Duration> = HashMap::new();
        for batch in batches {
            let (_volume, steps) = batch.beer.recipy.get(batch.system).expect(&format!(
                "Beer {} should have a recipy for system {:?}",
                batch.beer.name, batch.system
            ));
            for (step_group, interval) in StepIterator::new(steps) {
                match temp.get_mut(&(batch.system.clone(), step_group.clone())) {
                    None => {
                        temp.insert(
                            (batch.system.clone(), step_group.clone()),
                            interval.duration(),
                        );
                    }
                    Some(dur) => {
                        *dur = *dur + interval.duration();
                    }
                }
            }
        }
        //sort, descending on usage
        let mut temp_vec: Vec<(&(System, StepGroup), &Duration)> = temp.iter().collect();
        temp_vec.sort_by(|a, b| b.1.cmp(a.1));
        temp_vec
            .iter()
            .map(|((system, stepgroup), duration)| {
                (system.clone(), stepgroup.clone(), *duration.clone())
            })
            .collect()
    }

    pub fn calculate_bottleneck_equipment(
        &self,
        acc_batches: Vec<(System, StepGroup, Duration)>,
    ) -> Vec<(System, EquipmentGroup, Duration)> {
        unimplemented!()
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use crate::beer;
    use crate::equipment;

    pub fn factory() -> Factory {
        let mut factory = Factory::new("loonslanding");

        let equipment = equipment::mock::equipment();
        factory
            .equipments
            .insert(equipment.name.to_string(), equipment);

        let beer = beer::mock::beer();
        factory.beers.insert(beer.name.to_string(), beer);

        factory
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_new() {
        let factory = mock::factory();
        assert_eq!(&factory.name, "loonslanding");
        assert_eq!(factory.equipments.len(), 1);
        assert_eq!(factory.beers.len(), 1);
    }
}
