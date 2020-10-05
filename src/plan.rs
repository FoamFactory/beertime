use crate::batchneed::BatchNeed;
use crate::factory::Factory;
use crate::volume::Volume;

pub struct Plan {
    dummy: u8,
}

impl Plan {
    pub fn new() -> Self {
        Self { dummy: 4 }
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
