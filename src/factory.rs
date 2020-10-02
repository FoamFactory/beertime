use crate::beer::Beer;
use crate::equipment::Equipment;

#[derive(Debug, PartialEq)]
pub struct Factory {
    pub name: String,
    pub equipments: Vec<Equipment>,
    pub beers: Vec<Beer>,
}

impl Factory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            equipments: Vec::new(),
            beers: Vec::new(),
        }
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use crate::beer;
    use crate::equipment;

    pub fn factory() -> Factory {
        let mut factory = Factory::new("loonslanding");
        factory.equipments.push(equipment::mock::equipment());
        factory.beers.push(beer::mock::beer());

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
        assert_eq!(factory.equipments.len(), 1);
        assert_eq!(factory.beers.len(), 1);
    }
}
