// #[cfg(test)]
// pub mod mock {
//     use super::*;
//     use crate::steps;
//     use crate::capacity;
//     use crate::volume;

//     pub fn Recipe() -> Recipe {
//         Recipe::new(
//             capacity::mock::g5(),
//             volume::mock::gallon_us(),
//             steps::mock::steps(),
//         )
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::steps;
//     use crate::capacity;
//     use crate::volume;

//     #[test]
//     #[ignore]
//     fn test_recipe_new() {
//         let recipe = mock::Recipe();
//         assert_eq!(
//             recipe.get(&capacity::mock::g5()),
//             Some(&(volume::mock::gallon_us(), steps::mock::steps()))
//         );
//     }
// }