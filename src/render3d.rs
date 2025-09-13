use bevy::{math::Affine2, prelude::*};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use crate::game::{GamePosition, Robot};

pub struct Render3dPlugin;

impl Plugin for Render3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_plugins(PanOrbitCameraPlugin);
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // circular base
    commands.spawn((
        // Level textures are 127*127, so half size is 63.5x63.5
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, vec2(63.5, 63.5)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("temp.png")),
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        Transform::from_translation(vec3(0.0, -0.5, 0.0)),
    ));
    // cube
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Robot,
            GamePosition::default(),
        ))
        .with_children(|ctx| {
            ctx.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.1, 0.1, 0.1))),
                MeshMaterial3d(materials.add(Color::BLACK)),
                Transform::from_xyz(0.55, 0.3, 0.3),
            ));
            ctx.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.1, 0.1, 0.1))),
                MeshMaterial3d(materials.add(Color::BLACK)),
                Transform::from_xyz(0.55, 0.3, -0.3),
            ));
            ctx.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.1, 0.1, 0.1))),
                MeshMaterial3d(materials.add(Color::WHITE)),
                Transform::from_xyz(0.55, 0.3, 0.2),
            ));
            ctx.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.1, 0.1, 0.1))),
                MeshMaterial3d(materials.add(Color::WHITE)),
                Transform::from_xyz(0.55, 0.3, -0.2),
            ));
            ctx.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.1, 0.1, 0.5))),
                MeshMaterial3d(materials.add(Color::BLACK)),
                Transform::from_xyz(0.55, -0.3, 0.0),
            ));
            ctx.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.1, 0.2, 0.1))),
                MeshMaterial3d(materials.add(Color::BLACK)),
                Transform::from_xyz(0.55, -0.25, 0.3),
            ));
            ctx.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.1, 0.2, 0.1))),
                MeshMaterial3d(materials.add(Color::BLACK)),
                Transform::from_xyz(0.55, -0.25, -0.3),
            ));
        });
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera::default(),
    ));
}
