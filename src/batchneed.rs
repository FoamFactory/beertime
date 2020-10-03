use crate::beer::Beer;
use crate::system::System;
use crate::volume::Volume;

#[derive(Debug)]
pub struct BatchNeed<'a> {
    pub beer: &'a Beer,
    pub system: &'a System,
    pub volume: Volume,
}

impl<'a> BatchNeed<'a> {
    pub fn new(beer: &'a Beer, system: &'a System, volume: Volume) -> Self {
        Self {
            beer,
            system,
            volume,
        }
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use crate::volume;

    pub fn batchneed<'a>(beer: &'a Beer, system: &'a System) -> BatchNeed<'a> {
        BatchNeed::new(beer, system, volume::mock::gallon_us())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::beer;
    use crate::system;
    use crate::volume;

    #[test]
    fn test_batchneed_new() {
        let beer = beer::mock::beer();
        let system = system::mock::bbl5();
        let batchneed = mock::batchneed(&beer, &system);
        assert_eq!(batchneed.beer, &beer);
        assert_eq!(batchneed.system, &system);
        assert_eq!(batchneed.volume, volume::mock::gallon_us());
    }
}
