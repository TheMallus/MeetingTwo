//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(PostStartup, spawn_block)
        .add_systems(Update, (update_velocity, update_position))
        .add_systems(Update, block_movement_controls)
        .add_systems(Update, print_position)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
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
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn spawn_block(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(
        (MovingObjBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0,1.0,1.0)),
                material: materials.add(Color::BLACK),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            },
        },
        Block,
    ));
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

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
pub struct Block;

#[derive(Bundle)]
pub struct MovingObjBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub model: PbrBundle,
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}
fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}

const SPEED: f32 = 10.0;
const ROTATION_SPEED: f32 = 2.5;

fn block_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Block>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::KeyS) {
        movement = -SPEED;
    }
    else if keyboard_input.pressed(KeyCode::KeyW) {
        movement = SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        rotation = ROTATION_SPEED * time.delta_seconds();
    }
    else if keyboard_input.pressed(KeyCode::KeyD) {
        rotation = -ROTATION_SPEED * time.delta_seconds();
    }
    transform.rotate_y(rotation);
    velocity.value = -transform.forward() * movement;
}


fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!("Entity {:?} is at position {:?},", entity, transform.translation);
    }
}

