use crate::recipe::Recipe;
use crate::style::Style;

#[derive(Debug, PartialEq)]
pub struct Beer {
    pub name: String,
    pub style: Style,
    pub recipe: Recipe,
}

impl Beer {
    pub fn new(name: &str, style: Style, recipe: Recipe) -> Self {
        let beer_type = style.r#type();
        let needs_rest = beer_type.needs_diacetyl_rest();
        for (_volume, steps) in recipe.map.values() {
            assert_eq!(steps.needs_diacetyl_rest(), needs_rest);
        }

        Self {
            name: name.to_string(),
            style,
            recipe,
        }
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use crate::recipe;
    use crate::style;

    pub fn beer() -> Beer {
        Beer::new(
            "foobeer 2000",
            style::mock::blonde_ale(),
            recipe::mock::Recipe(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recipe;
    use crate::style;

    #[test]
    fn test_beer_new() {
        let beer = mock::beer();
        assert_eq!(&beer.name, "foobeer 2000");
        assert_eq!(beer.style, style::mock::blonde_ale());
        assert_eq!(beer.recipe, recipe::mock::Recipe());
    }
}
