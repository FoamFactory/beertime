use crate::equipment::Equipment;

pub struct Factory {
    pub name: String,
    pub equipment: Vec<Equipment>,
}

impl Factory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            equipment: Vec::new(),
        }
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use crate::equipment;

    pub fn factory() -> Factory {
        let mut factory = Factory::new("loonslanding");
        factory.equipment.push(equipment::mock::equipment());

        factory
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_new() {
        let factory = mock::factory();
        assert_eq!(&factory.name, "loonslanding");
        assert_eq!(factory.equipment.len(), 1);
    }
}
