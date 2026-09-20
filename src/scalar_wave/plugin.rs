use std::f32::consts::PI;

use crate::{
    display::surface::{SurfacePlugin, SurfaceSpawn, SurfaceUpdate},
    scalar_wave::{
        scalar_wave_field::ScalarWaveField,
        sim_config::{SimConfig, Simulation},
        sim_title::SimTitle,
    },
};
use bevy::{
    app::{Plugin, Startup, Update},
    asset::Assets,
    color::Color,
    ecs::{
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::{Quat, USizeVec2, Vec2, ops::*, primitives::Plane3d, vec2},
    mesh::{Mesh, Mesh3d, Meshable, VertexAttributeValues},
    pbr::{MeshMaterial3d, StandardMaterial},
    text::{FontSize, TextColor, TextFont},
    time::Time,
    transform::components::Transform,
    ui::{Node, PositionType, Val, widget::Text},
    utils::default,
};
use config::{ConfigTag, config_tag::ConfigTag};
use serde::{Deserialize, Serialize};

#[derive(ConfigTag, Serialize, Deserialize, Clone)]
pub struct ScalarWavePlugin {
    world_size: Vec2,
    world_centre: Vec2,
    simulation_size: Vec2,
    simulation_centre: Vec2,
    resolution: USizeVec2,
}
impl Plugin for ScalarWavePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let (x_len, y_len) = (self.resolution.x, self.resolution.y);
        let mut xy_points = Vec::<Vec2>::with_capacity(x_len * y_len);
        let mut base_u = vec![0.0; x_len * y_len];

        let mut cur_pos = self.simulation_centre - (self.simulation_size * 0.5);
        let step = vec2(
            self.simulation_size.x / x_len as f32,
            self.simulation_size.y / y_len as f32,
        );
        if step.x != step.y {
            panic!("The x and y step are different when they must be equal. step: {step:?}")
        }

        for _ in 0..y_len {
            for _ in 0..x_len {
                xy_points.push(cur_pos);
                cur_pos.x += step.x
            }
            cur_pos.x = (self.simulation_size * -0.5).x;
            cur_pos.y += step.y;
        }

        let space_step = step.x;
        let dur = 15.0;

        let mut sim_config = SimConfig::new(space_step, 1.0, 0.999);
        let mut sims = vec![];

        let mut u = base_u.clone();
        sims.push(Simulation::new("", u.clone(), 0.0));
        // ====================================================================================================
        let point = vec2(0.0, 0.0);
        for (index, pos) in xy_points.iter().enumerate() {
            let dist = pos.length();
            if dist < PI / 8.0 {
                u[index] = 50.0 * cos(dist * 4.0);
            }
        }
        sims.push(Simulation::new("Cosine peak placed on the centre", u, dur));
        // ====================================================================================================
        let mut u = base_u.clone();
        let point = vec2(0.0, 0.0);
        for (index, pos) in xy_points.iter().enumerate() {
            let dist = (pos - point).length();
            if dist > PI * 0.8 && dist < PI * 1.12 {
                u[index] = 10.0;
            }
        }
        sims.push(Simulation::new("Positive centred ring", u, dur));
        // ====================================================================================================
        let mut u = base_u.clone();
        for (index, pos) in xy_points.iter().enumerate() {
            if pos.x > -PI * 0.2 && pos.x < PI * 0.2 {
                u[index] = 10.0;
            }
        }
        sims.push(Simulation::new("Centred positive line", u, dur));
        // ====================================================================================================
        let mut u = base_u.clone();
        for (index, pos) in xy_points.iter().enumerate() {
            if pos.x > -PI * 1.2 && pos.x < -PI * 0.8 {
                u[index] = 10.0;
            }
        }
        sims.push(Simulation::new("Positive line placed on the left", u, dur));
        // ====================================================================================================
        let mut u = base_u.clone();
        for (index, pos) in xy_points.iter().enumerate() {
            if pos.x > -PI * 1.2 && pos.x < -PI * 0.8 {
                u[index] = 10.0;
            } else if pos.x > PI * 0.8 && pos.x < PI * 1.2 {
                u[index] = -10.0;
            }
        }
        sims.push(Simulation::new(
            "Positive and negative lines placed opposite each other",
            u,
            dur,
        ));
        // ====================================================================================================
        let mut u = base_u.clone();
        for (index, pos) in xy_points.iter().enumerate() {
            if (pos.x > PI * -1.2 && pos.x < PI * -0.8) || (pos.x > PI * 0.8 && pos.x < PI * 1.2) {
                u[index] = 10.0;
            }
        }
        sims.push(Simulation::new(
            "Positive lines placed opposite each other",
            u,
            dur,
        ));
        // ====================================================================================================
        let mut u = base_u.clone();
        let point = vec2(-PI, 0.0);
        for (index, pos) in xy_points.iter().enumerate() {
            let dist = pos.length();
            if dist < PI / 8.0 {
                u[index] = 50.0 * cos(dist * 4.0);
            }
        }
        sims.push(Simulation::new("Cosine peak placed on the left", u, dur));
        // ====================================================================================================
        let mut u = base_u.clone();
        let point = vec2(-PI, 0.0);
        for (index, pos) in xy_points.iter().enumerate() {
            let dist = (pos - point).length();
            if dist < PI / 8.0 {
                u[index] = 50.0 * cos(dist * 4.0);
            }
        }
        let point = vec2(PI, 0.0);
        for (index, pos) in xy_points.iter().enumerate() {
            let dist = (pos - point).length();
            if dist < PI / 8.0 {
                u[index] = -50.0 * cos(dist * 4.0);
            }
        }
        sims.push(Simulation::new(
            "Positive and negative cosine peaks placed opposite each other",
            u,
            dur,
        ));
        // ====================================================================================================
        let mut u = base_u;
        let point = vec2(-PI, 0.0);
        for (index, pos) in xy_points.iter().enumerate() {
            let dist = (pos - point).length();
            if dist < PI / 8.0 {
                u[index] = 50.0 * cos(dist * 4.0);
            }
        }
        let point = vec2(PI, 0.0);
        for (index, pos) in xy_points.iter().enumerate() {
            let dist = (pos - point).length();
            if dist < PI / 8.0 {
                u[index] = 50.0 * cos(dist * 4.0);
            }
        }
        sims.push(Simulation::new(
            "Positive cosine peaks placed opposite each other",
            u,
            dur,
        ));
        // ====================================================================================================

        sims.reverse();
        sim_config = sim_config.with_simulations(sims);

        let me = self.clone();
        app.insert_resource(sim_config)
            .add_plugins(SurfacePlugin)
            .add_systems(
                Startup,
                (
                    move |commands: Commands, sim_config: Res<SimConfig>| {
                        Self::spawn_surface(me.clone(), commands, sim_config)
                    },
                    Self::setup_title,
                ),
            )
            .add_systems(Update, Self::toggle_paused)
            .add_systems(Update, Self::change_sim.run_if(Self::should_change_sim))
            .add_systems(
                Update,
                (Self::step_wave_field_2d, Self::tick_sim_config).run_if(Self::can_step),
            );
    }
}
impl ScalarWavePlugin {
    fn setup_title(mut commands: Commands) {
        commands.spawn((
            SimTitle,
            Text::new("Hello, top left!"),
            TextFont {
                font_size: FontSize::Px(30.0),
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            },
        ));
    }
    fn can_step(sim_config: Res<SimConfig>) -> bool {
        !sim_config.is_paused()
    }
    fn should_change_sim(sim_config: Res<SimConfig>) -> bool {
        sim_config.should_change_sim()
    }

    fn tick_sim_config(mut sim_config: ResMut<SimConfig>, time: Res<Time>) {
        sim_config.tick(time.delta_secs());
    }

    fn change_sim(
        mut sim_config: ResMut<SimConfig>,
        mut wave_field: Query<&mut ScalarWaveField>,
        mut sim_title: Query<&mut Text, With<SimTitle>>,
    ) {
        let Some(sim) = sim_config.next_sim() else {
            return;
        };

        let Ok(mut wave_field) = wave_field.single_mut() else {
            return;
        };

        let Ok(mut sim_title) = sim_title.single_mut() else {
            return;
        };

        wave_field.set_u(sim.initial_u);
        sim_title.0 = sim.title;
    }

    fn toggle_paused(mut settings: ResMut<SimConfig>, keys: Res<ButtonInput<KeyCode>>) {
        if keys.just_released(KeyCode::Space) {
            settings.toggle_paused();
        }
    }

    fn step_wave_field_2d(
        mut commands: Commands,
        mut scalar_wave_field: ResMut<ScalarWaveField>,
        time: Res<Time>,
    ) {
        let surface = scalar_wave_field.step(time.delta_secs());
        commands.trigger(SurfaceUpdate::new(surface));
        // if let Some(mut mesh) = meshes.get_mut(mesh_handle) {
        //     if let Some(VertexAttributeValues::Float32x3(positions)) =
        //         mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        //     {
        //         for [x, y, z] in positions.iter_mut() {
        //             *y = field.get_xy(*x, *z, -50.0, 50.0);
        //         }
        //     }
        // }
    }

    fn spawn_surface(config: Self, mut commands: Commands, sim_config: Res<SimConfig>) {
        commands.insert_resource(ScalarWaveField::new(
            config.resolution,
            sim_config.space_step,
            sim_config.wave_speed,
            sim_config.damping,
        ));
        commands.trigger(SurfaceSpawn::new(
            config.world_size,
            config.world_centre,
            config.resolution,
        ));
    }
}
