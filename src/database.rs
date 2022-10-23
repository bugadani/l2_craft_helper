use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::recipe::Recipe;

#[derive(Deserialize, Serialize)]
pub struct RecipeDatabase {
    #[serde(flatten)]
    pub recipes: HashMap<String, Recipe>,
}

impl RecipeDatabase {
    pub fn new() -> Self {
        Self {
            recipes: HashMap::new()
        }
    }
    
    pub fn add(&mut self, recipes: RecipeDatabase) {
        self.recipes.extend(recipes.recipes.into_iter());
    }
}