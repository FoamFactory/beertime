use crate::r#type::Type;

#[derive(Debug, PartialEq)]
pub enum Style {
    AmberLager,
    BlondeAle,
    BrownAle,
    CaliforniaCommon,
    FruitBeer,
    IPA,
    ImperialStout,
    IrishRedAle,
    Kellerbier,
    Pilsner,
    SmokedAle,
    SpecialtyStout,
}

impl Style {
    pub fn lookup(&self) -> &'static str {
        match self {
            Style::AmberLager => "Amber Lager",
            Style::BlondeAle => "Blonde Ale",
            Style::BrownAle => "Brown Ale",
            Style::CaliforniaCommon => "California Common",
            Style::FruitBeer => "Fruit Beer",
            Style::IPA => "IPA",
            Style::ImperialStout => "Imperial Stout",
            Style::IrishRedAle => "Irish Red Ale",
            Style::Kellerbier => "Kellerbier",
            Style::Pilsner => "Pilsner",
            Style::SmokedAle => "Smoked Ale",
            Style::SpecialtyStout => "Specialty Stout",
        }
    }

    pub fn r#type(&self) -> Type {
        match self {
            Style::AmberLager | Style::Kellerbier | Style::Pilsner => Type::Lager,
            _ => Type::Ale,
        }
    }
}

impl std::str::FromStr for Style {
    type Err = ();

    fn from_str(s: &str) -> Result<Style, ()> {
        match s {
            "Amber Lager" => Ok(Style::AmberLager),
            "Blonde Ale" => Ok(Style::BlondeAle),
            "Brown Ale" => Ok(Style::BrownAle),
            "California Common" => Ok(Style::CaliforniaCommon),
            "Fruit Beer" => Ok(Style::FruitBeer),
            "IPA" => Ok(Style::IPA),
            "Imperial Stout" => Ok(Style::ImperialStout),
            "Irish Red Ale" => Ok(Style::IrishRedAle),
            "Kellerbier" => Ok(Style::Kellerbier),
            "Pilsner" => Ok(Style::Pilsner),
            "Smoked Ale" => Ok(Style::SmokedAle),
            "Specialty Stout" => Ok(Style::SpecialtyStout),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;

    pub fn mock_amber_lager() -> Style {
        Style::AmberLager
    }

    pub fn mock_blonde_ale() -> Style {
        Style::BlondeAle
    }

    pub fn mock_brown_ale() -> Style {
        Style::BrownAle
    }

    pub fn mock_california_common() -> Style {
        Style::CaliforniaCommon
    }

    pub fn mock_fruit_beer() -> Style {
        Style::FruitBeer
    }

    pub fn mock_ipa() -> Style {
        Style::IPA
    }

    pub fn mock_imperial_stout() -> Style {
        Style::ImperialStout
    }

    pub fn mock_irish_red_ale() -> Style {
        Style::IrishRedAle
    }

    pub fn mock_kellerbier() -> Style {
        Style::Kellerbier
    }

    pub fn mock_pilsner() -> Style {
        Style::Pilsner
    }

    pub fn mock_smoked_ale() -> Style {
        Style::SmokedAle
    }

    pub fn mock_specialty_stout() -> Style {
        Style::SpecialtyStout
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_lookup() {
        assert_eq!(Style::AmberLager.lookup(), "Amber Lager");
        assert_eq!(Style::BlondeAle.lookup(), "Blonde Ale");
        assert_eq!(Style::BrownAle.lookup(), "Brown Ale");
        assert_eq!(Style::CaliforniaCommon.lookup(), "California Common");
        assert_eq!(Style::FruitBeer.lookup(), "Fruit Beer");
        assert_eq!(Style::IPA.lookup(), "IPA");
        assert_eq!(Style::ImperialStout.lookup(), "Imperial Stout");
        assert_eq!(Style::IrishRedAle.lookup(), "Irish Red Ale");
        assert_eq!(Style::Kellerbier.lookup(), "Kellerbier");
        assert_eq!(Style::Pilsner.lookup(), "Pilsner");
        assert_eq!(Style::SmokedAle.lookup(), "Smoked Ale");
        assert_eq!(Style::SpecialtyStout.lookup(), "Specialty Stout");
    }

    #[test]
    fn test_style_parse() {
        assert_eq!("Amber Lager".parse(), Ok(Style::AmberLager));
        assert_eq!("Blonde Ale".parse(), Ok(Style::BlondeAle));
        assert_eq!("Brown Ale".parse(), Ok(Style::BrownAle));
        assert_eq!("California Common".parse(), Ok(Style::CaliforniaCommon));
        assert_eq!("Fruit Beer".parse(), Ok(Style::FruitBeer));
        assert_eq!("IPA".parse(), Ok(Style::IPA));
        assert_eq!("Imperial Stout".parse(), Ok(Style::ImperialStout));
        assert_eq!("Irish Red Ale".parse(), Ok(Style::IrishRedAle));
        assert_eq!("Kellerbier".parse(), Ok(Style::Kellerbier));
        assert_eq!("Pilsner".parse(), Ok(Style::Pilsner));
        assert_eq!("Smoked Ale".parse(), Ok(Style::SmokedAle));
        assert_eq!("Specialty Stout".parse(), Ok(Style::SpecialtyStout));
    }

    #[test]
    fn test_style_type() {
        assert_eq!(mock::mock_amber_lager().r#type(), Type::Lager);
        assert_eq!(mock::mock_kellerbier().r#type(), Type::Lager);
        assert_eq!(mock::mock_pilsner().r#type(), Type::Lager);

        assert_eq!(mock::mock_blonde_ale().r#type(), Type::Ale);
        assert_eq!(mock::mock_california_common().r#type(), Type::Ale);
        assert_eq!(mock::mock_fruit_beer().r#type(), Type::Ale);
        assert_eq!(mock::mock_ipa().r#type(), Type::Ale);
        assert_eq!(mock::mock_imperial_stout().r#type(), Type::Ale);
        assert_eq!(mock::mock_irish_red_ale().r#type(), Type::Ale);
        assert_eq!(mock::mock_smoked_ale().r#type(), Type::Ale);
        assert_eq!(mock::mock_specialty_stout().r#type(), Type::Ale);
    }
}
