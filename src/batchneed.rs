use crate::beer::Beer;
use crate::capacity::Capacity;
use crate::interval::Interval;
use crate::step_group::StepGroup;
use crate::volume::Volume;

#[derive(Debug, PartialEq)]
pub struct BatchNeed<'a> {
    pub id: usize,
    pub beer: &'a Beer,
    pub system: Capacity,
    pub volume: Volume,
}

impl<'a> BatchNeed<'a> {
    pub fn new(id: usize, beer: &'a Beer, system: Capacity, volume: Volume) -> Self {
        Self {
            id,
            beer,
            system,
            volume,
        }
    }

    pub fn steps(&self) -> Vec<(StepGroup, Interval)> {
        if let Some((max_volume, steps)) = self.beer.recipe.get(&self.system) {
            assert!(self.volume.ge(max_volume));
            return steps.iter().collect();
        }
        panic!("Should not happen");
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use crate::volume;

    pub fn mock_batchneed<'a>(beer: &'a Beer, system: Capacity) -> BatchNeed<'a> {
        BatchNeed::new(1, beer, system, volume::mock::mock_gallon_us())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::beer;
    use crate::capacity;
    use crate::volume;

    #[test]
    fn test_batchneed_new() {
        let beer = beer::mock::mock_beer();
        let system = capacity::mock::mock_bbl5();
        let batchneed = mock::mock_batchneed(&beer, system.clone());
        assert_eq!(batchneed.id, 1);
        assert_eq!(batchneed.beer, &beer);
        assert_eq!(batchneed.system, system);
        assert_eq!(batchneed.volume, volume::mock::mock_gallon_us());
    }
}
