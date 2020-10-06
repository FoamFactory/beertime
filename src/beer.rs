use serde::Serialize;

use crate::recipy::Recipy;
use crate::style::Style;

#[derive(Debug, PartialEq, Serialize)]
pub struct Beer {
    pub name: String,
    style: Style,
    pub recipy: Recipy,
}

impl Beer {
    pub fn new(name: &str, style: Style, recipy: Recipy) -> Self {
        Self {
            name: name.to_string(),
            style,
            recipy,
        }
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use crate::recipy;
    use crate::style;

    pub fn beer() -> Beer {
        Beer::new(
            "foobeer 2000",
            style::mock::pilsner(),
            recipy::mock::recipy(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recipy;
    use crate::style;

    #[test]
    fn test_beer_new() {
        let beer = mock::beer();
        assert_eq!(&beer.name, "foobeer 2000");
        assert_eq!(beer.style, style::mock::pilsner());
        assert_eq!(beer.recipy, recipy::mock::recipy());
    }
}
