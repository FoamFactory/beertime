use crate::equipment::Equipment;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum Action<'a> {
    Process(&'a Equipment),
    Clean(&'a Equipment),
    Transfer(&'a Equipment, &'a Equipment),
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

    #[test]
    fn test_action_new() {
        let equipment = equipment::mock::equipment();
        let action = mock::clean(&equipment);
        assert_eq!(action, Action::Clean(&equipment));
    }
}
