use std::collections::HashMap;

use chrono::Duration;
use z3::{ast, ast::Ast, Config, Context, Optimize, SatResult};

use crate::batchneed::BatchNeed;
use crate::beer::Beer;
use crate::equipment::Equipment;
use crate::equipment_group::EquipmentGroup;
use crate::step_group::StepGroup;
use crate::capacity::Capacity;
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

    pub fn calculate_batches(
        &self,
        wishlist: HashMap<&'static str, (&Beer, Volume)>,
    ) -> HashMap<usize, BatchNeed> {
        let mut batches_needed = HashMap::with_capacity(wishlist.len());
        let mut cfg = Config::new();
        cfg.set_proof_generation(false);
        cfg.set_model_generation(true);
        cfg.set_debug_ref_count(false);
        let ctx = Context::new(&cfg);
        let solver = Optimize::new(&ctx);

        let mut existing_systems = self
            .equipments
            .values()
            .map(|e| e.system.clone())
            .collect::<Vec<Capacity>>();
        existing_systems.sort();
        existing_systems.dedup();
        let mut systems = HashMap::new();
        for system in &existing_systems {
            match systems.get_mut(system) {
                None => {
                    if let Volume::Liter(liters) = system.volume().to_liter() {
                        // @FIXME: from i64 to real
                        systems.insert(system, ast::Int::from_i64(&ctx, liters as i64));
                    } else {
                        panic!("should not happen");
                    }
                }
                _ => { /* TODO: Take the smallest volume from the equipments in that system.
                                    Not sure if that would work, because these are probably kegs.
                     */
                }
            }
        }
        let total_batches = ast::Int::new_const(&ctx, "total batches");
        let mut all_beer_batches = Vec::with_capacity(wishlist.len());
        let mut all_beer_system_batches = HashMap::new();
        for (name, (_beer, volume)) in &wishlist {
            if let Volume::Liter(needed_liters) = volume.to_liter() {
                let beer_need = ast::Int::new_const(&ctx, format!("beer need {}", name));
                //@fixme: also here from i64 to real
                solver.assert(&beer_need._eq(&ast::Int::from_i64(&ctx, needed_liters as i64)));
                let beer_total = ast::Int::new_const(&ctx, format!("beer total {}", name));
                let mut beer_system_volumes = Vec::with_capacity(systems.len());
                for (system, r#_yield) in &systems {
                    let beer_system_batches = ast::Int::new_const(
                        &ctx,
                        format!("beer {} system {} batches", name, system.lookup()),
                    );
                    all_beer_batches.push(beer_system_batches.clone());
                    all_beer_system_batches.insert((name, system), beer_system_batches.clone());
                    let beer_system_volume =
                        ast::Int::new_const(&ctx, format!("beer systems volume {}", name));
                    solver.assert(&beer_system_batches.ge(&ast::Int::from_i64(&ctx, 0)));
                    let r#yield = systems.get(system).unwrap();
                    solver.assert(
                        &beer_system_volume
                            ._eq(&ast::Int::mul(&ctx, &[&beer_system_batches, r#yield])),
                    );
                    beer_system_volumes.push(beer_system_volume);
                }
                solver.assert(
                    &beer_total.le(&ast::Int::add(
                        &ctx,
                        beer_system_volumes
                            .iter()
                            .map(|x| x)
                            .collect::<Vec<&ast::Int>>()
                            .as_slice(),
                    )),
                );
                solver.assert(&beer_total.ge(&beer_need));
            }
        }

        solver.assert(
            &total_batches._eq(&ast::Int::add(
                &ctx,
                &all_beer_batches
                    .iter()
                    .map(|x| x)
                    .collect::<Vec<&ast::Int>>()
                    .as_slice(),
            )),
        );
        solver.minimize(&total_batches);
        match solver.check(&[]) {
            SatResult::Sat => {
                let model = solver.get_model();
                let mut id = 1;
                for ((name, system), batch_count_int) in all_beer_system_batches.iter() {
                    let batch_count = model.eval(batch_count_int).unwrap().as_i64().unwrap();
                    let beer = self.beers.get(&name.to_string()).unwrap();
                    for i in 0..batch_count {
                        let (r#yield, _steps) = beer.recipe.get(&system).unwrap();
                        let mut vol = r#yield.clone();
                        if i == batch_count - 1 {
                            if let Volume::Liter(want_liter) =
                                wishlist.get(*name).unwrap().1.to_liter()
                            {
                                if let Volume::Liter(yield_liter) = r#yield.to_liter() {
                                    vol = Volume::Liter(want_liter - yield_liter * i as f32)
                                }
                            }
                        }
                        if let Volume::Liter(liter) = vol.to_liter() {
                            if liter > 0.0 {
                                let batch =
                                    BatchNeed::new(id, beer, system.clone().clone().clone(), vol);
                                batches_needed.insert(batch.id, batch);
                                id += 1;
                            }
                        }
                    }
                }
            }
            _ => {
                //todo better error handling
                panic!("Can't calculate batches for these quanties and systems/equipment")
            }
        }

        batches_needed
    }

    pub fn calculate_bottleneck_step(
        &self,
        batches_needed: &HashMap<usize, BatchNeed>,
    ) -> Vec<(Capacity, StepGroup, Duration)> {
        let mut temp: HashMap<(Capacity, StepGroup), Duration> = HashMap::new();
        for batch in batches_needed.values() {
            let (_volume, steps) = batch.beer.recipe.get(&batch.system).expect(&format!(
                "Beer {} should have a recipe for system {:?}",
                batch.beer.name, batch.system
            ));
            for (step_group, interval) in steps.iter() {
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
        let mut temp_vec: Vec<(&(Capacity, StepGroup), &Duration)> = temp.iter().collect();
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
        acc_batches: &[(Capacity, StepGroup, Duration)],
    ) -> Vec<(Capacity, EquipmentGroup, Duration)> {
        // @TODO merge this with calculate_bottleneck_step to save time on building hashmaps and sorting them into vectors
        let mut temp: HashMap<(Capacity, EquipmentGroup), Duration> = HashMap::new();
        for (system, step_group, duration) in acc_batches {
            let equipment_group = step_group.equipment_group();
            match temp.get_mut(&(system.clone(), equipment_group.clone())) {
                None => {
                    temp.insert((system.clone(), equipment_group.clone()), duration.clone());
                }
                Some(dur) => *dur = *dur + duration.clone(),
            };
        }
        // sort, descending on usage
        let mut temp_vec: Vec<(&(Capacity, EquipmentGroup), &Duration)> = temp.iter().collect();
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
        system: &Capacity,
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
        acc_equipment: &[(Capacity, EquipmentGroup, Duration)],
    ) -> Vec<(Capacity, EquipmentGroup, Duration)> {
        let mut temp = HashMap::with_capacity(acc_equipment.len());
        for (system, equipment_group, duration) in acc_equipment {
            let suited = self.list_suited_equipment(system, equipment_group);
            assert!(suited.len() > 0);
            let avg_duration = *duration / (suited.len() as i32);
            temp.insert((system.clone(), equipment_group.clone()), avg_duration);
        }

        let mut temp_vec: Vec<(&(Capacity, EquipmentGroup), &Duration)> = temp.iter().collect();
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
    use crate::capacity;
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
            capacity::mock::bbl5(),
            equipment_group::mock::mash_tun(),
            volume::mock::gallon_us(),
        );

        factory
            .equipments
            .insert(equipment_2.name.to_string(), equipment_2.clone());
        assert_eq!(factory.equipments.len(), 2);
        assert_eq!(
            factory.list_suited_equipment(&Capacity::G10, &EquipmentGroup::CO2Tank),
            Vec::<&Equipment>::new()
        );
        assert_eq!(
            factory.list_suited_equipment(&Capacity::BBL5, &EquipmentGroup::CO2Tank),
            Vec::<&Equipment>::new()
        );
        let suited = factory.list_suited_equipment(&Capacity::BBL5, &EquipmentGroup::MashTun);
        assert!(
            (suited == vec![&equipment_1, &equipment_2])
                || (suited == vec![&equipment_2, &equipment_1])
        );
    }
}
