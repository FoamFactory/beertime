use beertime::volume::Volume;

#[cfg(test)]
pub mod mock {
    use super::*;

    pub fn gallon_us() -> Volume {
        Volume::GallonUS(5.0)
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

    pub fn seven_bbl() -> Volume { Volume::BeerBarrel(7.0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_convert_bbl() {
        assert_eq!(mock::seven_bbl().to_gallon_us(), Volume::GallonUS(217.00043));
        assert_eq!(mock::seven_bbl().to_liter(), Volume::Liter(821.4359));
    }
    #[test]
    fn test_volume_convert_liter_gallonus() {
        assert_eq!(mock::liter().to_liter(), Volume::Liter(12.2));
        assert_eq!(mock::gallon_us().to_gallon_us(), Volume::GallonUS(5.0));
        assert_eq!(mock::gallon_us().to_liter(), Volume::Liter(18.927057));
        assert_eq!(
            Volume::Liter(46.182).to_gallon_us(),
            Volume::GallonUS(12.199993)
        );
    }

    #[test]
    fn test_volume_lookup() {
        assert_eq!(&mock::gallon_us().lookup(), "5G");
        assert_eq!(&mock::gallon_us_dry().lookup(), "12.2 US Dry Gallon");
        assert_eq!(&mock::gallon_imperial().lookup(), "12.2 Imperial Gallon");
        assert_eq!(&mock::liter().lookup(), "12.2 liters");
    }

    #[test]
    fn test_volume_parse() {
        assert_eq!("5g".parse(), Ok(Volume::GallonUS(5.0)));
        assert_eq!("12.2g".parse(), Ok(Volume::GallonUS(12.2)));
        assert_eq!("5G".parse(), Ok(Volume::GallonUS(5.0)));
        assert_eq!("7bbl".parse(), Ok(Volume::BeerBarrel(7.0)));
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

    #[test]
    fn test_volume_si() {
        assert_eq!(
            Volume::si_unit(&Volume::GallonUS(10.0)),
            (0.0037854118, 10.0)
        );
    }
}
