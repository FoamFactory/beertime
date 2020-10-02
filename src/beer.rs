use crate::style::Style;

#[derive(Debug, PartialEq)]
pub struct Beer {
    name: String,
    style: Style,
}

impl Beer {
    pub fn new(name: &str, style: Style) -> Self {
        Self {
            name: name.to_string(),
            style,
        }
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use crate::style;

    pub fn beer() -> Beer {
        Beer::new("foobeer 2000", style::mock::pilsner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style;

    #[test]
    fn test_beer_new() {
        let beer = mock::beer();
        assert_eq!(&beer.name, "foobeer 2000");
        assert_eq!(beer.style, style::mock::pilsner());
    }
}
