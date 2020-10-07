use std::collections::HashMap;

use chrono::prelude::*;
use z3::{ast, ast::Ast, Config, Context, SatResult, Solver, Sort};

use crate::action::Action;
use crate::batchneed::BatchNeed;
use crate::equipment::Equipment;
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

const S1A: &'static str = "STARTED";
const E1A: &'static str = "STOPPED";
const S2A: &'static str = "TRANSFERED";
const S1F: &'static str = "CLEANED";

macro_rules! gen_z3_var {
    ($z3_vars: expr, $var_name: ident, $format: expr, $ctx: expr, $batch:expr, $step_group: expr, $label: expr) => {
        let $var_name = ast::Int::new_const(
            &$ctx,
            format!($format, $batch.beer.name, $batch.id, $step_group.clone()),
        );
        $z3_vars.insert(($batch.id, $step_group.clone(), $label), $var_name.clone());
    };
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
        /* @FIXME: activate this check, it panics now because the fake_plan is build with stupid equipments.
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
        batches_needed: &'a HashMap<usize, BatchNeed<'a>>,
        earliest_start: DateTime<Utc>,
    ) -> Vec<Plan<'a>> {
        let mut cfg = Config::new();
        cfg.set_proof_generation(false);
        cfg.set_model_generation(true);
        cfg.set_debug_ref_count(false);
        //cfg.set_timeout_msec(5_000);
        let ctx = Context::new(&cfg);
        let solver = Solver::new(&ctx);
        let start_horizon = ast::Int::from_i64(&ctx, earliest_start.timestamp());
        /*
        ===================================================================================================================
                                +---------------------------------------------------+------------------------------------
        Equipment 1             | Step 1, Batch A  | Transfer               | Clean | Step 1, Batch B                   >
                                +------------------+        from resource 1 +-------+------------------------------------
        Equipment 2                                |          to resource 2 | Step 2, Batch A                        >
                                                   +-----------------------------------------------------------------
        Manual labour                               xxxx               xxxx   xxxxx
        ========================^==================^========================^=======^========================================
        Time                    S1A                E1A                      S2A     S1F
        variable                step_start         step_stop               next_go  resource_available
                                 machine_step         machine_transfers     machine_clean
        */

        let mut z3_vars = HashMap::with_capacity(batches_needed.len() * 6 * 4);
        for batch in batches_needed.values() {
            //println!("batch {} beer {}", batch.id, batch.beer.name);
            //let mut prev = None;
            let mut start = start_horizon.clone();
            for (step_group, interval) in batch.steps() {
                let (_earliest, longest) = interval.range();
                //println!("\t step {:?} {:?}", step_group, longest.num_hours());
                gen_z3_var!(
                    z3_vars,
                    step_start,
                    "started batch: {}, beer: {} step: {:?}",
                    ctx,
                    batch,
                    step_group,
                    S1A
                );
                gen_z3_var!(
                    z3_vars,
                    step_stop,
                    "stopped batch: {}, beer: {} step: {:?}",
                    ctx,
                    batch,
                    step_group,
                    E1A
                );
                gen_z3_var!(
                    z3_vars,
                    next_go,
                    "transfered batch: {}, beer: {} step: {:?}",
                    ctx,
                    batch,
                    step_group,
                    S2A
                );
                gen_z3_var!(
                    z3_vars,
                    resource_available,
                    "cleaned batch: {}, beer: {} step: {:?}",
                    ctx,
                    batch,
                    step_group,
                    S1F
                );
                // TODO: in the future, some batches may be actually be in production,
                //       that would mean that need to skip some steps and set another
                //       start time here.
                // Constraint: set start first step in future
                solver.assert(&step_start.ge(&start));
                // Constraint: set end of step .. or .. after start
                solver.assert(&step_stop._eq(&ast::Int::add(
                    &ctx,
                    &[
                        &step_start,
                        &ast::Int::from_i64(&ctx, longest.num_seconds()),
                    ],
                )));
                // Constraint: the next step may only start after the previous
                //             step is done.
                //             although we did not do a 'assert' here, the effect
                //              is the same due to the way that we set up this loop.
                let transfer_time = step_group.post_process_time(batch.system);
                solver.assert(&next_go._eq(&ast::Int::add(
                    &ctx,
                    &[
                        &step_stop,
                        &ast::Int::from_i64(&ctx, transfer_time.num_seconds()),
                    ],
                )));
                start = next_go.clone();
                // Constraint: the equipment is available after the cleaning
                let clean_time = step_group.post_process_time(batch.system);
                solver.assert(&resource_available._eq(&ast::Int::add(
                    &ctx,
                    &[
                        &next_go,
                        &ast::Int::from_i64(&ctx, clean_time.num_seconds()),
                    ],
                )));

                /*
                match prev {
                    None => prev = Some((next_gostep_stop,)),
                    Some((ref _step_stop,)) => {
                        // Constraint: the next step may only start after the previous step is done
                    }
                }
                */
            }
        }

        // Constraint: set what resource can be used
        // Constraint: both the resources are occupied during transfer
        // Constraint: clean machine is the same as the machine that made it dirty
        // Constraint: Next step's machine is not this step machine

        // @TODO: avoid duplicate use of machine during operation, transfer or cleaning
        // @TODO: set transfer and clean operation only during office hours
        // @TODO: set transfer and clean operation not during holidays
        // @TODO: Bottleneck first
        // @TODO: solver.optimize(ctx, solver, &self.shortest_longest_duration_of_all_tasks);

        match solver.check() {
            SatResult::Sat => {
                let model = solver.get_model();
                // println!("{:?}", model);
                // First normalize all the z3 variables into a hashmap that let
                // us see the process, transfer and clean timestamps and the
                // involved equimpent
                let mut temp: HashMap<
                    (usize, StepGroup),
                    (
                        Option<&Equipment>,
                        Option<DateTime<Utc>>,
                        Option<DateTime<Utc>>,
                        Option<DateTime<Utc>>,
                        Option<DateTime<Utc>>,
                        Option<&Equipment>,
                    ),
                > = HashMap::with_capacity(batches_needed.len() * 6);
                let equipment_1 = factory.equipments.values().nth(0).unwrap();
                let equipment_2 = factory.equipments.values().nth(1).unwrap();
                for ((batch_id, step_group, label), var) in z3_vars.iter() {
                    let value = model.eval(var).unwrap().as_i64().unwrap();
                    let ts =
                        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(value, 0), Utc);
                    match temp.get_mut(&(*batch_id, step_group.clone())) {
                        None => {
                            let mut ts1a = None;
                            let mut te1a = None;
                            let mut ts2a = None;
                            let mut ts1f = None;
                            match *label {
                                S1A => ts1a = Some(ts),
                                E1A => te1a = Some(ts),
                                S2A => ts2a = Some(ts),
                                S1F => ts1f = Some(ts),
                                _ => panic!("should not happen"),
                            };
                            temp.insert(
                                (*batch_id, step_group.clone()),
                                (Some(equipment_1), ts1a, te1a, ts2a, ts1f, None),
                            );
                        }
                        Some((_equipment, ts1a, te1a, ts2a, ts1f, other_equipment)) => {
                            match *label {
                                S1A => {
                                    *ts1a = Some(ts);
                                }
                                E1A => {
                                    *te1a = Some(ts);
                                }
                                S2A => {
                                    *ts2a = Some(ts);
                                    *other_equipment = Some(equipment_2);
                                }
                                S1F => {
                                    *ts1f = Some(ts);
                                }
                                _ => panic!("should not happen"),
                            };
                        }
                    }
                    //println!("{} {:?} {} {:?}", batch_id, step_group, label, ts);
                }
                // now we can build a Vec<Plan> with the known actions
                // @TODO: investigate is this is realy needed after all.
                let mut solutions: Vec<Plan> = Vec::with_capacity(temp.len() * 3);
                let mut plan_id = 1;
                for tmp in temp.iter() {
                    //println!("{:?}", tmp);
                    let (
                        (batch_id, step_group),
                        (equipment, ts1a, te1a, ts2a, ts1f, other_equipment),
                    ) = tmp;
                    let batch = batches_needed.get(batch_id).unwrap();
                    solutions.push(Plan::new(
                        plan_id,
                        batch,
                        step_group.clone(),
                        Action::Process(equipment.unwrap()),
                        ts1a.unwrap(),
                        te1a.unwrap(),
                    ));
                    plan_id += 1;
                    if other_equipment.is_some() {
                        solutions.push(Plan::new(
                            plan_id,
                            batch,
                            step_group.clone(),
                            Action::Transfer(equipment.unwrap(), other_equipment.unwrap()),
                            te1a.unwrap(),
                            ts2a.unwrap(),
                        ));
                        plan_id += 1;
                    }
                    solutions.push(Plan::new(
                        plan_id,
                        batch,
                        step_group.clone(),
                        Action::Clean(equipment.unwrap()),
                        ts2a.unwrap(),
                        ts1f.unwrap(),
                    ));
                    plan_id += 1;
                }

                //println!(">{:?}", solutions);

                return solutions;
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
                .map(|plan| format!("\n    child {_plan_id}", _plan_id = plan.id))
                .collect::<Vec<String>>();
            let main_block = format!(
                r#"
[{_batch_id}] {_name} (Batch: {_batch_id})
    {_childs }
"#,
                _batch_id = first.batch.id * 10000, //@FIXME: there must be a better way
                _name = name,
                _childs = children.join("")
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
                    .map(|x| format!("\n    res {_res}", _res = x))
                    .collect::<Vec<String>>();
                let block = format!(
                    r#"
[{_plan_id}] {_step_name} {_activity}
    duration {_hours}
    start {_start}
    {_res}
    {_dep }
"#,
                    _plan_id = plan.id,
                    _step_name = step_group.lookup(),
                    _activity = plan.action.lookup(),
                    _hours = duration.num_hours(),
                    _res = resources.join(""),
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
            _blocks = blocks.join(""),
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
        let _solution = Plan::plan(&factory, &batches_needed, now);
        //@TODO: better tests
    }
}
