use crate::equipment_group::EquipmentGroup;
use crate::system::System;
use crate::volume::Volume;

#[derive(Debug, PartialEq)]
pub struct Equipment {
    id: String,
    system: System,
    equipment_group: EquipmentGroup,
    volume: Volume,
}

impl Equipment {
    pub fn new(
        id: String,
        system: System,
        equipment_group: EquipmentGroup,
        volume: Volume,
    ) -> Self {
        Self {
            id,
            system,
            equipment_group,
            volume,
        }
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
        assert_eq!(&equipment.id, "Foobar 2000");
        assert_eq!(equipment.system, system::mock::bbl5());
        assert_eq!(equipment.equipment_group, equipment_group::mock::mash_tun());
        assert_eq!(equipment.volume, volume::mock::gallon_us());
    }
}
