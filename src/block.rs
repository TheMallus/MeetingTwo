use crate::{
    // collision_detector::CollisionDamage,
    combat::{CollisionDamage, Health},
    entities::{Block, Dummy},
    movement::{Acceleration, MovingObjBundle, Velocity},
    schedule::InGameSet,
    states::GameState,
    asset_loader::SceneAssets,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, Sensor};
use rand::Rng;
use std::ops::Range;

const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -5.0..5.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..5.0;
pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_block)
            .add_systems(OnEnter(GameState::GameOver), spawn_block)
            .add_systems(Update, spawn_dummy.in_set(InGameSet::UserInput))
            .add_systems(Update, block_destroyed.in_set(InGameSet::EntityUpdates));
    }
}

// now that we have our block bundle let's spawn in the block!

pub fn spawn_block(
    mut commands: Commands, // this includes the spawn command that we'll use
    scene_assets: Res<SceneAssets>,
) {
    commands.spawn((
        MovingObjBundle {
            velocity: Velocity::new(Vec3::ZERO), // recall our functions we made this will just
            acceleration: Acceleration::new(Vec3::ZERO), // make a '0.0' 3d vector like this (0.0, 0.0, 0.0)
            health: Health::new(1000.0),
            collider: Collider::cuboid(0.1, 0.1, 0.1),
            collision: CollisionDamage(100.0),
            // this will spawn the block at the xyz coords (0.0, 0.5, 0.0)
        }, // the default sets the rest of the PbrBundle components to
        SceneBundle {
            scene: scene_assets.block.clone(),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        // a built in default variable
        Block, // <---- see below for why this is here
        Name::new("Player"),
        Sensor,
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

//This tells the program the spawned block is a "Block" entity and will not confuse it with something else

fn spawn_dummy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut rng = rand::thread_rng();
    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.5,
        rng.gen_range(SPAWN_RANGE_Z),
    );

    let mut random_unit_vector =
        || Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0)).normalize_or_zero();
    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

    if keyboard_input.pressed(KeyCode::KeyJ) {
        commands.spawn((
            MovingObjBundle {
                velocity: Velocity::new(velocity), // recall our functions we made this will just
                acceleration: Acceleration::new(acceleration), // make a '0.0' 3d vector like this (0.0, 0.0, 0.0)
                health: Health::new(100.0),
                collider: Collider::cuboid(0.5, 0.5, 0.5),
                collision: CollisionDamage(35.0),
                // this will spawn the block at the xyz coords (0.0, 0.5, 0.0)
            }, // the default sets the rest of the PbrBundle components to
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)), // makes the cube
                material: materials.add(Color::srgb(255.0, 0.0, 0.0)), // makes the cube black
                transform: Transform::from_translation(translation),
                ..default() // Transform describes the position of the block
            },
            // a built in default variable
            Dummy, // <---- see below for why this is here
            Name::new("Dummy Block"),
        ));
    }
    if keyboard_input.pressed(KeyCode::KeyK) {
        commands.spawn((
            MovingObjBundle {
                velocity: Velocity::new(Vec3::ZERO),
                acceleration: Acceleration::new(Vec3::ZERO),
                health: Health::new(10000.0),
                collider: Collider::cuboid(0.5,0.5,0.5),
                collision: CollisionDamage(35.0),
            },
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)), // makes the cube
                material: materials.add(Color::srgb(255.0, 0.0, 0.0)), // makes the cube black
                transform: Transform::from_xyz(5.0,0.5,5.0),
                ..default() // Transform describes the position of the block
            },
            Dummy,
            Name::new("Fixed Block"),
        ));
    }
}

fn block_destroyed(mut next_state: ResMut<NextState<GameState>>, query: Query<(), With<Block>>) {
    if query.get_single().is_err() {
        next_state.set(GameState::GameOver);
    }
}
