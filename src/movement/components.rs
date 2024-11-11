pub use bevy::prelude::*;

// ============================================================================================
//                                     Intro to Bevy
// ============================================================================================

// You compiled and ran an application that spawned a circular plane and a stationary block.
// Now it's time to give the block some life, in this demo, we'll add movement controls
// and add two components to the block: Velocity and acceleration.
// We'll also learn a few syntax words that you'll commonly see.

// Let's add some variables to our block:

#[derive(Component, Debug)] // this introduces our variable in the form #[derive(<terms>..)]
                            // You will often see terms like "Component", "Resource", "Debug", and "Bundle"
pub struct Velocity {
    // this creates a public struct for your component (we'll call our variables
    // components from here on out) public allows you to use the component in other
    // modules (this will be covered next week)
    pub value: Vec3,
} // We've given our component a 3D vector so we can change velocity to go wherever we want

impl Velocity {
    // this implements (impl) a function exclusively for our component
    pub fn new(value: Vec3) -> Self {
        Self { value }
    } // function merely assigns a given vector to the component
}

// Let's make another component for acceleration
// (this is what the code looks like without the comments)

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}
