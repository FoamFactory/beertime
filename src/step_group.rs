use chrono::Duration;
use serde::Serialize;

use crate::equipment_group::EquipmentGroup;
use crate::system::System;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub enum StepGroup {
    Brewing,
    PrimaryFermentation,
    DiactylRest,
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
            StepGroup::DiactylRest => "Diactyl Rest",
            StepGroup::PrimaryFermentation => "Primary Fermentation",
            StepGroup::SecondaryFermentation => "Secondary Fermentation",
        }
    }

    pub fn equipment_group(&self) -> EquipmentGroup {
        match self {
            StepGroup::Aging => EquipmentGroup::Keg,
            //StepGroup::Brewing => unimplemented!(), //EquipmentGroup::HotLiquorTank EquipmentGroup::MashTun,
            StepGroup::Brewing => EquipmentGroup::HotLiquorTank, //EquipmentGroup::MashTun,
            StepGroup::Carbonation => EquipmentGroup::CO2Tank,
            StepGroup::DiactylRest => EquipmentGroup::Fermentor,
            StepGroup::PrimaryFermentation => EquipmentGroup::Fermentor,
            StepGroup::SecondaryFermentation => EquipmentGroup::Fermentor,
        }
    }

    pub fn post_process_time(&self, system: &System) -> Duration {
        // @TODO: this is all made up, get some more sensable magic numbers
        let factor = match system {
            System::G5 | System::G10 => 1,
            System::BBL5 => 2,
            System::BBL7 => 5,
            System::BBL10 => 10,
            System::BBL15 => 20,
        };
        let dur = match self {
            StepGroup::Aging => Duration::minutes(2),
            StepGroup::Brewing => Duration::minutes(5),
            StepGroup::Carbonation => Duration::minutes(1),
            StepGroup::DiactylRest
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
            "Diactyl Rest" => Ok(StepGroup::DiactylRest),
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

    pub fn diactyl_rest() -> StepGroup {
        StepGroup::DiactylRest
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
    fn test_group_lookup() {
        assert_eq!(StepGroup::Aging.lookup(), "Aging");
        assert_eq!(StepGroup::Brewing.lookup(), "Brewing");
        assert_eq!(StepGroup::Carbonation.lookup(), "Carbonation");
        assert_eq!(StepGroup::DiactylRest.lookup(), "Diactyl Rest");
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
    fn test_group_equipment_group() {
        assert_eq!(StepGroup::Aging.equipment_group(), EquipmentGroup::Keg);
        assert_eq!(
            StepGroup::Brewing.equipment_group(),
            EquipmentGroup::HotLiquorTank
        );
        assert_eq!(
            StepGroup::Carbonation.equipment_group(),
            EquipmentGroup::CO2Tank
        );
        assert_eq!(
            StepGroup::DiactylRest.equipment_group(),
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
    fn test_group_parse() {
        assert_eq!("Aging".parse(), Ok(StepGroup::Aging));
        assert_eq!("Brewing".parse(), Ok(StepGroup::Brewing));
        assert_eq!("Carbonation".parse(), Ok(StepGroup::Carbonation));
        assert_eq!("Diactyl Rest".parse(), Ok(StepGroup::DiactylRest));
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
    fn test_group_post_process_time() {
        assert_eq!(
            StepGroup::Aging.post_process_time(&System::G5),
            Duration::minutes(2)
        );
        assert_eq!(
            StepGroup::Aging.post_process_time(&System::G10),
            Duration::minutes(2)
        );
        assert_eq!(
            StepGroup::Aging.post_process_time(&System::BBL5),
            Duration::minutes(4)
        );
        assert_eq!(
            StepGroup::Aging.post_process_time(&System::BBL7),
            Duration::minutes(10)
        );
        assert_eq!(
            StepGroup::Aging.post_process_time(&System::BBL10),
            Duration::minutes(20)
        );
        assert_eq!(
            StepGroup::Aging.post_process_time(&System::BBL15),
            Duration::minutes(40)
        );
    }
}
