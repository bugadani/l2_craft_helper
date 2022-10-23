use database::RecipeDatabase;
use inventory::{Inventory, ItemList};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod database;
pub mod inventory;
pub mod recipe;

#[derive(Deserialize, Serialize)]
pub struct Input {
    recipes: ItemList,
    inventory: Inventory,
}

fn read_file<T: DeserializeOwned>(file: &str) -> T {
    let file_reader = std::fs::File::open(file).expect("Failed to open database file");
    serde_json::from_reader(file_reader).expect("Failed to parse database file")
}

fn main() {
    let database: RecipeDatabase = read_file("database.json");
    let mut input: Input = read_file("input.json");

    let recipe = input.recipes.clone();
    match input.inventory.craft(recipe, &database) {
        Ok(_) => println!("You have enough materials"),
        Err(missing) => {
            println!("You need the following materials:");
            for (material, amount) in missing {
                println!(" - {amount} x {material}");
            }
        }
    }
}
