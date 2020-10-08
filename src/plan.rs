use std::collections::HashMap;

use chrono::prelude::*;
use z3::{ast, ast::Ast, Config, Context, FuncDecl, Optimize, SatResult, Sort};

use crate::action::Action;
use crate::batchneed::BatchNeed;
use crate::equipment::Equipment;
use crate::equipment_group::EquipmentGroup;
use crate::factory::Factory;
use crate::step_group::StepGroup;
use crate::system::System;

#[derive(Debug, PartialEq)]
pub struct Plan<'a> {
    id: usize,
    batch: &'a BatchNeed<'a>,
    step_group: StepGroup,
    action: Action,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

const S1A: &'static str = "STARTED";
const E1A: &'static str = "STOPPED";
const S2A: &'static str = "TRANSFERED";
const S1F: &'static str = "CLEANED";

macro_rules! gen_z3_var {
    ($z3_step_times: expr, $var_name: ident, $format: expr, $ctx: expr, $batch:expr, $step_group: expr, $label: expr) => {
        let $var_name = ast::Int::new_const(
            &$ctx,
            format!($format, $batch.beer.name, $batch.id, $step_group.clone()),
        );
        $z3_step_times.insert(($batch.id, $step_group.clone(), $label), $var_name.clone());
    };
}

impl<'a> Plan<'a> {
    pub fn new(
        id: usize,
        batch: &'a BatchNeed<'a>,
        step_group: StepGroup,
        action: Action,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Self {
        match &action {
            Action::Process(equipment) | Action::Clean(equipment) => {
                assert_eq!(step_group.equipment_group(), equipment.equipment_group);
            }
            Action::Transfer(equipment, _not_relavant) => {
                assert_eq!(step_group.equipment_group(), equipment.equipment_group);
            }
        }

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
        // 1) we setup the solver
        let mut cfg = Config::new();
        cfg.set_proof_generation(false);
        cfg.set_model_generation(true);
        cfg.set_debug_ref_count(false);
        //cfg.set_timeout_msec(5_000);
        let ctx = Context::new(&cfg);
        let solver = Optimize::new(&ctx);

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
        // 2) We setup some lookup tables to keep track of our variables
        let mut z3_step_times = HashMap::with_capacity(batches_needed.len() * 6 * 4);
        let mut z3_step_machine = HashMap::with_capacity(batches_needed.len() * 6);
        let mut all_endings = Vec::new();
        let step_groups = StepGroup::all();
        let systems = System::all();

        let mut z3_machines = HashMap::with_capacity(step_groups.len());
        for step_group in step_groups {
            for system in &systems {
                let equipment_group = step_group.equipment_group();
                z3_machines.insert((equipment_group, system.clone()), HashMap::new());
            }
        }
        let mut machine_id = 1;
        for equipment in factory.equipments.values() {
            if let Some(map) =
                z3_machines.get_mut(&(equipment.equipment_group.clone(), equipment.system.clone()))
            {
                // @TODO: a ast::Sort of ast::Int would be better here, so that
                //        we could check machine_step.member(suited_machines).
                //        For some reason z3 gives a illegale exectution (don't remember)
                //        error. Therefor, we use this (plain number) as a work around.
                let machine = ast::Int::new_const(&ctx, format!("Equipment {}", equipment.name));
                map.insert(machine_id, (machine.clone(), equipment.clone()));
                //     Constraint-like: give the machine a unqiue number, that can be added to every step
                solver.assert(&machine._eq(&ast::Int::from_i64(&ctx, machine_id as i64)));
                machine_id += 1;
            }
        }
        // 3) We iterate through the batches and each of its steps
        let start_horizon = ast::Int::from_i64(&ctx, earliest_start.timestamp());
        for batch in batches_needed.values() {
            let mut prev = None;
            let mut start = start_horizon.clone();
            for (step_group, interval) in batch.steps() {
                // Where we define some variables for the solver and add constraints
                let (_earliest, longest) = interval.range();
                let machine_step = ast::Int::new_const(
                    &ctx,
                    format!(
                        "Machine for batch: {}, beer: {} step: {:?}",
                        batch.beer.name,
                        batch.id,
                        step_group.clone()
                    ),
                );
                z3_step_machine.insert((batch.id, step_group.clone()), machine_step.clone());
                let equipment_group = step_group.equipment_group();
                let mut machine_counts = 0;
                if let Some(suited) = z3_machines.get(&(equipment_group, batch.system.clone())) {
                    let suited_machines = suited
                        .values()
                        .map(|(int, _equ)| int)
                        .collect::<Vec<&ast::Int>>();
                    let mut ors = Vec::with_capacity(suited_machines.len());
                    for machine in suited_machines {
                        let allowed = machine_step._eq(machine);
                        ors.push(allowed);
                        machine_counts += 1;
                    }
                    let bors = ors.iter().map(|x| x).collect::<Vec<&ast::Bool>>();
                    //     Constraint: only one of these machines can be used for this step
                    solver.assert(&ast::Bool::or(&ctx, bors.as_slice()));
                }
                gen_z3_var!(
                    z3_step_times,
                    step_start,
                    "started batch: {}, beer: {} step: {:?}",
                    ctx,
                    batch,
                    step_group,
                    S1A
                );
                gen_z3_var!(
                    z3_step_times,
                    step_stop,
                    "stopped batch: {}, beer: {} step: {:?}",
                    ctx,
                    batch,
                    step_group,
                    E1A
                );
                gen_z3_var!(
                    z3_step_times,
                    next_go,
                    "transfered batch: {}, beer: {} step: {:?}",
                    ctx,
                    batch,
                    step_group,
                    S2A
                );
                gen_z3_var!(
                    z3_step_times,
                    resource_available,
                    "cleaned batch: {}, beer: {} step: {:?}",
                    ctx,
                    batch,
                    step_group,
                    S1F
                );
                all_endings.push(ast::Dynamic::from_ast(&resource_available));
                // TODO: in the future, some batches may be actually be in production,
                //       that would mean that need to skip some steps and set another
                //       start time here.
                //     Constraint: set start first step in future
                solver.assert(&step_start.ge(&start));
                //     Constraint: set end of step .. or .. after start
                solver.assert(&step_stop._eq(&ast::Int::add(
                    &ctx,
                    &[
                        &step_start,
                        &ast::Int::from_i64(&ctx, longest.num_seconds()),
                    ],
                )));
                //     Constraint: the next step may only start after the previous step is done.
                //     Although we did not do a 'assert' here, the effect
                //     is the same due to the way that we set up this loop.
                let transfer_time = step_group.post_process_time(batch.system);
                solver.assert(&next_go._eq(&ast::Int::add(
                    &ctx,
                    &[
                        &step_stop,
                        &ast::Int::from_i64(&ctx, transfer_time.num_seconds()),
                    ],
                )));
                start = next_go.clone();
                //     Constraint: the equipment is available after the cleaning
                //     Implied Constraint: clean machine is the same as the machine that made it dirty
                let clean_time = step_group.post_process_time(batch.system);
                solver.assert(&resource_available._eq(&ast::Int::add(
                    &ctx,
                    &[
                        &next_go,
                        &ast::Int::from_i64(&ctx, clean_time.num_seconds()),
                    ],
                )));
                match prev {
                    None => prev = Some((step_group, machine_step, machine_counts)),
                    Some((ref _prev_step_group, ref prev_machine_step, _prev_machine_counts)) => {
                        // TODO: find out why "if &step_group == prev_step_group && prev_machine_counts > 1 " fails
                        let same = machine_step._eq(prev_machine_step);
                        //     Constraint: Previous step's machine is not this step's machine
                        solver.assert(&ast::Bool::and(&ctx, &[&same]).not());
                    }
                }
            }
        }
        // 3b) Now that we have variables for the start/stop-times and the machines,
        //     we can set up that one machine can only do 1 task at the same time.
        for ((this_batch_id, this_step_group), _this_step_machine) in z3_step_machine.iter() {
            // we unwrap here 4 * 2 times, but a pyramid of 'if let Some()' could also work
            let this_step_start = z3_step_times
                .get(&(*this_batch_id, this_step_group.clone(), S1A))
                .unwrap();
            let this_resource_available = z3_step_times
                .get(&(*this_batch_id, this_step_group.clone(), S1F))
                .unwrap();
            let mut overlaps = Vec::new();
            for ((other_batch_id, other_step_group), _other_step_machine) in z3_step_machine.iter()
            {
                if this_batch_id != other_batch_id && this_step_group != other_step_group {
                    let other_step_start = z3_step_times
                        .get(&(*other_batch_id, other_step_group.clone(), S1A))
                        .unwrap();
                    let other_resource_available = z3_step_times
                        .get(&(*other_batch_id, other_step_group.clone(), S1F))
                        .unwrap();
                    //     Constraint: This machine in occupied from step_start till resource_available
                    overlaps.push(
                        ast::Bool::and(
                            &ctx,
                            &[
                                &this_resource_available.ge(&other_step_start),
                                &other_resource_available.ge(&this_step_start),
                            ],
                        )
                        .not(),
                    );
                    // todo....The other machine is also occupied from step_stop till next_go
                }
            }
            let ooverlaps = overlaps.iter().map(|x| x).collect::<Vec<&ast::Bool>>();
            solver.assert(&ast::Bool::or(&ctx, ooverlaps.as_slice()));
        }

        //     Constraint: both the resources are occupied during transfer
        // @TODO: avoid duplicate use of machine during operation, transfer or cleaning
        // @TODO: set transfer and clean operation only during office hours
        // @TODO: set transfer and clean operation not during holidays
        // @TODO: Bottleneck first
        // @TODO: Buffer before bottleneck

        Plan::optimize(&solver, &ctx, earliest_start, all_endings);
        Plan::process_solution(
            factory,
            batches_needed,
            solver,
            z3_machines,
            z3_step_machine,
            z3_step_times,
        )
    }

    fn optimize<'ctx>(
        solver: &'ctx Optimize,
        ctx: &'ctx Context,
        earliest_start: DateTime<Utc>,
        all_endings: Vec<ast::Dynamic>,
    ) {
        // 4) We optimize for the shortest time that all machines are in the resource_available state
        //    The variabls all_endings
        let longest_start = earliest_start.timestamp();
        let shortest_longest_duration_of_all_tasks = ast::Int::new_const(ctx, "Max time");
        let parameter_sorts = all_endings
            .iter()
            .map(|_x| Sort::int(ctx))
            .collect::<Vec<Sort>>();
        let oparameter_sorts = parameter_sorts.iter().map(|x| x).collect::<Vec<&Sort>>();
        let fmax = FuncDecl::new(
            ctx,
            "MaximizeFunction",
            oparameter_sorts.as_slice(),
            &Sort::int(ctx),
        );
        //@FIXME: implement fmax function
        //     Constraint-hint: We can limit the search field, so that the optimize has less work.
        solver.assert(
            &shortest_longest_duration_of_all_tasks.ge(&ast::Int::from_i64(ctx, longest_start)),
        );
        //     Constraint-optimizer
        let oall_endings = all_endings
            .iter()
            .map(|x| x)
            .collect::<Vec<&ast::Dynamic>>();
        solver.assert(
            &shortest_longest_duration_of_all_tasks
                ._eq(&fmax.apply(oall_endings.as_slice()).as_int().unwrap()),
        );
        solver.minimize(&shortest_longest_duration_of_all_tasks);
    }

    fn process_solution<'ctx>(
        factory: &'a Factory,
        batches_needed: &'a HashMap<usize, BatchNeed<'a>>,
        solver: Optimize<'ctx>,
        z3_machines: HashMap<(EquipmentGroup, System), HashMap<usize, (ast::Int<'ctx>, Equipment)>>,
        z3_step_machine: HashMap<(usize, StepGroup), ast::Int<'ctx>>,
        z3_step_times: HashMap<(usize, StepGroup, &'static str), ast::Int<'ctx>>,
    ) -> Vec<Plan<'a>> {
        let mut machine_lookup = HashMap::with_capacity(factory.equipments.len());
        for (k, (_int, equ)) in z3_machines.values().flatten() {
            machine_lookup.insert(*k, equ);
        }

        match solver.check(&[]) {
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
            SatResult::Sat => {
                let model = solver.get_model();
                // println!("{:?}", model);
                // First normalize all the z3 variables into a hashmap that let
                // us see the process, transfer and clean timestamps and the
                // involved equimpent
                let mut temp: HashMap<
                    (usize, StepGroup),
                    (
                        Option<Equipment>,
                        Option<DateTime<Utc>>,
                        Option<DateTime<Utc>>,
                        Option<DateTime<Utc>>,
                        Option<DateTime<Utc>>,
                        Option<Equipment>,
                    ),
                > = HashMap::with_capacity(batches_needed.len() * 6);

                for ((batch_id, step_group, label), var) in z3_step_times.iter() {
                    let machine_step = z3_step_machine
                        .get(&(*batch_id, step_group.clone()))
                        .unwrap();
                    let machine_value = model.eval(machine_step).unwrap().as_i64().unwrap();
                    let equipment = machine_lookup.get(&(machine_value as usize)).unwrap();
                    let equipment_2 = factory.equipments.values().nth(1).unwrap();
                    let ts_value = model.eval(var).unwrap().as_i64().unwrap();
                    let ts =
                        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(ts_value, 0), Utc);
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
                                (
                                    Some(equipment.clone().clone().clone()),
                                    ts1a,
                                    te1a,
                                    ts2a,
                                    ts1f,
                                    None,
                                ),
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
                                    *other_equipment = Some(equipment_2.clone());
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
                        step_group.clone().clone(),
                        Action::Process(equipment.as_ref().unwrap().clone()),
                        ts1a.unwrap(),
                        te1a.unwrap(),
                    ));
                    plan_id += 1;
                    if other_equipment.is_some() {
                        solutions.push(Plan::new(
                            plan_id,
                            batch,
                            step_group.clone(),
                            Action::Transfer(
                                equipment.as_ref().unwrap().clone(),
                                other_equipment.as_ref().unwrap().clone(),
                            ),
                            te1a.unwrap(),
                            ts2a.unwrap(),
                        ));
                        plan_id += 1;
                    }
                    solutions.push(Plan::new(
                        plan_id,
                        batch,
                        step_group.clone(),
                        Action::Clean(equipment.as_ref().unwrap().clone()),
                        ts2a.unwrap(),
                        ts1f.unwrap(),
                    ));
                    plan_id += 1;
                }

                //println!(">{:?}", solutions);

                return solutions;
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
        // Put steps in each sequence in a logical order: from brew->aging
        for seq in out.values_mut() {
            seq.sort_by(|a, b| a.step_group.cmp(&b.step_group));
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
        equipment: equipment::Equipment,
        step_group: step_group::StepGroup,
        batchneed: &'a batchneed::BatchNeed<'a>,
    ) -> Plan<'a> {
        let action = action::mock::process(equipment);
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
        let plan = mock::plan(equipment.clone(), step_group.clone(), &batchneed);
        let equipment = equipment::mock::equipment();
        assert_eq!(plan.id, 666);
        assert_eq!(plan.batch.beer, &beer);
        assert_eq!(plan.step_group, step_group);
        assert_eq!(plan.batch.system, &system);
        assert_eq!(plan.action, action::mock::process(equipment));
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
