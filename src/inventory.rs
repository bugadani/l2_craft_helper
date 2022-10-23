use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::database::RecipeDatabase;

pub type ItemList = HashMap<String, usize>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Inventory {
    pub items: ItemList,
}

impl Inventory {
    pub fn take(&mut self, item: &str, count: usize) -> Result<(), usize> {
        match self.items.get_mut(item) {
            Some(amount) => {
                if *amount >= count {
                    *amount -= count;
                    Ok(())
                } else {
                    Err(*amount)
                }
            }

            None => Err(0),
        }
    }

    pub fn add(&mut self, item: &str, count: usize) {
        let amount = self.items.entry(item.to_string()).or_insert(0);
        *amount += count;
    }

    /// Tries to craft a recipe.
    /// Returns the new inventory on success and the missing items on failure.
    pub fn craft(&mut self, recipe: ItemList, recipes: &RecipeDatabase) -> Result<(), ItemList> {
        // Clone inventory because we will be modifying it before making sure we have everything for the recipe.
        let original_items = self.items.clone();

        // The working list of ingredients.
        let mut missing_items = ItemList::new();

        for (item, &amount) in recipe.iter() {
            for (ingredient, &ing_amount) in &recipes.recipes[item].ingredients {
                missing_items.insert(ingredient.clone(), ing_amount * amount);
            }
        }

        // Work gradually towards less and less complex ingredients.

        let mut run_next_iteration = true;
        while !missing_items.is_empty() && run_next_iteration {
            let mut missing_items_after = ItemList::new();

            run_next_iteration = false;

            // - try and take all ingredients
            for (item, amount) in missing_items {
                if let Err(present) = self.take(&item, amount) {
                    // println!("Inventory contains {present} {item} instead of {amount}");
                    // We have some of the item, remove that from the inventory.
                    let _ = self.take(&item, present);

                    // - break down remaining ingredients into subcomponents if possible
                    let subcomponents = if let Some(recipe) = recipes.recipes.get(&item) {
                        &recipe.ingredients
                    } else {
                        // println!("{item} has no recipe, adding back missing amount");
                        *missing_items_after.entry(item.clone()).or_insert(0) += amount - present;
                        continue;
                    };

                    if subcomponents.is_empty() {
                        // println!("{item} has no components, adding back missing amount");
                        *missing_items_after.entry(item.clone()).or_insert(0) += amount - present;
                        continue;
                    }

                    for (component, count) in subcomponents.iter() {
                        // println!("Adding {count} x {component} from {item}");
                        run_next_iteration = true;
                        *missing_items_after.entry(component.clone()).or_insert(0) +=
                            count * (amount - present);
                    }
                }
            }

            missing_items = missing_items_after;
        }

        if missing_items.is_empty() {
            for (item, amount) in recipe {
                self.add(&item, amount * recipes.recipes[&item].result_count);
            }
            Ok(())
        } else {
            self.items = original_items;
            Err(missing_items)
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{
        database::RecipeDatabase,
        inventory::{Inventory, ItemList},
        recipe::Recipe,
    };

    fn test_database() -> RecipeDatabase {
        RecipeDatabase {
            recipes: HashMap::from([
                (
                    "Coarse Bone Powder".into(),
                    Recipe {
                        ingredients: ItemList::from([("Animal Bone".into(), 10)]),
                        result_count: 1,
                    },
                ),
                (
                    "Varnish of Purity".into(),
                    Recipe {
                        ingredients: ItemList::from([
                            ("Stone of Purity".into(), 1),
                            ("Varnish".into(), 3),
                            ("Coarse Bone Powder".into(), 3),
                        ]),
                        result_count: 1,
                    },
                ),
            ]),
        }
    }

    #[test]
    fn test_success() {
        let database = test_database();

        let mut inventory = Inventory {
            items: ItemList::from([("Animal Bone".into(), 10)]),
        };

        inventory
            .craft(
                ItemList::from([("Coarse Bone Powder".into(), 1)]),
                &database,
            )
            .expect("Failed to craft cbp");

        assert_eq!(inventory.items["Coarse Bone Powder"], 1);
        assert_eq!(inventory.items["Animal Bone"], 0);
    }

    #[test]
    fn test_failure() {
        let database = test_database();

        let mut inventory = Inventory {
            items: ItemList::from([("Animal Bone".into(), 9)]),
        };

        let missing = inventory
            .craft(
                ItemList::from([("Coarse Bone Powder".into(), 1)]),
                &database,
            )
            .expect_err("Crafted CBP from insufficient items");

        assert_eq!(missing["Animal Bone"], 1);
    }

    #[test]
    fn test_vop() {
        let database = test_database();

        let mut inventory = Inventory {
            items: ItemList::from([
                ("Animal Bone".into(), 21),
                ("Coarse Bone Powder".into(), 1),
                ("Stone of Purity".into(), 2),
                ("Varnish".into(), 5),
                ("Varnish of Purity".into(), 1),
            ]),
        };

        inventory
            .craft(ItemList::from([("Varnish of Purity".into(), 1)]), &database)
            .expect("Failed to craft VoP");

        assert_eq!(inventory.items["Varnish of Purity"], 2);
        assert_eq!(inventory.items["Stone of Purity"], 1);
        assert_eq!(inventory.items["Coarse Bone Powder"], 0);
        assert_eq!(inventory.items["Animal Bone"], 1);
        assert_eq!(inventory.items["Varnish"], 2);
    }

    #[test]
    fn test_failure_vop() {
        let database = test_database();

        let mut inventory = Inventory {
            items: ItemList::from([
                ("Animal Bone".into(), 19),
                ("Coarse Bone Powder".into(), 1),
                ("Stone of Purity".into(), 2),
                ("Varnish".into(), 5),
            ]),
        };

        let missing = inventory
            .craft(ItemList::from([("Varnish of Purity".into(), 1)]), &database)
            .expect_err("Crafted VoP from insufficient items");

        assert_eq!(missing["Animal Bone"], 1);
    }
}
