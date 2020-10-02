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
mod tests {
    use super::*;

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
