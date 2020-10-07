#[derive(Debug, PartialEq)]
pub enum Type {
    Lager,
    Ale,
}

impl Type {
    pub fn needs_diactyl_rest(&self) -> bool {
        self == &Type::Lager
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_type_diactyl_rest() {
        assert_eq!(Type::Ale.needs_diactyl_rest(), false);
        assert_eq!(Type::Lager.needs_diactyl_rest(), true);
    }
}
