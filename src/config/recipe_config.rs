#[derive(serde::Deserialize)]
pub struct RecipeConfig {
    pub name: String,
    pub batch_size: String,
}
