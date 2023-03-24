use chrono::Duration;

use crate::equipment_group::EquipmentGroup;
use crate::capacity::Capacity;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum StepGroup {
    Brewing,
    PrimaryFermentation,
    DiacetylRest,
    SecondaryFermentation,
    Aging,
    Carbonation,
}

impl StepGroup {
    pub fn lookup(&self) -> &'static str {
        match self {
            StepGroup::Aging => "Aging",
            StepGroup::Brewing => "Brewing",
            StepGroup::Carbonation => "Carbonation",
            StepGroup::DiacetylRest => "Diacetyl Rest",
            StepGroup::PrimaryFermentation => "Primary Fermentation",
            StepGroup::SecondaryFermentation => "Secondary Fermentation",
        }
    }
    pub fn all() -> Vec<StepGroup> {
        vec![
            StepGroup::Brewing,
            StepGroup::PrimaryFermentation,
            StepGroup::DiacetylRest,
            StepGroup::SecondaryFermentation,
            StepGroup::Aging,
            StepGroup::Carbonation,
        ]
    }

    pub fn equipment_group(&self) -> EquipmentGroup {
        match self {
            StepGroup::Aging => EquipmentGroup::Keg,
            //StepGroup::Brewing => unimplemented!(), //EquipmentGroup::HotLiquorTank EquipmentGroup::MashTun,
            StepGroup::Brewing => EquipmentGroup::MashTun,
            StepGroup::Carbonation => EquipmentGroup::CO2Tank,
            StepGroup::DiacetylRest => EquipmentGroup::Fermentor,
            StepGroup::PrimaryFermentation => EquipmentGroup::Fermentor,
            StepGroup::SecondaryFermentation => EquipmentGroup::Fermentor,
        }
    }

    pub fn post_process_time(&self, system_capacity: &Capacity) -> Duration {
        // @TODO: this is all made up, get some more sensable magic numbers
        let factor = match system_capacity {
            Capacity::G5 | Capacity::G10 | Capacity::G14 | Capacity::G15 => 1,
            Capacity::BBL5 => 2,
            Capacity::BBL7 => 5,
            Capacity::BBL10 => 10,
            Capacity::BBL15 => 20,
            Capacity::UNKNOWN => 1000000,
        };
        let dur = match self {
            StepGroup::Aging => Duration::minutes(2),
            StepGroup::Brewing => Duration::minutes(5),
            StepGroup::Carbonation => Duration::minutes(1),
            StepGroup::DiacetylRest
            | StepGroup::PrimaryFermentation
            | StepGroup::SecondaryFermentation => Duration::minutes(10),
        };
        dur * factor
    }
}

impl std::str::FromStr for StepGroup {
    type Err = ();

    fn from_str(s: &str) -> Result<StepGroup, ()> {
        match s {
            "Aging" => Ok(StepGroup::Aging),
            "Brewing" => Ok(StepGroup::Brewing),
            "Carbonation" => Ok(StepGroup::Carbonation),
            "Diacetyl Rest" => Ok(StepGroup::DiacetylRest),
            "Primary Fermentation" => Ok(StepGroup::PrimaryFermentation),
            "Secondary Fermentation" => Ok(StepGroup::SecondaryFermentation),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    pub fn aging() -> StepGroup {
        StepGroup::Aging
    }

    pub fn brewing() -> StepGroup {
        StepGroup::Brewing
    }

    pub fn carbonation() -> StepGroup {
        StepGroup::Carbonation
    }

    pub fn diacetyl_rest() -> StepGroup {
        StepGroup::DiacetylRest
    }

    pub fn primary_fermentation() -> StepGroup {
        StepGroup::PrimaryFermentation
    }

    pub fn secondary_fermentation() -> StepGroup {
        StepGroup::SecondaryFermentation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_stepgroup_lookup() {
        assert_eq!(StepGroup::Aging.lookup(), "Aging");
        assert_eq!(StepGroup::Brewing.lookup(), "Brewing");
        assert_eq!(StepGroup::Carbonation.lookup(), "Carbonation");
        assert_eq!(StepGroup::DiacetylRest.lookup(), "Diacetyl Rest");
        assert_eq!(
            StepGroup::PrimaryFermentation.lookup(),
            "Primary Fermentation"
        );
        assert_eq!(
            StepGroup::SecondaryFermentation.lookup(),
            "Secondary Fermentation"
        );
    }

    #[test]
    #[ignore]
    fn test_stepgroup_equipment_group() {
        assert_eq!(StepGroup::Aging.equipment_group(), EquipmentGroup::Keg);
        assert_eq!(
            StepGroup::Brewing.equipment_group(),
            EquipmentGroup::MashTun
        );
        assert_eq!(
            StepGroup::Carbonation.equipment_group(),
            EquipmentGroup::CO2Tank
        );
        assert_eq!(
            StepGroup::DiacetylRest.equipment_group(),
            EquipmentGroup::Fermentor
        );
        assert_eq!(
            StepGroup::PrimaryFermentation.equipment_group(),
            EquipmentGroup::Fermentor
        );
        assert_eq!(
            StepGroup::SecondaryFermentation.equipment_group(),
            EquipmentGroup::Fermentor
        );
    }

    #[test]
    #[ignore]
    fn test_stepgroup_parse() {
        assert_eq!("Aging".parse(), Ok(StepGroup::Aging));
        assert_eq!("Brewing".parse(), Ok(StepGroup::Brewing));
        assert_eq!("Carbonation".parse(), Ok(StepGroup::Carbonation));
        assert_eq!("Diacetyl Rest".parse(), Ok(StepGroup::DiacetylRest));
        assert_eq!(
            "Primary Fermentation".parse(),
            Ok(StepGroup::PrimaryFermentation)
        );
        assert_eq!(
            "Secondary Fermentation".parse(),
            Ok(StepGroup::SecondaryFermentation)
        );
    }

    #[test]
    #[ignore]
    fn test_stepgroup_all() {
        assert_eq!(StepGroup::all().len(), 6);
    }

    #[test]
    #[ignore]
    fn test_stepgroup_post_process_time() {
        assert_eq!(
            StepGroup::Aging.post_process_time(&Capacity::G5),
            Duration::minutes(2)
        );
        assert_eq!(
            StepGroup::Aging.post_process_time(&Capacity::G10),
            Duration::minutes(2)
        );
        assert_eq!(
            StepGroup::Aging.post_process_time(&Capacity::BBL5),
            Duration::minutes(4)
        );
        assert_eq!(
            StepGroup::Aging.post_process_time(&Capacity::BBL7),
            Duration::minutes(10)
        );
        assert_eq!(
            StepGroup::Aging.post_process_time(&Capacity::BBL10),
            Duration::minutes(20)
        );
        assert_eq!(
            StepGroup::Aging.post_process_time(&Capacity::BBL15),
            Duration::minutes(40)
        );
    }
}
