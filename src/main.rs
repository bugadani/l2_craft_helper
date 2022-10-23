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

fn read_database() -> RecipeDatabase {
    let mut database = RecipeDatabase::new();

    database.add(read_file("database/materials.json"));
    database.add(read_file("database/common/common.json"));
    database.add(read_file("database/common/accessories.json"));
    database.add(read_file("database/common/elixir.json"));
    database.add(read_file("database/common/dye.json"));
    
    for category in ["weapons", "jewels", "armor"] {
        for grade in ["ng", "d", "c", "b", "a"] {
            database.add(read_file(&format!("database/{category}/{grade}.json")));
        }
    }

    database
}

fn main() {
    let database = read_database();
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
