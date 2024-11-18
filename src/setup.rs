//simple world create
use bevy::prelude::*;
// use crate::{movement::*, combat::*, entities::Dummy};
// use bevy_rapier3d::prelude::Collider;
pub struct StartingWorldPlugin;

impl Plugin for StartingWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
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
    // commands.spawn((
    //     MovingObjBundle {
    //         velocity: Velocity::new(Vec3::ZERO),
    //         acceleration: Acceleration::new(Vec3::ZERO),
    //         health: Health::new(10000.0),
    //         collider: Collider::cuboid(0.5,0.5,0.5),
    //         collision: CollisionDamage(35.0),
    //     },
    //     PbrBundle {
    //         mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)), // makes the cube
    //         material: materials.add(Color::srgb(255.0, 0.0, 0.0)), // makes the cube black
    //         transform: Transform::from_xyz(5.0,0.5,5.0),
    //         ..default() // Transform describes the position of the block
    //     },
    //     Dummy,
    //     Name::new("Fixed Block"),
    // ));
}
