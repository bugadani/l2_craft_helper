use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::recipe::Recipe;

#[derive(Deserialize, Serialize)]
pub struct RecipeDatabase {
    pub recipes: HashMap<String, Recipe>,
}
