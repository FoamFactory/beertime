use crate::volume::Volume;

#[macro_use]
use crate::convert_to;

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum BatchSize {
    G5,
    G10,
    BBL5,
    BBL7,
    BBL10,
    BBL15,
}

impl BatchSize {
    pub fn lookup(&self) -> &'static str {
        match self {
            BatchSize::G5 => "5G",
            BatchSize::G10 => "10G",
            BatchSize::BBL5 => "5BBL",
            BatchSize::BBL7 => "7BBL",
            BatchSize::BBL10 => "10BBL",
            BatchSize::BBL15 => "15BBL",
        }
    }

    pub fn volume(&self) -> Volume {
        match self {
            BatchSize::G5 => Volume::GallonUS(5.0),
            BatchSize::G10 => Volume::GallonUS(10.0),
            BatchSize::BBL5 => Volume::BeerBarrel(5.0),
            BatchSize::BBL7 => Volume::BeerBarrel(7.0),
            BatchSize::BBL10 => Volume::BeerBarrel(10.0),
            BatchSize::BBL15 => Volume::BeerBarrel(15.0),
        }
    }

    pub fn all() -> Vec<BatchSize> {
        vec![
            BatchSize::G5,
            BatchSize::G10,
            BatchSize::BBL5,
            BatchSize::BBL7,
            BatchSize::BBL10,
            BatchSize::BBL15,
        ]
    }
}

impl std::str::FromStr for BatchSize {
    type Err = ();

    fn from_str(s: &str) -> Result<BatchSize, ()> {
        match s {
            "5G" => Ok(BatchSize::G5),
            "10G" => Ok(BatchSize::G10),
            "5BBL" => Ok(BatchSize::BBL5),
            "10BBL" => Ok(BatchSize::BBL10),
            "15BBL" => Ok(BatchSize::BBL15),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;

    pub fn g5() -> BatchSize {
        BatchSize::G5
    }

    pub fn g10() -> BatchSize {
        BatchSize::G10
    }

    pub fn bbl5() -> BatchSize {
        BatchSize::BBL5
    }

    pub fn bbl10() -> BatchSize {
        BatchSize::BBL10
    }

    pub fn bbl15() -> BatchSize {
        BatchSize::BBL15
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_lookup() {
        assert_eq!(BatchSize::G5.lookup(), "5G");
        assert_eq!(BatchSize::G10.lookup(), "10G");
        assert_eq!(BatchSize::BBL5.lookup(), "5BBL");
        assert_eq!(BatchSize::BBL7.lookup(), "7BBL");
        assert_eq!(BatchSize::BBL10.lookup(), "10BBL");
        assert_eq!(BatchSize::BBL15.lookup(), "15BBL");
    }

    #[test]
    fn test_style_parse() {
        assert_eq!("5G".parse(), Ok(BatchSize::G5));
        assert_eq!("10G".parse(), Ok(BatchSize::G10));
        assert_eq!("5BBL".parse(), Ok(BatchSize::BBL5));
        assert_eq!("10BBL".parse(), Ok(BatchSize::BBL10));
        assert_eq!("15BBL".parse(), Ok(BatchSize::BBL15));
    }

    #[test]
    fn test_system_volume() {
        assert_eq!(BatchSize::G5.volume(), Volume::GallonUS(5.0));
        assert_eq!(BatchSize::G10.volume(), Volume::GallonUS(10.0));
        assert_eq!(BatchSize::BBL5.volume(), Volume::BeerBarrel(5.0));
        assert_eq!(BatchSize::BBL7.volume(), Volume::BeerBarrel(7.0));
        assert_eq!(BatchSize::BBL10.volume(), Volume::BeerBarrel(10.0));
        assert_eq!(BatchSize::BBL15.volume(), Volume::BeerBarrel(15.0));
    }
    #[test]
    fn test_system_all() {
        assert_eq!(BatchSize::all().len(), 6);
    }
}
