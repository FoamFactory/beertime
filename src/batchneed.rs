use crate::beer::Beer;
use crate::interval::Interval;
use crate::step_group::StepGroup;
use crate::batch_size::BatchSize;
use crate::volume::Volume;

#[derive(Debug, PartialEq)]
pub struct BatchNeed<'a> {
    pub id: usize,
    pub beer: &'a Beer,
    pub system: BatchSize,
    pub volume: Volume,
}

impl<'a> BatchNeed<'a> {
    pub fn new(id: usize, beer: &'a Beer, system: BatchSize, volume: Volume) -> Self {
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

    pub fn batchneed<'a>(beer: &'a Beer, system: BatchSize) -> BatchNeed<'a> {
        BatchNeed::new(1, beer, system, volume::mock::gallon_us())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::beer;
    use crate::batch_size;
    use crate::volume;

    #[test]
    fn test_batchneed_new() {
        let beer = beer::mock::beer();
        let system = batch_size::mock::bbl5();
        let batchneed = mock::batchneed(&beer, system.clone());
        assert_eq!(batchneed.id, 1);
        assert_eq!(batchneed.beer, &beer);
        assert_eq!(batchneed.system, system);
        assert_eq!(batchneed.volume, volume::mock::gallon_us());
    }
}
