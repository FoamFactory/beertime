use crate::volume::Volume;

#[derive(Debug, PartialEq)]
pub enum System {
    G5,
    G10,
    BBL5,
    BBL7,
    BBL10,
    BBL15,
}

impl System {
    pub fn lookup(&self) -> &'static str {
        match self {
            System::G5 => "5G",
            System::G10 => "10G",
            System::BBL5 => "5BBL",
            System::BBL7 => "7BBL",
            System::BBL10 => "10BBL",
            System::BBL15 => "15BBL",
        }
    }
}

impl std::str::FromStr for System {
    type Err = ();

    fn from_str(s: &str) -> Result<System, ()> {
        match s {
            "5G" => Ok(System::G5),
            "10G" => Ok(System::G10),
            "5BBL" => Ok(System::BBL5),
            "10BBL" => Ok(System::BBL10),
            "15BBL" => Ok(System::BBL15),
            _ => Err(()),
        }
    }
}

impl System {
    pub fn volume(&self) -> Volume {
        match self {
            System::G5 => Volume::GallonUS(5.0),
            System::G10 => Volume::GallonUS(10.0),
            System::BBL5 => Volume::GallonUS(210.0),
            System::BBL7 => Volume::GallonUS(294.0),
            System::BBL10 => Volume::GallonUS(420.0),
            System::BBL15 => Volume::GallonUS(630.0),
        }
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;

    pub fn g5() -> System {
        System::G5
    }

    pub fn g10() -> System {
        System::G10
    }

    pub fn bbl5() -> System {
        System::BBL5
    }

    pub fn bbl10() -> System {
        System::BBL10
    }

    pub fn bbl15() -> System {
        System::BBL15
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_lookup() {
        assert_eq!(System::G5.lookup(), "5G");
        assert_eq!(System::G10.lookup(), "10G");
        assert_eq!(System::BBL5.lookup(), "5BBL");
        assert_eq!(System::BBL7.lookup(), "7BBL");
        assert_eq!(System::BBL10.lookup(), "10BBL");
        assert_eq!(System::BBL15.lookup(), "15BBL");
    }

    #[test]
    fn test_style_parse() {
        assert_eq!("5G".parse(), Ok(System::G5));
        assert_eq!("10G".parse(), Ok(System::G10));
        assert_eq!("5BBL".parse(), Ok(System::BBL5));
        assert_eq!("10BBL".parse(), Ok(System::BBL10));
        assert_eq!("15BBL".parse(), Ok(System::BBL15));
    }

    #[test]
    fn test_system_volume() {
        assert_eq!(System::G5.volume(), Volume::GallonUS(5.0));
        assert_eq!(System::G10.volume(), Volume::GallonUS(10.0));
        assert_eq!(System::BBL5.volume(), Volume::GallonUS(210.0));
        assert_eq!(System::BBL7.volume(), Volume::GallonUS(294.0));
        assert_eq!(System::BBL10.volume(), Volume::GallonUS(420.0));
        assert_eq!(System::BBL15.volume(), Volume::GallonUS(630.0));
    }
}
