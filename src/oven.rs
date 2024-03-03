//--------------------------    OVEN BLUEPRINT    --------------------------
// This is the component of the oven module. It is responsible for
// creating the oven steering it's behaviour.
//--------------------------------------------------------------------------

use scrypto::prelude::*;

#[blueprint]
mod oven {

    // Define how our oven component should work
    struct Oven {
        temperature: Decimal,
        duration: u16,
        program: String,
    }

    impl Oven {
        
        // This will create a new Oven component and direcly start it with
        // the given temperature, duration and program as initial parameters
        pub fn instantiate_oven(temperature: Decimal, duration: u16, program: String) -> Global<Oven> {
            Self {
                temperature,
                duration,
                program,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        // In case you want to adjust the temperature of the oven
        // even after the oven has been started, call this method
        pub fn adjust_temperature(&mut self, new_temperature: Decimal) {
            self.temperature = new_temperature;
        }

        // In case you want to prolong the duration of the oven call this method
        pub fn add_time(&mut self, additional_time: u16) {
            self.duration += additional_time;
        }

        // Stop the oven
        pub fn stop(&mut self) {
            // Stop the oven
            self.temperature = Decimal::from(0);
            self.duration = 0;
        }
    }
}
