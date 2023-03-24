use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum Volume {
    BeerBarrel(f32),
    GallonUS(f32),
    GallonUSDry(f32),
    GallonImperial(f32),
    Liter(f32),
    Lb(f32), //CO2 weight unit
}

#[macro_export]
macro_rules! convert_to {
    ($enumname: expr, $unit: expr) => {{
        let (in_factor, amount) = Volume::si_unit($unit);
        let (out_factor, _) = Volume::si_unit(&$enumname(0.0));
        $enumname(in_factor / out_factor * amount)
    }};
}

impl Display for Volume {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lookup())
    }
}

impl Volume {
    pub fn lookup(&self) -> String {
        match self {
            Volume::BeerBarrel(x) => format!("{}BBL", x),
            Volume::GallonUS(x) => format!("{}G", x),
            Volume::GallonUSDry(x) => format!("{} US Dry Gallon", x),
            Volume::GallonImperial(x) => format!("{} Imperial Gallon", x),
            Volume::Liter(x) => format!("{} liters", x),
            Volume::Lb(x) => format!("{} pound (mass)", x),
        }
    }

    pub fn si_unit(unit: &Volume) -> (f32, f32) {
        match unit {
            &Volume::BeerBarrel(x) => (117.348 / 1_000.0, x),
            &Volume::GallonUS(x) => (3.785411784 / 1_000.0, x),
            &Volume::GallonUSDry(x) => (4.40488377086 / 1_000.0, x),
            &Volume::GallonImperial(x) => (4.54609 / 1_000.0, x),
            &Volume::Liter(x) => (1.0 / 1_000.0, x),
            &Volume::Lb(_) => panic!("lb is a weight, not a volume"),
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

    pub fn to_bbl(&self) -> Volume { convert_to!(Volume::BeerBarrel, self) }

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
        } else if s.len() > 3 && (s.ends_with("bbl") || s.ends_with("BBL")) {
            if let Ok(barrels) = s[0..s.len() - 3].parse() {
                return Ok(Volume::BeerBarrel(barrels));
            }
        }
        Err(())
    }
}