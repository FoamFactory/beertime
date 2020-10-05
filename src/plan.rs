use crate::batchneed::BatchNeed;
use crate::factory::Factory;
use crate::volume::Volume;

use z3::{Config, Context, SatResult, Solver};

pub struct Plan {
    dummy: u8,
}

impl Plan {
    pub fn new() -> Self {
        Self { dummy: 4 }
    }
    pub fn do_magic(&self) {
        let mut config = Config::new();
        config.set_proof_generation(false);
        config.set_model_generation(true);
        config.set_debug_ref_count(false);
        config.set_timeout_msec(5_000);
        let context = Context::new(&config);
        let solver = Solver::new(&context);
        // for batch in batches
        //   for step in batch
        //      set start first step in future
        //      set end of step .. or .. after start
        //      set start of transfer operation after end of step
        //      set end of  transfer .. after start of transfer
        //      set start of of next step ... after end of transfer
        //      set start of clean operation after end of transfer
        //      set end of clean operation .. after start of clean
        // avoid duplicate use of machine during operation, transfer or cleaning
        // set transfer and clean operation only during office hours
        // set transfer and clean operation nod during holidays
        //todo bottleneck first
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
