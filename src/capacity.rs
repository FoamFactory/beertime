use crate::volume::Volume;
use std::fmt::{Display, Formatter};

#[macro_use]
use crate::convert_to;

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Capacity {
    G5,
    G10,
    G14,
    G15,
    BBL5,
    BBL7,
    BBL10,
    BBL15,
    UNKNOWN,
}

pub trait SizeCheck {
    fn is_us_gallon(&self, amount: f32) -> bool;
    fn is_beer_barrel(&self, amount: f32) -> bool;
}

impl SizeCheck for String {
    fn is_us_gallon(&self, amount: f32) -> bool {
        return self.as_str() == format!("{}g", amount) || self.as_str() == format!("{}G", amount);
    }

    fn is_beer_barrel(&self, amount: f32) -> bool {
        return self.as_str() == format!("{}bbl", amount)
            || self.as_str() == format!("{}BBL", amount);
    }
}

impl Capacity {
    pub fn lookup(&self) -> &'static str {
        match self {
            Capacity::G5 => "5G",
            Capacity::G10 => "10G",
            Capacity::G14 => "14G",
            Capacity::G15 => "15G",
            Capacity::BBL5 => "5BBL",
            Capacity::BBL7 => "7BBL",
            Capacity::BBL10 => "10BBL",
            Capacity::BBL15 => "15BBL",
            Capacity::UNKNOWN => "Unknown",
        }
    }

    pub fn volume(&self) -> Volume {
        match self {
            Capacity::G5 => Volume::GallonUS(5.0),
            Capacity::G10 => Volume::GallonUS(10.0),
            Capacity::G14 => Volume::GallonUS(14.0),
            Capacity::G15 => Volume::GallonUS(15.0),
            Capacity::BBL5 => Volume::BeerBarrel(5.0),
            Capacity::BBL7 => Volume::BeerBarrel(7.0),
            Capacity::BBL10 => Volume::BeerBarrel(10.0),
            Capacity::BBL15 => Volume::BeerBarrel(15.0),
            Capacity::UNKNOWN => panic!("Unknown capacity!"),
        }
    }

    pub fn all() -> Vec<Capacity> {
        vec![
            Capacity::G5,
            Capacity::G10,
            Capacity::G14,
            Capacity::G15,
            Capacity::BBL5,
            Capacity::BBL7,
            Capacity::BBL10,
            Capacity::BBL15,
        ]
    }
}

impl Display for Capacity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.volume().lookup())
    }
}

impl std::str::FromStr for Capacity {
    type Err = ();

    fn from_str(s: &str) -> Result<Capacity, ()> {
        match s {
            s if s.to_string().is_us_gallon(5 as f32) => Ok(Capacity::G5),
            s if s.to_string().is_us_gallon(10 as f32) => Ok(Capacity::G10),
            s if s.to_string().is_us_gallon(14 as f32) => Ok(Capacity::G14),
            s if s.to_string().is_us_gallon(15 as f32) => Ok(Capacity::G15),
            s if s.to_string().is_beer_barrel(5 as f32) => Ok(Capacity::BBL5),
            s if s.to_string().is_beer_barrel(7 as f32) => Ok(Capacity::BBL7),
            s if s.to_string().is_beer_barrel(10 as f32) => Ok(Capacity::BBL10),
            s if s.to_string().is_beer_barrel(15 as f32) => Ok(Capacity::BBL15),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;

    pub fn g5() -> Capacity {
        Capacity::G5
    }

    pub fn g10() -> Capacity {
        Capacity::G10
    }

    pub fn g14() -> Capacity {
        Capacity::G14
    }

    pub fn g15() -> Capacity {
        Capacity::G15
    }

    pub fn bbl5() -> Capacity {
        Capacity::BBL5
    }

    pub fn bbl10() -> Capacity {
        Capacity::BBL10
    }

    pub fn bbl15() -> Capacity {
        Capacity::BBL15
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_lookup() {
        assert_eq!(Capacity::G5.lookup(), "5G");
        assert_eq!(Capacity::G10.lookup(), "10G");
        assert_eq!(Capacity::G15.lookup(), "15G");
        assert_eq!(Capacity::BBL5.lookup(), "5BBL");
        assert_eq!(Capacity::BBL7.lookup(), "7BBL");
        assert_eq!(Capacity::BBL10.lookup(), "10BBL");
        assert_eq!(Capacity::BBL15.lookup(), "15BBL");
    }

    #[test]
    fn test_style_parse() {
        assert_eq!("5G".parse(), Ok(Capacity::G5));
        assert_eq!("5g".parse(), Ok(Capacity::G5));
        assert_eq!("10G".parse(), Ok(Capacity::G10));
        assert_eq!("15g".parse(), Ok(Capacity::G15));
        assert_eq!("5BBL".parse(), Ok(Capacity::BBL5));
        assert_eq!("7bbl".parse(), Ok(Capacity::BBL7));
        assert_eq!("10BBL".parse(), Ok(Capacity::BBL10));
        assert_eq!("15BBL".parse(), Ok(Capacity::BBL15));
    }

    #[test]
    fn test_system_capacity() {
        assert_eq!(Capacity::G5.volume(), Volume::GallonUS(5.0));
        assert_eq!(Capacity::G10.volume(), Volume::GallonUS(10.0));
        assert_eq!(Capacity::G15.volume(), Volume::GallonUS(15.0));
        assert_eq!(Capacity::BBL5.volume(), Volume::BeerBarrel(5.0));
        assert_eq!(Capacity::BBL7.volume(), Volume::BeerBarrel(7.0));
        assert_eq!(Capacity::BBL10.volume(), Volume::BeerBarrel(10.0));
        assert_eq!(Capacity::BBL15.volume(), Volume::BeerBarrel(15.0));
    }
    #[test]
    fn test_system_all() {
        assert_eq!(Capacity::all().len(), 8);
    }
}
