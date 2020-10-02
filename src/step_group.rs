#[derive(Debug, PartialEq)]
pub enum StepGroup {
    Brewing,
    PrimaryFermentation,
    DiactylRest,
    SecondaryFermentation,
    Aging,
    Carbonation,
}

impl StepGroup {
    pub fn lookup(&self) -> &'static str {
        match self {
            StepGroup::Aging => "Aging",
            StepGroup::Brewing => "Brewing",
            StepGroup::Carbonation => "Carbonation",
            StepGroup::DiactylRest => "Diactyl Rest",
            StepGroup::PrimaryFermentation => "Primary Fermentation",
            StepGroup::SecondaryFermentation => "Secondary Fermentation",
        }
    }
}
impl std::str::FromStr for StepGroup {
    type Err = ();

    fn from_str(s: &str) -> Result<StepGroup, ()> {
        match s {
            "Aging" => Ok(StepGroup::Aging),
            "Brewing" => Ok(StepGroup::Brewing),
            "Carbonation" => Ok(StepGroup::Carbonation),
            "Diactyl Rest" => Ok(StepGroup::DiactylRest),
            "Primary Fermentation" => Ok(StepGroup::PrimaryFermentation),
            "Secondary Fermentation" => Ok(StepGroup::SecondaryFermentation),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_lookup() {
        assert_eq!(StepGroup::Aging.lookup(), "Aging");
        assert_eq!(StepGroup::Brewing.lookup(), "Brewing");
        assert_eq!(StepGroup::Carbonation.lookup(), "Carbonation");
        assert_eq!(StepGroup::DiactylRest.lookup(), "Diactyl Rest");
        assert_eq!(
            StepGroup::PrimaryFermentation.lookup(),
            "Primary Fermentation"
        );
        assert_eq!(
            StepGroup::SecondaryFermentation.lookup(),
            "Secondary Fermentation"
        );
    }
    #[test]

    fn test_group_parse() {
        assert_eq!("Aging".parse(), Ok(StepGroup::Aging));
        assert_eq!("Brewing".parse(), Ok(StepGroup::Brewing));
        assert_eq!("Carbonation".parse(), Ok(StepGroup::Carbonation));
        assert_eq!("Diactyl Rest".parse(), Ok(StepGroup::DiactylRest));
        assert_eq!(
            "Primary Fermentation".parse(),
            Ok(StepGroup::PrimaryFermentation)
        );
        assert_eq!(
            "Secondary Fermentation".parse(),
            Ok(StepGroup::SecondaryFermentation)
        );
    }
}
