use crate::equipment::Equipment;

#[derive(Debug, PartialEq)]
pub enum Action {
    Process(Equipment),
    Clean(Equipment),
    Transfer((Equipment, Equipment)),
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use crate::equipment;
    use crate::equipment_group;
    use crate::system;
    use crate::volume;

    pub fn process() -> Action {
        Action::Process(equipment::mock::equipment())
    }

    pub fn clean() -> Action {
        Action::Clean(equipment::mock::equipment())
    }

    pub fn transfer() -> Action {
        let other = Equipment::new(
            "Foobar 2001".to_string(),
            system::mock::bbl5(),
            equipment_group::mock::mash_tun(),
            volume::mock::gallon_us(),
        );
        Action::Transfer((equipment::mock::equipment(), other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_new() {
        let action = mock::clean();
        // TODO: better tests
        assert_eq!(1 + 3, 4);
    }
}
