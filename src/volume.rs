use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Volume {
    GallonUS(f32),
    GallonUSDry(f32),
    GallonImperial(f32),
    Liter(f32),
    Lb(f32), //CO2 weight unit
}

macro_rules! convert_to {
    ($enumname: expr, $unit: expr) => {{
        let (in_factor, amount) = Volume::si_unit($unit);
        let (out_factor, _) = Volume::si_unit(&$enumname(0.0));
        $enumname(in_factor / out_factor * amount)
    }};
}

impl Volume {
    pub fn lookup(&self) -> String {
        match self {
            Volume::GallonUS(x) => format!("{}G", x),
            Volume::GallonUSDry(x) => format!("{} US Dry Gallon", x),
            Volume::GallonImperial(x) => format!("{} Imperial Gallon", x),
            Volume::Liter(x) => format!("{} liters", x),
            Volume::Lb(x) => format!("{} pound (mass)", x),
        }
    }

    pub fn si_unit(unit: &Volume) -> (f32, f32) {
        match unit {
            &Volume::GallonUS(x) => (3.785411784 / 1_000_000.0, x),
            &Volume::GallonUSDry(x) => (4.404_883_770_86 / 1_000_000.0, x),
            &Volume::GallonImperial(x) => (4.546_09 / 1_000_000.0, x),
            &Volume::Liter(x) => (1.0 / 1_000_000.0, x),
            &Volume::Lb(_) => panic!("lb is a weigth, not a volume"),
        }
    }

    pub fn to_gallon_us(&self) -> Volume {
        convert_to!(Volume::GallonUS, self)
    }

    pub fn to_gallon_us_dry(&self) -> Volume {
        convert_to!(Volume::GallonUSDry, self)
    }

    pub fn to_gallon_imperial(&self) -> Volume {
        convert_to!(Volume::GallonImperial, self)
    }

    pub fn to_liter(&self) -> Volume {
        convert_to!(Volume::Liter, self)
    }

    pub fn full_batches(&self, batch_size: &Volume) -> usize {
        if let Volume::Liter(need) = self.to_liter() {
            if let Volume::Liter(size) = batch_size.to_liter() {
                return (need / size).ceil() as usize;
            }
        }
        panic!("Should not happen");
    }

    pub fn ge(&self, other: &Volume) -> bool {
        //todo: change this into an impl std::num::cmp
        if let Volume::Liter(this) = self.to_liter() {
            if let Volume::Liter(that) = other.to_liter() {
                return this >= that;
            }
        }
        panic!("Should not happen");
    }
}

impl std::str::FromStr for Volume {
    type Err = ();

    fn from_str(s: &str) -> Result<Volume, ()> {
        if s.len() > 1 && (s.ends_with('g') || s.ends_with('G')) {
            if let Ok(gallons) = s[0..s.len() - 1].parse() {
                return Ok(Volume::GallonUS(gallons));
            }
        }
        Err(())
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;

    pub fn gallon_us() -> Volume {
        Volume::GallonUS(12.2)
    }

    pub fn gallon_us_dry() -> Volume {
        Volume::GallonUSDry(12.2)
    }

    pub fn gallon_imperial() -> Volume {
        Volume::GallonImperial(12.2)
    }

    pub fn liter() -> Volume {
        Volume::Liter(12.2)
    }

    pub fn lb() -> Volume {
        Volume::Lb(12.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_convert_liter_gallonus() {
        assert_eq!(mock::liter().to_liter(), Volume::Liter(12.2));
        assert_eq!(mock::gallon_us().to_gallon_us(), Volume::GallonUS(12.2));
        assert_eq!(mock::gallon_us().to_liter(), Volume::Liter(46.182022));
        assert_eq!(
            Volume::Liter(46.18202).to_gallon_us(),
            Volume::GallonUS(12.199998)
        );
    }

    #[test]
    fn test_volume_lookup() {
        assert_eq!(&mock::gallon_us().lookup(), "12.2G");
        assert_eq!(&mock::gallon_us_dry().lookup(), "12.2 US Dry Gallon");
        assert_eq!(&mock::gallon_imperial().lookup(), "12.2 Imperial Gallon");
        assert_eq!(&mock::liter().lookup(), "12.2 liters");
    }

    #[test]
    fn test_volume_parse() {
        assert_eq!("5g".parse(), Ok(Volume::GallonUS(5.0)));
        assert_eq!("12.2g".parse(), Ok(Volume::GallonUS(12.2)));
        assert_eq!("5G".parse(), Ok(Volume::GallonUS(5.0)));
        //assert_eq!("5 Gallon".parse().is_err(), true);
        //assert_eq!("5 L".parse().is_err(), true);
        //assert_eq!("5l".parse().is_err(), true);
    }

    #[test]
    fn test_batch_count() {
        let need_gallons = Volume::GallonUS(10.0);
        let size = Volume::GallonUS(0.3);
        assert_eq!(need_gallons.full_batches(&size), 34);

        let need_liters = Volume::Liter(100.0);
        assert_eq!(need_liters.full_batches(&size), 89);
    }

    #[test]
    fn test_volume_eq() {
        let ten = Volume::GallonUS(10.0);
        let one = Volume::GallonUS(1.0);
        assert_eq!(ten.ge(&one), true);
        assert_eq!(one.ge(&ten), false);
        assert_eq!(ten.ge(&ten), true);
    }
}
