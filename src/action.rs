use crate::equipment::Equipment;

#[derive(Debug, PartialEq)]
pub enum Action {
    Process(Equipment),
    Clean(Equipment),
    Transfer(Equipment, Equipment),
}

impl Action {
    pub fn lookup(&self) -> String {
        match self {
            Action::Process(equipment) => format!("Process ({})", equipment.name),
            Action::Clean(equipment) => format!("Clean ({})", equipment.name),
            Action::Transfer(from, to) => format!("Transfer (from {} to {})", from.name, to.name),
        }
    }
    pub fn resources(&self) -> Vec<String> {
        match self {
            Action::Process(equipment) => vec![equipment.name.clone()],
            Action::Clean(equipment) => vec!["Cleaner".to_string(), equipment.name.clone()],
            Action::Transfer(from, to) => {
                vec!["Pumper".to_string(), from.name.clone(), to.name.clone()]
            }
        }
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use crate::equipment::Equipment;

    pub fn mock_process(equipment: Equipment) -> Action {
        Action::Process(equipment)
    }

    pub fn mock_clean(equipment: Equipment) -> Action {
        Action::Clean(equipment)
    }

    pub fn mock_transfer(equipment: Equipment, other: Equipment) -> Action {
        Action::Transfer(equipment, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capacity;
    use crate::equipment;
    use crate::equipment_group;
    use crate::volume;

    #[test]
    fn test_action_new() {
        let equipment = equipment::mock::mock_equipment();
        let action = mock::mock_clean(equipment.clone());
        assert_eq!(action, Action::Clean(equipment));
    }

    #[test]
    fn test_action_lookup() {
        let equipment_1 = equipment::mock::mock_equipment();
        let action_1 = mock::mock_clean(equipment_1.clone());
        assert_eq!(&action_1.lookup(), "Clean (Foobar 2000)");

        let action_2 = mock::mock_process(equipment_1.clone());
        assert_eq!(&action_2.lookup(), "Process (Foobar 2000)");

        let equipment_2 = Equipment::new(
            "Foobar 2001".to_string(),
            capacity::mock::mock_bbl5(),
            equipment_group::mock::mock_mash_tun(),
        );
        let action_3 = mock::mock_transfer(equipment_1.clone(), equipment_2);
        assert_eq!(
            &action_3.lookup(),
            "Transfer (from Foobar 2000 to Foobar 2001)"
        );
    }

    #[test]
    fn test_action_resources() {
        let equipment_1 = equipment::mock::mock_equipment();
        let action_1 = mock::mock_clean(equipment_1.clone());
        assert_eq!(
            action_1.resources(),
            vec!["Cleaner".to_string(), "Foobar 2000".to_string()]
        );

        let action_2 = mock::mock_process(equipment_1.clone());
        assert_eq!(action_2.resources(), vec!["Foobar 2000".to_string()]);

        let equipment_2 = Equipment::new(
            "Foobar 2001".to_string(),
            capacity::mock::mock_bbl5(),
            equipment_group::mock::mock_mash_tun(),
        );
        let action_3 = mock::mock_transfer(equipment_1.clone(), equipment_2);
        assert_eq!(
            action_3.resources(),
            vec![
                "Pumper".to_string(),
                "Foobar 2000".to_string(),
                "Foobar 2001".to_string()
            ]
        );
    }
}
