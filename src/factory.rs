pub struct Factory {
    pub name: String,
}

impl Factory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;

    pub fn factory() -> Factory {
        Factory::new("loonslanding")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_new() {
        let factory = mock::factory();
        assert_eq!(&factory.name, "loonslanding");
    }
}
