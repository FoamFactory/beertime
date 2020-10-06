use std::collections::HashMap;

use chrono::{DateTime, Utc};
use z3::{ast, ast::Ast, Config, Context, SatResult, Solver, Sort};

use crate::batchneed::BatchNeed;
use crate::factory::Factory;
use crate::steps::StepIterator;

pub struct Plan {
    _dummy: u8,
}

impl Plan {
    pub fn new() -> Self {
        Self { _dummy: 4 }
    }

    pub fn do_magic(
        &self,
        factory: &Factory,
        batches_needed: &[BatchNeed],
        earliest_start: DateTime<Utc>,
    ) {
        let mut cfg = Config::new();
        cfg.set_proof_generation(false);
        cfg.set_model_generation(true);
        cfg.set_debug_ref_count(false);
        cfg.set_timeout_msec(5_000);
        let ctx = Context::new(&cfg);
        let solver = Solver::new(&ctx);

        let mut machines = HashMap::new();
        let mut i = 0;
        for equipment in factory.equipments.values() {
            let suited =
                factory.list_suited_equipment(&equipment.system, &equipment.equipment_group);
            let one_of_these = ast::Set::new_const(
                &ctx,
                format!(
                    "machines capable for {:?} {:?}",
                    equipment.system, equipment.equipment_group
                ),
                &Sort::int(&ctx),
            );
            for seq in &suited {
                let suit = ast::Int::new_const(&ctx, format!("Machine {}", seq.name));
                solver.assert(&suit._eq(&ast::Int::new_const(&ctx, i)));
                i += 1;
            }
            machines.insert(
                (equipment.system.clone(), equipment.equipment_group.clone()),
                one_of_these,
            );
        }
        let start_horizon = ast::Int::from_i64(&ctx, earliest_start.timestamp());
        for (i, batch) in batches_needed.iter().enumerate() {
            if let Some((max_volume, steps)) = batch.beer.recipy.get(batch.system) {
                assert!(batch.volume.ge(max_volume));
                let mut prev = None;
                let step_iter = StepIterator::new(steps);
                /*
                ===================================================================================================================
                                        +---------------------------------------------------+------------------------------------
                Equipment 1             | Step 1, Batch A  | Transfer               | Clean | Step 1, Batch C                   >
                                        +------------------+        from resource 1 +-------+------------------------------------
                Equipment 2                                |          to resource 2 | Step 2, Batch A                        >
                                                           +-----------------------------------------------------------------
                Manual labour                               xxxx               xxxx   xxxxx
                ========================^==================^========================^=======^========================================
                Time                    S1A                E1A                      S2A     S1F
                variable                step_start         step_stop               next_go  resource_available
                */
                for (step_group, interval) in step_iter {
                    let step_start = ast::Int::new_const(
                        &ctx,
                        format!("start batch {} {} {:?}", batch.beer.name, i, step_group),
                    );

                    // Constraint: set start first step in future
                    solver.assert(&step_start.ge(&start_horizon));

                    // Constraint: set end of step .. or .. after start
                    let step_stop = ast::Int::new_const(
                        &ctx,
                        format!("stop batch {} {} {:?}", batch.beer.name, i, step_group),
                    );
                    let (_earliest, latest) = interval.range();
                    solver.assert(&step_stop.ge(&ast::Int::add(
                        &ctx,
                        &[&step_start, &ast::Int::from_i64(&ctx, latest.num_seconds())],
                    )));

                    // Constraint: set what resource can be used
                    let equipment_group = step_group.equipment_group();
                    let step_machine = ast::Dynamic::from_ast(&ast::Int::new_const(
                        &ctx,
                        format!(
                            "machine for batch {} {} {:?}",
                            batch.beer.name, i, step_group
                        ),
                    ));
                    let one_of_these = machines
                        .get(&(batch.system.clone(), equipment_group.clone()))
                        .expect(&format!(
                            "Cannot find machines for system {:?} and group {:?}",
                            batch.system, equipment_group
                        ));

                    solver.assert(&one_of_these.member(&step_machine));

                    // Constraint: the next step may only start after the previous step is done
                    match &prev {
                        None => prev = Some(step_stop),
                        Some(p) => {
                            solver.assert(&step_start.ge(&p));
                            // @FIXME: should be "end of transfer"instead of previous step;
                            let transfer_time = step_group.post_process_time(batch.system);
                            // @FIME: block the resource untill it is ready after cleaning
                            let clean_time = step_group.post_process_time(batch.system);
                        }
                    }
                }
            }
        }
        // avoid duplicate use of machine during operation, transfer or cleaning
        // set transfer and clean operation only during office hours
        // set transfer and clean operation nod during holidays
        //todo bottleneck first

        //todo: solver.optimize(ctx, solver, &self.containers);

        match solver.check() {
            SatResult::Sat => {
                let _model = solver.get_model();
                //let used = model.eval(z3var).unwrap().as_bool().unwrap();
            }
            SatResult::Unsat => {
                // println!("No solution found!");
            }
            SatResult::Unknown => {
                print!(
                    "No solution found: {}",
                    solver.get_reason_unknown().unwrap()
                );
                panic!("TODO: better error handling");
            }
        }
    }
    // @FIXME: return the planning outcome
}

#[cfg(test)]
pub mod mock {
    use super::*;

    pub fn plan() -> Plan {
        Plan::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::factory;

    #[test]
    fn test_plan_new() {
        let plan = mock::plan();
        assert_eq!(plan._dummy, 4);
    }
    #[test]
    fn test_plan_do_magic() {
        let plan = mock::plan();
        let factory = factory::mock::factory();
        let now: DateTime<Utc> = Utc::now();
        let wishlist = vec![];
        // @FIXME: better test case: real beer in factory
        let batches_needed = factory.calculate_batches(wishlist);
        plan.do_magic(&factory, batches_needed.as_slice(), now);
    }
}
