use crate::batchneed::BatchNeed;
use crate::factory::Factory;
use crate::steps::StepIterator;
use crate::volume::Volume;

use z3::{ast, ast::Ast, Config, Context, SatResult, Solver};

pub struct Plan {
    dummy: u8,
}

impl Plan {
    pub fn new() -> Self {
        Self { dummy: 4 }
    }

    pub fn do_magic(&self, batches_needed: &[BatchNeed]) {
        let mut cfg = Config::new();
        cfg.set_proof_generation(false);
        cfg.set_model_generation(true);
        cfg.set_debug_ref_count(false);
        cfg.set_timeout_msec(5_000);
        let ctx = Context::new(&cfg);
        let solver = Solver::new(&ctx);
        for (i, batch) in batches_needed.iter().enumerate() {
            if let Some((max_volume, steps)) = batch.beer.recipy.get(batch.system) {
                assert!(batch.volume.ge(max_volume));
                let mut prev = None;
                let step_iter = StepIterator::new(steps);
                for (step_group, interval) in step_iter {
                    // @todo set start first step in future
                    let stop_start = ast::Int::new_const(
                        &ctx,
                        format!("start batch {} {} {:?}", batch.beer.name, i, step_group),
                    );
                    let step_stop = ast::Int::new_const(
                        &ctx,
                        format!("stop batch {} {} {:?}", batch.beer.name, i, step_group),
                    );
                    // set end of step .. or .. after start
                    solver.assert(&step_stop.ge(&ast::Int::add(
                        &ctx,
                        &[
                            &stop_start,
                            &ast::Int::from_i64(&ctx, interval.range().1.num_seconds()),
                        ],
                    )));
                    // @TODO: set what resource can be used

                    // @TODO: set start of transfer operation after end of step
                    // @TODO: set end of  transfer .. after start of transfer
                    // @TODO: set start of clean operation after end of transfer
                    // @TODO: set end of clean operation .. after start of clean
                    // @TODO: set start of of next step ... after end of privious step
                    // @FIXME: should be "end of transfer"instead of previous step;
                    match &prev {
                        None => prev = Some(step_stop),
                        Some(p) => solver.assert(&stop_start.ge(&p)),
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
                let model = solver.get_model();
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
}

pub fn plan(_factory: &Factory, _batches_needed: &[BatchNeed]) -> Plan {
    let plan = Plan::new();
    plan
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

    #[test]
    fn test_plan_new() {
        let plan = mock::plan();
        assert_eq!(plan.dummy, 4);
    }
}
