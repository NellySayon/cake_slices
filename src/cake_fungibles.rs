//--------------------------    CAKE BLUEPRINT    --------------------------
// This is the main component of the cake module. It is responsible for
// creating the cake and managing the other components.
// It contains fungible tokens representing cake slices.
//--------------------------------------------------------------------------

use scrypto::prelude::*;

// Import the other components we need to create the cake
use crate::mixer::*;
use crate::oven::*;
use crate::recipe::*;

#[blueprint]
mod cake_fungibles {
    struct CakeFungible {
        // Define what resources and data will be managed by Cake components
        recipe: Global<recipe::Recipe>,
        mixer: Global<mixer::Mixer>,
        oven: Global<oven::Oven>,
        cake_slices: FungibleVault,
        collected_xrd: Vault,
        price: Decimal
    }

    impl CakeFungible {
        pub fn instantiate_cake_with_fungibles(is_vegan: bool, price_per_slice: Decimal) -> Global<CakeFungible> {
            // Instantiate a Recipe component, and tell it how we want our cake
            let my_recipe = recipe::Recipe::instantiate_recipe(is_vegan);

            // Instantiate a Mixer component
            let my_mixer = mixer::Mixer::instantiate_mixer();

            // set the speed level and start the mixer
            my_mixer.adjust_speed(5);
            my_mixer.start();

            // Instantiate an Oven component and directly start it with the given temperature, duration and program
            // as we assume we know what we need to bake this cake
            let my_oven =
                oven::Oven::instantiate_oven(Decimal::from(180), 30, "top-bottom-heat".to_string());

            // create a fungible bucket to store the cake slices
            let my_cake_slices: FungibleBucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .metadata(metadata!(
                    init {
                        "name" => "Cake slices", locked;
                        "symbol" => "CAKE", locked;
                        "description" => "A fungible token that represents cake slices", locked;
                    }
                ))
                .divisibility(0) // slices cannot be divided any further
                .mint_initial_supply(12); // fixed supply of 12 cake slices

            // Finally put everything together
            Self {
                recipe: my_recipe,
                mixer: my_mixer,
                oven: my_oven,
                cake_slices: FungibleVault::with_bucket(my_cake_slices),
                collected_xrd: Vault::new(XRD),
                price: price_per_slice,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        pub fn buy_cake_slice(&mut self, mut payment: Bucket) -> (FungibleBucket, Bucket) {
            // take our price in XRD out of the payment
            // if the caller has sent too few, or sent something other than XRD, they'll get a runtime error
            self.collected_xrd.put(payment.take(self.price));

            // return a tuple containing a cake_slice, plus whatever change is left on the input payment (if any)
            // if we're out of cake_slices to give, we'll see a runtime error when we try to grab one
            (self.cake_slices.take(1), payment)
        }

        // Note: there is more things to consider like access roles, changing the price, and withdrawing the collected XRD
        // but again, this is just a simplified example to show how to create a smart contract with fungibles
    }
}
