use serde::Serialize;

use crate::equipment_group::EquipmentGroup;
use crate::system::System;
use crate::volume::Volume;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Equipment {
    pub name: String,
    pub system: System,
    pub equipment_group: EquipmentGroup,
    volume: Volume,
}

impl Equipment {
    pub fn new(
        name: String,
        system: System,
        equipment_group: EquipmentGroup,
        volume: Volume,
    ) -> Self {
        Self {
            name,
            system,
            equipment_group,
            volume,
        }
    }

    pub fn can_hold(&self, volume: &Volume) -> bool {
        if let Volume::Liter(this) = self.volume.to_liter() {
            if let Volume::Liter(that) = volume.to_liter() {
                return this >= that;
            }
        }
        panic!("should not happen");
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use crate::equipment_group;
    use crate::system;
    use crate::volume;

    pub fn equipment() -> Equipment {
        Equipment::new(
            "Foobar 2000".to_string(),
            system::mock::bbl5(),
            equipment_group::mock::mash_tun(),
            volume::mock::gallon_us(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::equipment_group;
    use crate::system;
    use crate::volume;

    #[test]
    fn test_equimpment_new() {
        let equipment = mock::equipment();
        assert_eq!(&equipment.name, "Foobar 2000");
        assert_eq!(equipment.system, system::mock::bbl5());
        assert_eq!(equipment.equipment_group, equipment_group::mock::mash_tun());
        assert_eq!(equipment.volume, volume::mock::gallon_us());
    }

    #[test]
    fn test_equimpment_fits() {
        let equipment = mock::equipment();
        assert_eq!(equipment.can_hold(&Volume::GallonUS(12.3)), false);
        assert_eq!(equipment.can_hold(&Volume::GallonUS(12.2)), true);
    }
}
