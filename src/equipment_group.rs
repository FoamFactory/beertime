#[derive(Debug, PartialEq)]
pub enum EquipmentGroup {
    MashTun,
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
            "Hot Liqoor Tank" => Ok(EquipmentGroup::HotLiquorTank),
            "Kettle" => Ok(EquipmentGroup::Kettle),
            "Fermentor" => Ok(EquipmentGroup::Fermentor),
            "CO2 Tank" => Ok(EquipmentGroup::CO2Tank),
            "Keg" => Ok(EquipmentGroup::Keg),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equipmentgroupstyle_lookup() {
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
        assert_eq!("Hot Liqoor Tank".parse(), Ok(EquipmentGroup::HotLiquorTank));
        assert_eq!("Kettle".parse(), Ok(EquipmentGroup::Kettle));
        assert_eq!("Fermentor".parse(), Ok(EquipmentGroup::Fermentor));
        assert_eq!("CO2 Tank".parse(), Ok(EquipmentGroup::CO2Tank));
        assert_eq!("Keg".parse(), Ok(EquipmentGroup::Keg));
    }
}
