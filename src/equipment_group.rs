#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EquipmentGroup {
    MashTun,
    LauterTun,
    HotLiquorTank,
    Kettle,
    Fermentor,
    CO2Tank,
    Keg,
}

impl EquipmentGroup {
    pub fn lookup(&self) -> &'static str {
        match self {
            EquipmentGroup::MashTun => "Mash Tun",
            EquipmentGroup::LauterTun => "Lauter Tun",
            EquipmentGroup::HotLiquorTank => "Hot Liqoor Tank",
            EquipmentGroup::Kettle => "Kettle",
            EquipmentGroup::Fermentor => "Fermentor",
            EquipmentGroup::CO2Tank => "CO2 Tank",
            EquipmentGroup::Keg => "Keg",
        }
    }
}

impl std::str::FromStr for EquipmentGroup {
    type Err = ();

    fn from_str(s: &str) -> Result<EquipmentGroup, ()> {
        match s {
            "Mash Tun" => Ok(EquipmentGroup::MashTun),
            "mashtun" => Ok(EquipmentGroup::MashTun),
            "Lauter Tun" => Ok(EquipmentGroup::LauterTun),
            "lautertun" => Ok(EquipmentGroup::LauterTun),
            "Hot Liquor Tank" => Ok(EquipmentGroup::HotLiquorTank),
            "hlt" => Ok(EquipmentGroup::HotLiquorTank),
            "Kettle" => Ok(EquipmentGroup::Kettle),
            "kettle" => Ok(EquipmentGroup::Kettle),
            "Fermentor" => Ok(EquipmentGroup::Fermentor),
            "fermentor" => Ok(EquipmentGroup::Fermentor),
            "CO2 Tank" => Ok(EquipmentGroup::CO2Tank),
            "gastank" => Ok(EquipmentGroup::CO2Tank),
            "Keg" => Ok(EquipmentGroup::Keg),
            "keg" => Ok(EquipmentGroup::Keg),

            _ => Err(()),
        }
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;

    pub fn mock_mash_tun() -> EquipmentGroup {
        EquipmentGroup::MashTun
    }

    pub fn mock_hot_liquor_tank() -> EquipmentGroup {
        EquipmentGroup::HotLiquorTank
    }

    pub fn mock_kettle() -> EquipmentGroup {
        EquipmentGroup::Kettle
    }

    pub fn mock_fermentor() -> EquipmentGroup {
        EquipmentGroup::Fermentor
    }

    pub fn mock_co2_tank() -> EquipmentGroup {
        EquipmentGroup::CO2Tank
    }

    pub fn mock_keg() -> EquipmentGroup {
        EquipmentGroup::Keg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equipmentgroup_lookup() {
        assert_eq!(EquipmentGroup::MashTun.lookup(), "Mash Tun");
        assert_eq!(EquipmentGroup::HotLiquorTank.lookup(), "Hot Liqoor Tank");
        assert_eq!(EquipmentGroup::Kettle.lookup(), "Kettle");
        assert_eq!(EquipmentGroup::Fermentor.lookup(), "Fermentor");
        assert_eq!(EquipmentGroup::CO2Tank.lookup(), "CO2 Tank");
        assert_eq!(EquipmentGroup::Keg.lookup(), "Keg");
    }

    #[test]
    fn test_equipmentgroup_parse() {
        assert_eq!("Mash Tun".parse(), Ok(EquipmentGroup::MashTun));
        assert_eq!("Lauter Tun".parse(), Ok(EquipmentGroup::LauterTun));
        assert_eq!("Hot Liquor Tank".parse(), Ok(EquipmentGroup::HotLiquorTank));
        assert_eq!("Kettle".parse(), Ok(EquipmentGroup::Kettle));
        assert_eq!("Fermentor".parse(), Ok(EquipmentGroup::Fermentor));
        assert_eq!("CO2 Tank".parse(), Ok(EquipmentGroup::CO2Tank));
        assert_eq!("Keg".parse(), Ok(EquipmentGroup::Keg));
    }
}
