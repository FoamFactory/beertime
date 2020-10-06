use serde::Serialize;
use std::collections::HashMap;

use crate::steps::Steps;
use crate::system::System;
use crate::volume::Volume;

#[derive(Debug, PartialEq, Serialize)]
pub struct Recipy {
    pub map: HashMap<System, (Volume, Steps)>,
}

impl Recipy {
    pub fn blank() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn new(system: System, r#yield: Volume, steps: Steps) -> Self {
        let mut recipy = Recipy::blank();
        recipy.map.insert(system, (r#yield, steps));

        recipy
    }

    pub fn store(&mut self, system: System, r#yield: Volume, steps: Steps) {
        assert_eq!(self.get(&system), None);
        self.map.insert(system, (r#yield, steps));
    }

    pub fn get(&self, system: &System) -> Option<&(Volume, Steps)> {
        self.map.get(system)
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use crate::steps;
    use crate::system;
    use crate::volume;

    pub fn recipy() -> Recipy {
        Recipy::new(
            system::mock::g5(),
            volume::mock::gallon_us(),
            steps::mock::steps(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::steps;
    use crate::system;
    use crate::volume;

    #[test]
    fn test_recipy_new() {
        let recipy = mock::recipy();
        assert_eq!(
            recipy.get(&system::mock::g5()),
            Some(&(volume::mock::gallon_us(), steps::mock::steps()))
        );
    }
}
