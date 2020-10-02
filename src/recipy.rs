use crate::beer::Beer;
use crate::steps::Steps;
use crate::system::System;
use crate::volume::Volume;

#[derive(Debug, PartialEq)]
pub struct Recipy {
    beer: Beer,
    system: System,
    r#yield: Volume,
    steps: Steps,
}

impl Recipy {
    pub fn new(beer: Beer, system: System, r#yield: Volume, steps: Steps) -> Self {
        Self {
            beer,
            system,
            r#yield,
            steps,
        }
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use crate::beer;
    use crate::steps;
    use crate::system;
    use crate::volume;

    pub fn recipy() -> Recipy {
        Recipy::new(
            beer::mock::beer(),
            system::mock::g5(),
            volume::mock::gallon_us(),
            steps::mock::steps(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::beer;
    use crate::steps;
    use crate::system;
    use crate::volume;

    #[test]
    fn test_beer_new() {
        let recipy = mock::recipy();
        assert_eq!(recipy.beer, beer::mock::beer());
        assert_eq!(recipy.system, system::mock::g5());
        assert_eq!(recipy.r#yield, volume::mock::gallon_us());
        assert_eq!(recipy.steps, steps::mock::steps());
    }
}
