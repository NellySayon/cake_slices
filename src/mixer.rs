//------------------------ MIXER BLUEPRINT -------------------------
// This is the component of the mixer module. It is responsible for
// creating the mixer and managing it's behaviour.
//------------------------------------------------------------------

use scrypto::prelude::*;

#[blueprint]
mod mixer {

    // Define how our mixer component should work
    struct Mixer {
        speed: u8,
        start: bool,
    }

    impl Mixer {
        
        // It will create a new Mixer component and we can use it to steer it's behaviour
        // This is an example with no initial parameters
        pub fn instantiate_mixer() -> Global<Mixer> {
            
            // Instantiate a Mixer component but don't start it yet
            Self {
                speed: 0,
                start: false,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        // Method to start the mixer
        pub fn start(&mut self) {
            self.start = true;
        }

        // Choose the speed level of the mixer
        pub fn adjust_speed(&mut self, new_speed: u8) {
            // the mixer has speed levels from 1 to 10, so let's make sure we don't exceed that
            assert!(new_speed <= 10, "The speed of the mixer can not exceed 10");

            self.speed = new_speed;
        }

        // Method to stop the mixer
        pub fn stop(&mut self) {
            self.start = false;
        }
        
    }
}
