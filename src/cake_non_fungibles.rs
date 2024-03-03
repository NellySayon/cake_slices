//--------------------------    CAKE BLUEPRINT    --------------------------
// This is the main component of the cake module. It is responsible for
// creating the cake and managing the other components.
// It can create non fungible tokens representing cake slices.
//--------------------------------------------------------------------------

use scrypto::prelude::*;

// Import the other components we need to create the cake
use crate::mixer::*;
use crate::oven::*;
use crate::recipe::*;

// here we define the non-fungible data that will be managed by the Cake component
#[derive(NonFungibleData, ScryptoSbor)]
pub struct CakeSlice {
    #[mutable]
    gramms: Decimal,
    calories: Decimal,
    taste: String,
}

#[blueprint]
mod cake_non_fungibles {
    struct CakeNFT{
        // Define what resources and data will be managed by Cake components
        recipe: Global<recipe::Recipe>,
        mixer: Global<mixer::Mixer>,
        oven: Global<oven::Oven>,
        collected_xrd: Vault,
        // The resource manager address for the NFTs
        cake_resource_manager: ResourceManager,
        // A counter for NFT ID generation
        slice_id_counter: u64,
    }

    impl CakeNFT {
        pub fn instantiate_cake_with_nft(is_vegan: bool) -> Global<CakeNFT> {
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

            // Create a resource manager for the cake slices. this is needed later on to actually mint the NFTs    
            let cake_resource_manager = ResourceBuilder::new_integer_non_fungible::<CakeSlice>(OwnerRole::None)
            .metadata(metadata! {
                init {
                    "name" => "Cake", locked;
                    "description" => "Individual piece of cake", locked;
                }
            })
            .mint_roles(mint_roles!{
                minter => rule!(allow_all);
                minter_updater => rule!(deny_all);
            })            
            .create_with_no_initial_supply();

            // Finally put everything together
            Self {
                recipe: my_recipe,
                mixer: my_mixer,
                oven: my_oven,
                collected_xrd: Vault::new(XRD),
                cake_resource_manager,
                slice_id_counter: 1
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        // get a cake slice, add it's individual data and take the payment for it
        pub fn get_cake_slice(&mut self, gramms: Decimal, calories: Decimal, taste: String, payment: Bucket) -> Bucket {
            let data = CakeSlice {
                gramms,
                calories,
                taste,
            };
            // mint a new non fungible token = cut a new slice of cake          
            let slice_bucket = self.cake_resource_manager.mint_non_fungible(
                &NonFungibleLocalId::Integer(self.slice_id_counter.into()),
                data,
            );
            // increment the counter to prepare for the next minting
            self.slice_id_counter += 1;

            // if the customer has sent too much XRD we assume that this is a tip and just keep it ;-)
            self.collected_xrd.put(payment);
            
            // return the newly cut slice of our cake            
            slice_bucket
        }

        // Note: there is more things to consider like access roles, withdrawing the collected XRD
        // and e.g. checking if the cake is already sold out
        // but again, this is just a simplified example to show how to create a smart contract with non fungibles
    }
}
