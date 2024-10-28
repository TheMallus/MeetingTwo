use std::ops::Range;

use bevy::{prelude::*, utils::HashMap};
use rand::Rng;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::{quick::WorldInspectorPlugin, bevy_egui::EguiContext};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(PostStartup, spawn_block)
        .add_systems(Update, (update_velocity, update_position))
        .add_systems(Update, block_movement_controls)
        .add_systems(Update, (print_position, print_health))
        .add_systems(Update, collision_detection)
        .add_systems(Update, block_weapons)
        .add_systems(Update, despawn_far_away_entities)
        .add_systems(Update, spawn_dummy)
        .add_systems(Update, handle_collisions)
        .add_systems(Update, health_ui)
        .run();
}

//                                      The Original File
// =======================================================================================

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(100.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-10.5, 7.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

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
pub struct Velocity {       // this creates a public struct for your component (we'll call our variables
                            // components from here on out) public allows you to use the component in other
                            // modules (this will be covered next week)
    pub value: Vec3,
} // We've given our component a 3D vector so we can change velocity to go wherever we want

impl Velocity {             // this implements (impl) a function exclusively for our component
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

#[derive(Component, Debug)]
pub struct Health {
    pub value: f32,
}

impl Health {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

// const DAMAGE: f32 = 0.5;
// fn update_health (
//     mut query: Query<(Entity, &mut Health)>,
//     mut commands: Commands
// )
// {
//     for (entity, mut health) in query.iter_mut() {
//         health.value += -DAMAGE;
//         if health.value <= 0.0 {
//             commands.entity(entity).despawn_recursive();
//             continue
//         }
//     }
// }
// damahe component is based on collision, possibly use rapier for this or else use raycast or the demo
// collision system that isn't run with a bevy engine and hard coded but actually works

// Components are great for organizing various items for an entity(like the block)
// but we can organize them further by grouping them into bundles
// bundles are great for categorizing your variables into different parts
// a robot for example will have a wheels bundle, movement bundle, weapons bundle
// and a physics bundle (like weight, mass, etc.) which have their own variables

// Let's make our first bundle for our block, this will add our 2 components we made
// as well as the cube model for our block

#[derive(Bundle)]
pub struct MovingObjBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub model: PbrBundle, // this is a built in bevy bundle
    pub health: Health,
    pub collider: Collider,
}

// now that we have our block bundle let's spawn in the block!

pub fn spawn_block(
    mut commands: Commands,                           // this includes the spawn command that we'll use
    mut meshes: ResMut<Assets<Mesh>>,                 // PbrBundle components
    mut materials: ResMut<Assets<StandardMaterial>>   // PbrBundle components
    ) 
{
    commands.spawn(
        (MovingObjBundle {
            velocity: Velocity::new(Vec3::ZERO),         // recall our functions we made this will just
            acceleration: Acceleration::new(Vec3::ZERO), // make a '0.0' 3d vector like this (0.0, 0.0, 0.0)
            model: PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0,1.0,1.0)), // makes the cube
                material: materials.add(Color::BLACK), // makes the cube black
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default() // Transform describes the position of the block
            },
            health: Health::new(1000.0), 
            collider: Collider::new(1.0),             
                            // this will spawn the block at the xyz coords (0.0, 0.5, 0.0)
        },                  // the default sets the rest of the PbrBundle components to
                            // a built in default variable
        Block,   // <---- see below for why this is here
    ));
}

// This looks similar to the original spawn block command with the difference being we have given
// our block a velocity and acceleration component... and also a block component?

//                                          Entity Naming
//====================================================================================================

//it's good practice to keep track of entities when there's quite a few in a game.
//Most functions that deal with movement and firing weapons like bullets iterate through
//every entity per frame, this includes every block, object, map, and yes every single bullet.
//However if we name our entities, we can write functions that only read selected entities
//reducing memory usage and making the game run smoother.

//Above we named our block by adding an empty struct component called Block as seen below:

#[derive(Component, Debug)]
pub struct Block;

//This tells the program the spawned block is a "Block" entity and will not confuse it with something else



//                                      Making the Block Move
// ===================================================================================================

// So now we have a labeled block but it still doesn't move so we need to give it some controls.
// First we need to create functions for our 2 components velocity and acceleration so our system
// updates the current velocity and acceleration per frame

fn update_velocity(
    mut query: Query<(&Acceleration, &mut Velocity)>,
    time: Res<Time>
    )
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

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}
// translation is the position of the entity, consider it the variable of the Transform component


// Now for the fun part the controls! We're gonna make 2 constants speed and rotational speed
// (you can adjust this if you think the block moves way too slow or fast or turns too slow)

const SPEED: f32 = 10.0;
const ROTATION_SPEED: f32 = 2.5;

// const means constant often capitalized to indiciate these variable will NEVER change
// now lets bind some awsd buttons to our block to make it move :D

fn block_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Block>>, //With<Block> specifies we only want the
    keyboard_input: Res<ButtonInput<KeyCode>>,                      //data from "Block" entities
    time: Res<Time>,
    ) 
{
    //we're gonna make some local variables to help with our controller system
    let (mut transform, mut velocity) = query.single_mut();
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
    }
    else if keyboard_input.pressed(KeyCode::KeyW) {
        movement = SPEED;
    } // forward and backwards movement controls

    
    if keyboard_input.pressed(KeyCode::KeyA) {
        rotation = ROTATION_SPEED * time.delta_seconds();
    }
    else if keyboard_input.pressed(KeyCode::KeyD) {
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

//                                               Debug
// ====================================================================================================

// Our block moves now! that's the end of the demo extension, last step we want to add for debugging cases
// (or for future purpose if you want to figure out why something doesn't work), we'll quickly make a function
// that will output to the terminal the blocks position at every frame (this will look like spam when
// when running the program but it'll tell you when the block changes position)

fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!("Entity {:?} is at position {:?},", entity, transform.translation);
    } // function basically prints location of ALL existing entities per frame
}

fn print_health(query: Query<(Entity, &Health)>) {
    for  (entity, health) in query.iter() {
        info!("Entity {:?} has {:?} health.", entity, health.value);
    }
}




#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a != entity_b {
                let distance = transform_a.translation().distance(transform_b.translation());
                if distance < collider_a.radius + collider_b.radius {
                    colliding_entities
                        .entry(entity_a)
                        .or_insert_with(Vec::new)
                        .push(entity_b);
                }
            }
        }
    }

    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider.colliding_entities.extend(collisions.iter().copied());
        }
    }

}

#[derive(Component, Debug)]
pub struct Bullet;

fn block_weapons(
    mut commands: Commands,
    query: Query<&mut Transform, With<Block>>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) 
{
    let transform = query.single();
    if keyboard_input.pressed(KeyCode::KeyF) {
        commands.spawn((MovingObjBundle {
            velocity: Velocity::new(-transform.forward() * 25.0),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(1.0),
            health: Health::new(100.0),
            model: PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1,0.1,0.1)),
                material: materials.add(Color::BLACK),
                transform: Transform::from_translation(
                    transform.translation + -transform.forward() * 1.1,
                ),
                ..default()
            },
        },
        Bullet,
        ));
    }

}

fn despawn_far_away_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);

        if distance > 25.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn handle_collisions(mut commands: Commands, query: Query<(Entity, &Collider), With<Dummy>>,) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            if query.get(collided_entity).is_ok() {
                continue;
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_dummy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {

    let mut rng = rand::thread_rng();
    let translation = Vec3::new(rng.gen_range(SPAWN_RANGE_X), 0.5, rng.gen_range(SPAWN_RANGE_Z),);

    let mut random_unit_vector = || Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0)).normalize_or_zero();
    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;
    

    if keyboard_input.pressed(KeyCode::KeyJ) {
        commands.spawn(
            (MovingObjBundle {
                velocity: Velocity::new(velocity),         // recall our functions we made this will just
                acceleration: Acceleration::new(acceleration), // make a '0.0' 3d vector like this (0.0, 0.0, 0.0)
                model: PbrBundle {
                    mesh: meshes.add(Cuboid::new(1.0,1.0,1.0)), // makes the cube
                    material: materials.add(Color::srgb(255.0,0.0,0.0)), // makes the cube black
                    transform: Transform::from_translation(translation),
                    ..default() // Transform describes the position of the block
                },
                health: Health::new(100.0), 
                collider: Collider::new(1.0),             
                                // this will spawn the block at the xyz coords (0.0, 0.5, 0.0)
            },                  // the default sets the rest of the PbrBundle components to
                                // a built in default variable
            Dummy,   // <---- see below for why this is here
        ));
    }
}

#[derive(Component, Debug)]
pub struct Dummy;


const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -5.0..5.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..5.0;

fn health_ui(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    entity_health: Query<(Entity, &Health), Without <Bullet>>,
) {
    for mut context in primary_window.iter_mut() {
        egui::Window::new("Health").show(context.get_mut(), |ui| {
            for (e, health) in entity_health.iter() {
                ui.heading(format!(
                    "{:#?}'s Health: {:#}",
                    e, health.value
                ));
            }
        });
    }
}