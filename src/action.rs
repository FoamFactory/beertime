use crate::equipment::Equipment;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum Action<'a> {
    Process(&'a Equipment),
    Clean(&'a Equipment),
    Transfer(&'a Equipment, &'a Equipment),
}

impl<'a> Action<'a> {
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

    pub fn process<'a>(equipment: &'a Equipment) -> Action<'a> {
        Action::Process(equipment)
    }

    pub fn clean<'a>(equipment: &'a Equipment) -> Action<'a> {
        Action::Clean(equipment)
    }

    pub fn transfer<'a>(equipment: &'a Equipment, other: &'a Equipment) -> Action<'a> {
        Action::Transfer(equipment, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::equipment;
    use crate::equipment_group;
    use crate::system;
    use crate::volume;

    #[test]
    fn test_action_new() {
        let equipment = equipment::mock::equipment();
        let action = mock::clean(&equipment);
        assert_eq!(action, Action::Clean(&equipment));
    }

    #[test]
    fn test_action_lookup() {
        let equipment_1 = equipment::mock::equipment();
        let action_1 = mock::clean(&equipment_1);
        assert_eq!(&action_1.lookup(), "Clean (Foobar 2000)");

        let action_2 = mock::process(&equipment_1);
        assert_eq!(&action_2.lookup(), "Process (Foobar 2000)");

        let equipment_2 = Equipment::new(
            "Foobar 2001".to_string(),
            system::mock::bbl5(),
            equipment_group::mock::mash_tun(),
            volume::mock::gallon_us(),
        );
        let action_3 = mock::transfer(&equipment_1, &equipment_2);
        assert_eq!(
            &action_3.lookup(),
            "Transfer (from Foobar 2000 to Foobar 2001)"
        );
    }

    #[test]
    fn test_action_resources() {
        let equipment_1 = equipment::mock::equipment();
        let action_1 = mock::clean(&equipment_1);
        assert_eq!(
            action_1.resources(),
            vec!["Cleaner".to_string(), "Foobar 2000".to_string()]
        );

        let action_2 = mock::process(&equipment_1);
        assert_eq!(action_2.resources(), vec!["Foobar 2000".to_string()]);

        let equipment_2 = Equipment::new(
            "Foobar 2001".to_string(),
            system::mock::bbl5(),
            equipment_group::mock::mash_tun(),
            volume::mock::gallon_us(),
        );
        let action_3 = mock::transfer(&equipment_1, &equipment_2);
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
