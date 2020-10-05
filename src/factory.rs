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
            let (system, (r#yield, _steps)) = beer
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
            .map(|((system, step_group), duration)| {
                (system.clone(), step_group.clone(), *duration.clone())
            })
            .collect()
    }

    pub fn calculate_bottleneck_equipment(
        &self,
        acc_batches: Vec<(System, StepGroup, Duration)>,
    ) -> Vec<(System, EquipmentGroup, Duration)> {
        // @TODO merge this with calculate_bottleneck_step to save time on building hashmaps and sorting them into vectors
        let mut temp: HashMap<(System, EquipmentGroup), Duration> = HashMap::new();
        for (system, step_group, duration) in &acc_batches {
            let equipment_group = step_group.equipment_group();
            match temp.get_mut(&(system.clone(), equipment_group.clone())) {
                None => {
                    temp.insert((system.clone(), equipment_group.clone()), duration.clone());
                }
                Some(dur) => *dur = *dur + duration.clone(),
            };
        }
        // sort, descending on usage
        let mut temp_vec: Vec<(&(System, EquipmentGroup), &Duration)> = temp.iter().collect();
        temp_vec.sort_by(|a, b| b.1.cmp(&a.1));
        temp_vec
            .iter()
            .map(|((system, equipment_group), duration)| {
                (system.clone(), equipment_group.clone(), *duration.clone())
            })
            .collect()
    }

    pub fn list_suited_equipment(
        &self,
        system: &System,
        equipment_group: &EquipmentGroup,
    ) -> Vec<&Equipment> {
        let mut out = Vec::new();
        for equipment in self.equipments.values() {
            if &equipment.system == system && &equipment.equipment_group == equipment_group {
                out.push(equipment)
            }
        }

        out
    }

    pub fn calculate_bottleneck(
        &self,
        acc_equipment: Vec<(System, EquipmentGroup, Duration)>,
    ) -> Vec<(System, EquipmentGroup, Duration)> {
        let mut temp = HashMap::with_capacity(acc_equipment.len());
        for (system, equipment_group, duration) in &acc_equipment {
            let suited = self.list_suited_equipment(system, equipment_group);
            assert!(suited.len() > 0);
            let avg_duration = *duration / (suited.len() as i32);
            temp.insert((system.clone(), equipment_group.clone()), avg_duration);
        }

        let mut temp_vec: Vec<(&(System, EquipmentGroup), &Duration)> = temp.iter().collect();
        temp_vec.sort_by(|a, b| b.1.cmp(&a.1));

        temp_vec
            .iter()
            .map(|((system, equipment_group), duration)| {
                (system.clone(), equipment_group.clone(), *duration.clone())
            })
            .collect()
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
    use crate::equipment;
    use crate::equipment_group;
    use crate::system;
    use crate::volume;

    #[test]
    fn test_factory_new() {
        let factory = mock::factory();
        assert_eq!(&factory.name, "loonslanding");
        assert_eq!(factory.equipments.len(), 1);
        assert_eq!(factory.beers.len(), 1);
    }

    #[test]
    fn test_factory_list_suited_equipment() {
        let mut factory = mock::factory();
        let equipment_1 = equipment::mock::equipment();
        let equipment_2 = Equipment::new(
            "Foobar 2001".to_string(),
            system::mock::bbl5(),
            equipment_group::mock::mash_tun(),
            volume::mock::gallon_us(),
        );

        factory
            .equipments
            .insert(equipment_2.name.to_string(), equipment_2.clone());
        assert_eq!(factory.equipments.len(), 2);
        assert_eq!(
            factory.list_suited_equipment(&System::G10, &EquipmentGroup::CO2Tank),
            Vec::<&Equipment>::new()
        );
        assert_eq!(
            factory.list_suited_equipment(&System::BBL5, &EquipmentGroup::CO2Tank),
            Vec::<&Equipment>::new()
        );
        let suited = factory.list_suited_equipment(&System::BBL5, &EquipmentGroup::MashTun);
        assert!(
            (suited == vec![&equipment_1, &equipment_2])
                || (suited == vec![&equipment_2, &equipment_1])
        );
    }
}
