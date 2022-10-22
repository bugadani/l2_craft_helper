use super::inventory::ItemList;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Recipe {
    pub ingredients: ItemList,
    pub result_count: usize,
}
