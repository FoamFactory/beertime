use crate::config::{FactoryConfig, RecipeConfig};
use crate::recipe::Recipe;
use crate::style;
use crate::style::Style;

#[derive(Debug, PartialEq)]
pub struct Beer {
    pub name: String,
    pub style: Style,
    pub recipe: Recipe,
}

impl Beer {
    pub fn new(name: String, style: Style, recipe: Recipe) -> Self {
        let beer_type = style.r#type();
        let needs_rest = beer_type.needs_diacetyl_rest();
        for (_volume, steps) in recipe.map.values() {
            assert_eq!(steps.needs_diacetyl_rest(), needs_rest);
        }

        Self {
            name,
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
            "foobeer 2000".to_string(),
            style::mock::blonde_ale(),
            recipe::mock::Recipe(),
        )
    }
}

impl From<(&FactoryConfig, &RecipeConfig)> for Beer {
    fn from(config: (&FactoryConfig, &RecipeConfig)) -> Self {
        let (factory_config, recipe_config) = config;
        let cloned_str = String::from(&recipe_config.name);
        Beer::new(
            cloned_str,
            Style::BlondeAle,
            Recipe::from((factory_config, recipe_config))
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
