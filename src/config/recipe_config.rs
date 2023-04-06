#[derive(serde::Deserialize, Debug)]
pub struct RecipeConfig {
    pub name: String,
    pub batch_size: String,
}
