use std::str::FromStr;
use crate::equipment_group::EquipmentGroup;
use crate::capacity::Capacity;
use crate::volume::Volume;
use crate::config::EquipmentConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct Equipment {
    pub name: String,
    pub system: Capacity,
    pub equipment_group: EquipmentGroup,
    pub volume: Volume,
}

impl Equipment {
    pub fn new(
        name: String,
        system: Capacity,
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

impl std::convert::From<&EquipmentConfig> for Equipment {
    fn from(config: &EquipmentConfig) -> Self {
        let equipment_type = match EquipmentGroup::from_str(&config.equipment_type) {
            Ok(x) => x,
            Err(_e) => panic!("Unable to convert from configuration string to equipment group"),
        };

        let capacity_vol = match Volume::from_str(&config.capacity) {
            Ok(x) => x,
            Err(_e) => panic!("{} does not appear to be a valid volume for capacity", &config.capacity),
        };

        Equipment::new(
            String::from(&config.name),
            Capacity::G10,
            equipment_type,
            capacity_vol,
        )
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use crate::equipment_group;
    use crate::capacity;
    use crate::volume;

    pub fn equipment() -> Equipment {
        Equipment::new(
            "Foobar 2000".to_string(),
            capacity::mock::bbl5(),
            equipment_group::mock::mash_tun(),
            volume::mock::gallon_us(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::equipment_group;
    use crate::capacity;
    use crate::volume;

    #[test]
    fn test_equimpment_new() {
        let equipment = mock::equipment();
        assert_eq!(&equipment.name, "Foobar 2000");
        assert_eq!(equipment.system, capacity::mock::bbl5());
        assert_eq!(equipment.equipment_group, equipment_group::mock::mash_tun());
        assert_eq!(equipment.volume, volume::mock::gallon_us());
    }

    #[test]
    fn test_equimpment_fits() {
        let equipment = mock::equipment();
        assert_eq!(equipment.can_hold(&Volume::GallonUS(12.3)), false);
        assert_eq!(equipment.can_hold(&Volume::GallonUS(5.0)), true);
    }
}
