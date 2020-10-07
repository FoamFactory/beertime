use std::collections::HashMap;

use chrono::prelude::*;
use z3::{ast, ast::Ast, Config, Context, SatResult, Solver, Sort};

use crate::action::Action;
use crate::batchneed::BatchNeed;
use crate::factory::Factory;
use crate::step_group::StepGroup;

#[derive(Debug, PartialEq)]
pub struct Plan<'a> {
    id: usize,
    batch: &'a BatchNeed<'a>,
    step_group: StepGroup,
    action: Action<'a>,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl<'a> Plan<'a> {
    pub fn new(
        id: usize,
        batch: &'a BatchNeed<'a>,
        step_group: StepGroup,
        action: Action<'a>,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Self {
        /* @FIXME: activate this check
        match action {
            Action::Process(equipment) | Action::Clean(equipment) => {
                assert_eq!(step_group.equipment_group(), equipment.equipment_group);
            }
            Action::Transfer(equipment, _not_relavant) => {
                assert_eq!(step_group.equipment_group(), equipment.equipment_group);
            }
        }
        */
        Self {
            id,
            batch,
            step_group,
            action,
            start,
            end,
        }
    }

    pub fn plan(
        factory: &'a Factory,
        batches_needed: &'a [BatchNeed<'a>],
        earliest_start: DateTime<Utc>,
    ) -> Vec<Plan<'a>> {
        let mut cfg = Config::new();
        cfg.set_proof_generation(false);
        cfg.set_model_generation(true);
        cfg.set_debug_ref_count(false);
        //cfg.set_timeout_msec(5_000);
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
        /*
        let start_horizon = ast::Int::from_i64(&ctx, earliest_start.timestamp());
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
                                  machine_step         machine_transfers     machine_clean
        */
        /*
        for (i, batch) in batches_needed.iter().enumerate() {
            if let Some((max_volume, steps)) = batch.beer.recipy.get(batch.system) {
                assert!(batch.volume.ge(max_volume));
                let mut prev = None;
                for (step_group, interval) in steps.iter() {
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
                    let machine_step = ast::Dynamic::from_ast(&ast::Int::new_const(
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

                    //solver.assert(&one_of_these.member(&machine_step));

                    // Constraint: the next step may only start after the previous step is done
                    match &prev {
                        None => prev = Some((step_stop, machine_step)),
                        Some((prev_step_stop, prev_machine_step)) => {
                            solver.assert(&step_start.ge(&prev_step_stop));
                            // Constraint: both the resources are occupied
                            let transfer_time = step_group.post_process_time(batch.system);
                            let next_go = ast::Int::new_const(
                                &ctx,
                                format!("Transfered {} {} {:?}", batch.beer.name, i, step_group),
                            );
                            solver.assert(&next_go._eq(&ast::Int::add(
                                &ctx,
                                &[
                                    &step_stop,
                                    &ast::Int::from_i64(&ctx, transfer_time.num_seconds()),
                                ],
                            )));
                            let machine_transfers = ast::Set::new_const(
                                &ctx,
                                format!(
                                    "machines needed during transfer before batch {} {} {:?}",
                                    batch.beer.name, i, step_group
                                ),
                                &Sort::int(&ctx),
                            );
                            machine_transfers.add(&machine_step);
                            machine_transfers.add(&prev_machine_step);

                            // Constraint: clean time occupies resource
                            let clean_time = step_group.post_process_time(batch.system);
                            let resource_available = ast::Int::new_const(
                                &ctx,
                                format!("Transfered {} {} {:?}", batch.beer.name, i, step_group),
                            );
                            solver.assert(&resource_available._eq(&ast::Int::add(
                                &ctx,
                                &[
                                    &step_stop,
                                    &ast::Int::from_i64(
                                        &ctx,
                                        transfer_time.num_seconds() + clean_time.num_seconds(),
                                    ),
                                ],
                            )));
                            let machine_clean = ast::Dynamic::from_ast(&ast::Set::new_const(
                                &ctx,
                                format!(
                                    "machine cleaning after batch {} {} {:?}",
                                    batch.beer.name, i, step_group
                                ),
                                &Sort::int(&ctx),
                            ));
                            // Constraint: clean machine is the sames as the machine that made it dirty
                            //solver.assert(&machine_clean._eq(&machine_step));

                            // Constraint: Next step's machine is not this step machine
                            let prev_match_set = ast::Set::fresh_const(&ctx, &format!("this machine is not the same as prev step for batch {} {} {:?}", batch.beer.name, i, step_group), &Sort::int(&ctx));
                            prev_match_set.add(&prev_machine_step);
                            //solver.assert(&prev_match_set.member(&machine_step).not());
                        }
                    }
                }
                // @TODO: cleaning of the last step...
                counter += 3; // the step, the transfer, the cleaning
            }
        }
        // @TODO: avoid duplicate use of machine during operation, transfer or cleaning
        // @TODO: set transfer and clean operation only during office hours
        // @TODO: set transfer and clean operation nod during holidays
        // @TODO: Bottleneck first
        // @TODO: solver.optimize(ctx, solver, &self.containers);
        */

        let solution = match solver.check() {
            SatResult::Sat => {
                let _model = solver.get_model();
                //let used = model.eval(z3var).unwrap().as_bool().unwrap();
                //println!("{:?}", model);
                let solution = Plan::_fake_plan(factory, batches_needed, earliest_start);
                solution
            }
            SatResult::Unsat => {
                println!("No solution found!");
                panic!("TODO: better error handling");
            }
            SatResult::Unknown => {
                print!(
                    "No solution found: {}",
                    solver.get_reason_unknown().unwrap()
                );
                panic!("TODO: better error handling");
            }
        };

        solution
    }

    pub fn _fake_plan(
        factory: &'a Factory,
        batches_needed: &'a [BatchNeed<'a>],
        earliest_start: DateTime<Utc>,
    ) -> Vec<Plan<'a>> {
        //fake output
        let mut solution = Vec::with_capacity(batches_needed.len());
        let mut start = earliest_start;
        let mut plan_id = 1;
        for batch in batches_needed {
            if let Some((max_volume, steps)) = batch.beer.recipy.get(batch.system) {
                assert!(batch.volume.ge(max_volume));
                let mut prev = None;
                for (step_group, interval) in steps.iter() {
                    let equipment = factory.equipments.values().nth(0).unwrap(); // this is not correct
                    let process_start = start;
                    let (_fastest, longest) = interval.range();
                    let process_duration = longest;
                    let process_end = process_start + process_duration;
                    let process = Plan::new(
                        plan_id,
                        batch,
                        step_group.clone(),
                        Action::Process(&equipment),
                        process_start,
                        process_end,
                    );
                    plan_id += 1;
                    solution.push(process);
                    let other_equipment = factory.equipments.values().nth(1).unwrap(); // this is not correct
                    let duration_transfer = step_group.post_process_time(batch.system);
                    let transfer_end = process_end + duration_transfer;
                    let transfer = Plan::new(
                        plan_id,
                        batch,
                        step_group.clone(),
                        Action::Transfer(&equipment, &other_equipment),
                        process_end,
                        transfer_end,
                    );
                    plan_id += 1;
                    solution.push(transfer);
                    let duration_clean = step_group.post_process_time(batch.system);
                    let clean_end = transfer_end + duration_clean;
                    let clean = Plan::new(
                        plan_id,
                        batch,
                        step_group,
                        Action::Clean(&equipment),
                        transfer_end,
                        clean_end,
                    );
                    plan_id += 1;
                    solution.push(clean);
                    match prev {
                        None => prev = Some(1),
                        Some(_what_is_this_thing) => {}
                    }
                    start = transfer_end;
                }
            }
        }

        solution
    }

    // @TODO: sort_by_equipment_group
    //        sort_by_equipment
    //        sort_by_step_group,
    //        sort_by_step
    //        sort_by_beer,
    //        sort_by_style,
    pub fn sort_by_batch(planning: &'a [Plan<'a>]) -> HashMap<String, Vec<&'a Plan<'a>>> {
        Plan::sort_by_xxxx(planning, |plan| plan.batch.id.to_string())
    }

    pub fn sort_by_xxxx(
        planning: &'a [Plan<'a>],
        cl: fn(&'a Plan<'a>) -> String,
    ) -> HashMap<String, Vec<&'a Plan<'a>>> {
        let mut out = HashMap::new();
        for plan in planning {
            let id = cl(plan);
            match out.get_mut(&id) {
                None => {
                    out.insert(id, vec![plan]);
                }
                Some(seq) => {
                    seq.push(plan);
                }
            }
        }

        out
    }

    pub fn pla_basic(
        planning: &'a [Plan<'a>],
        ordering: fn(&'a [Plan<'a>]) -> HashMap<String, Vec<&'a Plan<'a>>>,
    ) -> String {
        let sorted = (ordering)(planning);
        let mut blocks = Vec::with_capacity(planning.len());
        for (_ids, plans) in sorted.iter() {
            let first = plans.get(0).unwrap();
            let name = first.batch.beer.name.clone();
            let mut prev = None;
            let children = plans
                .iter()
                .map(|plan| format!("\t\t\t\t child {_plan_id}", _plan_id = plan.id))
                .collect::<Vec<String>>();
            let main_block = format!(
                r#"[{_batch_id}] {_name} (Batch: {_batch_id})
                {_childs }
                "#,
                _batch_id = first.batch.id * 10000, //@FIXME: there must be a better way
                _name = name,
                _childs = children.join("\n")
            );
            blocks.push(main_block);
            for plan in plans.iter() {
                let step_group = plan.step_group.clone();
                let duration = plan.end - plan.start;
                let dep = if let Some(p) = prev {
                    format!("dep {}", p)
                } else {
                    "".to_string()
                };
                let resources = plan
                    .action
                    .resources()
                    .iter()
                    .map(|x| format!("\t\t\t\tres {_res}", _res = x))
                    .collect::<Vec<String>>();
                let block = format!(
                    r#" [{_plan_id}] {_step_name} {_activity}
                            duration {_hours}
                            start {_start}
                            {_res}
                            {_dep }

                        "#,
                    _plan_id = plan.id,
                    _step_name = step_group.lookup(),
                    _activity = plan.action.lookup(),
                    _hours = duration.num_hours(),
                    _res = resources.join("\n"),
                    _start = plan.start.format("%Y-%m-%d %H"),
                    _dep = dep,
                );
                prev = Some(plan.id);
                blocks.push(block);
            }
        }

        let out = format!(
            r#"
{_blocks}
        "#,
            _blocks = blocks.join("\n"),
        );
        out
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use crate::action;
    use crate::batchneed;
    use crate::equipment;
    use crate::step_group;

    pub fn plan<'a>(
        equipment: &'a equipment::Equipment,
        step_group: step_group::StepGroup,
        batchneed: &'a batchneed::BatchNeed<'a>,
    ) -> Plan<'a> {
        let action = action::mock::process(&equipment);
        let start = Utc.ymd(2020, 12, 30).and_hms(13, 14, 15);
        let end = Utc.ymd(2020, 12, 30).and_hms(15, 14, 15);

        Plan::new(666, &batchneed, step_group, action, start, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action;
    use crate::batchneed;
    use crate::beer;
    use crate::equipment;
    use crate::factory;
    use crate::step_group;
    use crate::system;

    #[test]
    fn test_plan_new() {
        let beer = beer::mock::beer();
        let system = system::mock::bbl5();
        let equipment = equipment::mock::equipment();
        let batchneed = batchneed::mock::batchneed(&beer, &system);
        let step_group = step_group::mock::brewing();
        let plan = mock::plan(&equipment, step_group.clone(), &batchneed);
        let equipment = equipment::mock::equipment();
        assert_eq!(plan.id, 666);
        assert_eq!(plan.batch.beer, &beer);
        assert_eq!(plan.step_group, step_group);
        assert_eq!(plan.batch.system, &system);
        assert_eq!(plan.action, action::mock::process(&equipment));
        assert!(plan.start < plan.end);
    }

    #[test]
    fn test_plan_do_magic() {
        let factory = factory::mock::factory();
        let now: DateTime<Utc> = Utc::now();
        let wishlist = vec![];
        // @FIXME: better test case: real beer in factory
        let batches_needed = factory.calculate_batches(wishlist);
        let _solution = Plan::plan(&factory, batches_needed.as_slice(), now);
        //@TODO: better tests
    }
}
