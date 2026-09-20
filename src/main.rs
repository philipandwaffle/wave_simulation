pub mod camera_controller;
pub mod config;
pub mod consts;
pub mod display;
pub mod handles;
pub mod scalar_wave;
pub mod yee_lattice;

use std::f32::consts::PI;

use bevy::{
    math::{
        USizeVec2,
        ops::{sin, sqrt},
    },
    prelude::*,
};

use crate::{
    camera_controller::plugin::CameraControllerPlugin, config::ConfigPlugin,
    handles::HandlesPluginType, scalar_wave::plugin::ScalarWavePlugin,
};

#[derive(Component)]
pub struct DeformablePlane;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ConfigPlugin,
            CameraControllerPlugin,
            SandboxPlugin,
        ))
        .run();
}

struct SandboxPlugin;
impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::spawn_light);
    }
}

impl SandboxPlugin {
    fn spawn_light(mut commands: Commands) {
        commands.spawn((
            // DirectionalLight {
            //     shadow_maps_enabled: true,
            //     ..default()
            // },
            PointLight {
                shadow_maps_enabled: true,
                intensity: 10000000.0,
                range: 100.0,
                ..default()
            },
            Transform::from_xyz(0.0, 20.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));
    }
}
