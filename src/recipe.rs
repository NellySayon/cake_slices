//------------------------- RECIPE BLUEPRINT -------------------------------
// This is the component of the recipe module. It is responsible for
// creating the recipe and managing the ingredients.
//--------------------------------------------------------------------------

use scrypto::prelude::*;

#[blueprint]
mod recipe {

    // Define what ingredients will be managed by the Recipe component
    struct Recipe {
        flour_gramm: Decimal,
        baking_powder_gramm: Decimal,
        sugar_gramm: Decimal,
        butter_gramm: Decimal,
        eggs: u8,
        milk_ml: Decimal,
        //these are the vegan alternatives
        oil_ml: Decimal, 
        bananas: u8,
        oat_milk_ml: Decimal,
    }

    // Note: we could build the Recipe blueprint to be able to manage different recipes, 
    // but for now we will stick to one that will work for our cake
    impl Recipe {
        
        // This will create a new Recipe component and we can use it to manage the ingredients
        pub fn instantiate_recipe(is_vegan: bool) -> Global<Recipe> {
            // dependent on the is_vegan parameter we will create the recipe with the vegan or non-vegan ingredients
            let my_recipe: Recipe = if is_vegan == true {
                Recipe {
                    flour_gramm: 200.into(),
                    baking_powder_gramm: 16.into(),
                    sugar_gramm: 100.into(),
                    butter_gramm: 0.into(),
                    eggs: 0,
                    milk_ml: 0.into(),
                    oil_ml: 100.into(),
                    bananas: 2,
                    oat_milk_ml: 200.into(),
                }
            } else {
                Recipe {
                    flour_gramm: 200.into(),
                    baking_powder_gramm: 16.into(),
                    sugar_gramm: 100.into(),
                    butter_gramm: 100.into(),
                    eggs: 2,
                    milk_ml: 100.into(),
                    oil_ml: 0.into(),
                    bananas: 0,
                    oat_milk_ml: 0.into(),
                }
            };

            // Instantiate a Recipe component and tell it what ingredients we have
            Self {
                flour_gramm: my_recipe.flour_gramm,
                baking_powder_gramm: my_recipe.baking_powder_gramm,
                sugar_gramm: my_recipe.sugar_gramm,
                butter_gramm: my_recipe.butter_gramm,
                eggs: my_recipe.eggs,
                milk_ml: my_recipe.milk_ml,
                oil_ml: my_recipe.oil_ml,
                bananas: my_recipe.bananas,
                oat_milk_ml: my_recipe.oat_milk_ml,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        // In case you want to have less (or more) sugar in your recipe, 
        // you can adjust it with this method
        // even after the recipe has been instantiated
        pub fn adjust_sugar(&mut self, sugar_gramm: Decimal) {
            self.sugar_gramm = sugar_gramm;
        }
    }
}
