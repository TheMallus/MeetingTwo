pub use bevy::prelude::*;

use crate::entities::Block;

use super::*;

pub fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>)
// lets quickly explain what the parameters mean. mut stands for mutable or when the variable
// is going to change value. A query lets you access the data of a given entity, in this case
// we want to grab the velocity and acceleration data and update the values
// Res stands for resource which in this case is the 'Time' resource
// you might also see ResMut which stands for mutable resource
{
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    } // we use a for loop to iterate data given through every frame
      // this function basically says for every velocity value per frame, update the velocity
      // based on whether acceleration is being applied to the entity
}

// We're going to do the same thing but for position

pub fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}
// translation is the position of the entity, consider it the variable of the Transform component

// #[derive(Component, Debug)]
// pub struct Collider {
//     pub radius: f32,
//     pub colliding_entities: Vec<Entity>,
// }

// impl Collider {
//     pub fn new(radius: f32) -> Self {
//         Self {
//             radius,
//             colliding_entities: vec![],
//         }
//     }
// }

//                                      Making the Block Move
// ===================================================================================================

// So now we have a labeled block but it still doesn't move so we need to give it some controls.
// First we need to create functions for our 2 components velocity and acceleration so our system
// updates the current velocity and acceleration per frame

// Now for the fun part the controls! We're gonna make 2 constants speed and rotational speed
// (you can adjust this if you think the block moves way too slow or fast or turns too slow)

const SPEED: f32 = 10.0;
const ROTATION_SPEED: f32 = 2.5;

// const means constant often capitalized to indiciate these variable will NEVER change
// now lets bind some awsd buttons to our block to make it move :D

pub fn block_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Block>>, //With<Block> specifies we only want the
    keyboard_input: Res<ButtonInput<KeyCode>>,                      //data from "Block" entities
    time: Res<Time>,
) {
    //we're gonna make some local variables to help with our controller system
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
        return;
    };
    //we use single_mut since we're only looking at the block we spawned, this will only work with
    //EXACTLY 1 entity
    let mut rotation = 0.0;
    let mut movement = 0.0;

    //in python you'll probably see variables declared like x = int(3) or y = str("Hello!")
    //or in c++ this will be declared as int x = 0 or string y = "Hello!"
    //but in rust we have mutable and immutable variables which uses the syntax 'let' and 'let mut'
    //however does not need a variable type specified

    //now lets bind the button inputs

    if keyboard_input.pressed(KeyCode::KeyS) {
        movement = -SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyW) {
        movement = SPEED;
    } // forward and backwards movement controls

    if keyboard_input.pressed(KeyCode::KeyA) {
        rotation = ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        rotation = -ROTATION_SPEED * time.delta_seconds();
    } // we want our block to have real rotation so instead of sideways movement
      // we'll rotate the block over time to move with our forward and backward movement
      // or spin in place

    //now lets update our variables based on button input
    transform.rotate_y(rotation); // this will rotate our block if buttons A or D are pressed
    velocity.value = -transform.forward() * movement; //this updates velocity based on buttons S or W
                                                      //being pressed
                                                      //notice it says negative transform that's because the forward command indicates the negative Z value
                                                      //so we use a 2nd negative to turn it positive
}
