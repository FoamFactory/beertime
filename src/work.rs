#[derive(Debug, PartialEq)]
pub enum Work {
    Transfer,
    Clean,
}

impl Work {
    pub fn lookup(&self) -> &'static str {
        match self {
            Work::Transfer => "Transfer",
            Work::Clean => "Clean",
        }
    }
}
impl std::str::FromStr for Work {
    type Err = ();

    fn from_str(s: &str) -> Result<Work, ()> {
        match s {
            "Transfer" => Ok(Work::Transfer),
            "Clean" => Ok(Work::Clean),
            _ => Err(()),
        }
    }
}
#[cfg(test)]
pub mod mock {
    use super::*;

    pub fn transfer() -> Work {
        Work::Transfer
    }

    pub fn clean() -> Work {
        Work::Clean
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_work_lookup() {
        assert_eq!(Work::Transfer.lookup(), "Transfer");
        assert_eq!(Work::Clean.lookup(), "Clean");
    }

    #[test]
    fn test_work_parse() {
        assert_eq!("Transfer".parse(), Ok(Work::Transfer));
        assert_eq!("Clean".parse(), Ok(Work::Clean));
    }
}
