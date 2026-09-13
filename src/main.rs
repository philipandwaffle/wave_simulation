pub mod consts;
pub mod scalar_wave;

use std::f32::consts::PI;

use bevy::{
    math::{
        USizeVec2,
        ops::{sin, sqrt},
    },
    prelude::*,
};

use crate::scalar_wave::plugin::WavePlugin;

#[derive(Component)]
pub struct DeformablePlane;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SandboxPlugin))
        .run();
}

struct SandboxPlugin;
impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WavePlugin)
            .add_systems(Startup, (Self::spawn_light, Self::spawn_cam));
    }
}

impl SandboxPlugin {
    fn spawn_cam(mut commands: Commands) {
        commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 40.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));
    }
    fn spawn_light(mut commands: Commands) {
        commands.spawn((
            // DirectionalLight { ..default() },
            PointLight {
                intensity: 10000000.0,
                range: 100.0,
                ..default()
            },
            Transform::from_xyz(0.0, 20.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));
    }
}
