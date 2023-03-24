use std::str::FromStr;
use crate::equipment_group::EquipmentGroup;
use crate::capacity::Capacity;
use crate::volume::Volume;
use crate::config::EquipmentConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct Equipment {
    pub name: String,
    pub capacity: Capacity,
    pub equipment_group: EquipmentGroup,
}

impl Equipment {
    pub fn new(
        name: String,
        capacity: Capacity,
        equipment_group: EquipmentGroup,
    ) -> Self {
        Self {
            name,
            capacity,
            equipment_group,
        }
    }

    pub fn can_hold(&self, volume: &Volume) -> bool {
        if let Volume::Liter(this) = self.capacity.volume().to_liter() {
            if let Volume::Liter(that) = volume.to_liter() {
                return this >= that;
            }
        }

        panic!("Something went wonky when trying to convert volumes");
    }
}

impl std::convert::From<&EquipmentConfig> for Equipment {
    fn from(config: &EquipmentConfig) -> Self {
        let equipment_type = match EquipmentGroup::from_str(&config.equipment_type) {
            Ok(x) => x,
            Err(_e) => panic!("Unable to convert from configuration string to equipment group"),
        };

        // let capacity_vol = match Volume::from_str(&config.capacity) {
        //     Ok(x) => x,
        //     Err(_e) => panic!("{} does not appear to be a valid volume for capacity", &config.capacity),
        // };

        Equipment::new(
            String::from(&config.name),
            Capacity::from_str(&config.capacity).unwrap(),
            equipment_type
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
            capacity::mock::g14(),
            equipment_group::mock::mash_tun(),
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
    #[ignore]
    fn test_equimpment_new() {
        let equipment = mock::equipment();
        assert_eq!(&equipment.name, "Foobar 2000");
        assert_eq!(equipment.capacity, capacity::mock::g14());
        assert_eq!(equipment.equipment_group, equipment_group::mock::mash_tun());
        // assert_eq!(equipment.volume, volume::mock::gallon_us());
    }

    #[test]
    #[ignore]
    fn test_equimpment_fits() {
        let equipment = mock::equipment();
        assert_eq!(equipment.can_hold(&Volume::GallonUS(14.9)), false);
        assert_eq!(equipment.can_hold(&Volume::GallonUS(5.0)), true);
    }
}
