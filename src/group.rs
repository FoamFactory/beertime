#[derive(Debug, PartialEq)]
pub enum Group {
    Brewing,
    PrimaryFermentation,
    DiactylRest,
    SecondaryFermentation,
    Aging,
    Carbonation,
}

impl Group {
    pub fn lookup(&self) -> &'static str {
        match self {
            Group::Aging => "Aging",
            Group::Brewing => "Brewing",
            Group::Carbonation => "Carbonation",
            Group::DiactylRest => "Diactyl Rest",
            Group::PrimaryFermentation => "Primary Fermentation",
            Group::SecondaryFermentation => "Secondary Fermentation",
        }
    }
}
impl std::str::FromStr for Group {
    type Err = ();

    fn from_str(s: &str) -> Result<Group, ()> {
        match s {
            "Aging" => Ok(Group::Aging),
            "Brewing" => Ok(Group::Brewing),
            "Carbonation" => Ok(Group::Carbonation),
            "Diactyl Rest" => Ok(Group::DiactylRest),
            "Primary Fermentation" => Ok(Group::PrimaryFermentation),
            "Secondary Fermentation" => Ok(Group::SecondaryFermentation),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_lookup() {
        assert_eq!(Group::Aging.lookup(), "Aging");
        assert_eq!(Group::Brewing.lookup(), "Brewing");
        assert_eq!(Group::Carbonation.lookup(), "Carbonation");
        assert_eq!(Group::DiactylRest.lookup(), "Diactyl Rest");
        assert_eq!(Group::PrimaryFermentation.lookup(), "Primary Fermentation");
        assert_eq!(
            Group::SecondaryFermentation.lookup(),
            "Secondary Fermentation"
        );
    }
    #[test]

    fn test_group_parse() {
        assert_eq!("Aging".parse(), Ok(Group::Aging));
        assert_eq!("Brewing".parse(), Ok(Group::Brewing));
        assert_eq!("Carbonation".parse(), Ok(Group::Carbonation));
        assert_eq!("Diactyl Rest".parse(), Ok(Group::DiactylRest));
        assert_eq!(
            "Primary Fermentation".parse(),
            Ok(Group::PrimaryFermentation)
        );
        assert_eq!(
            "Secondary Fermentation".parse(),
            Ok(Group::SecondaryFermentation)
        );
    }
}
