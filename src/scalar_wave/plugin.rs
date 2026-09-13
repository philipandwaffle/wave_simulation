use std::f32::consts::PI;

use crate::{
    DeformablePlane,
    consts::{N_1D, N_2D},
    scalar_wave::{
        sim_config::{SimConfig, Simulation},
        sim_title::SimTitle,
        wave_field_1d::WaveField1D,
        wave_field_2d::WaveField2D,
    },
};
use bevy::{
    app::{Plugin, Startup, Update},
    asset::Assets,
    color::Color,
    ecs::{
        change_detection::DetectChangesMut,
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::{Quat, ops::*, primitives::Plane3d, vec2},
    mesh::{Mesh, Mesh3d, Meshable, VertexAttributeValues},
    pbr::{MeshMaterial3d, StandardMaterial},
    text::{FontSize, TextColor, TextFont},
    time::Time,
    transform::components::Transform,
    ui::{Node, PositionType, Val, widget::Text},
    utils::default,
};

pub struct WavePlugin;
impl Plugin for WavePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let mut x_1d = [0.0; N_1D];
        let base_u = [0.0; N_2D];

        let start = PI * -2.0;
        let stop = PI * 2.0;
        let space_step = (stop - start) / (N_1D as f32);
        let dur = 15.0;

        // Init x points
        let mut cur = start;
        for i in 0..N_1D {
            x_1d[i] = cur;
            cur += space_step;
        }

        let mut sim_config = SimConfig::new(space_step, 1.0, 0.999);
        let mut sims = vec![];

        sims.push(Simulation::new("", base_u, 0.0));
        // ====================================================================================================
        let mut u = base_u;
        let point = vec2(0.0, 0.0);
        for (i, x) in x_1d.iter().enumerate() {
            for (j, y) in x_1d.iter().enumerate() {
                let dist = (vec2(*x, *y) - point).length();
                if dist < PI / 8.0 {
                    u[i + (j * N_1D)] = 50.0 * cos(x * 4.0);
                }
            }
        }
        sims.push(Simulation::new("Cosine peak placed on the centre", u, dur));
        // ====================================================================================================
        let mut u = base_u;
        let point = vec2(0.0, 0.0);
        for (i, x) in x_1d.iter().enumerate() {
            for (j, y) in x_1d.iter().enumerate() {
                let dist = (vec2(*x, *y) - point).length();
                if dist > PI * 0.8 && dist < PI * 1.12 {
                    u[i + (j * N_1D)] = 10.0;
                }
            }
        }
        sims.push(Simulation::new("Positive centred ring", u, dur));
        // ====================================================================================================
        let mut u = base_u;
        for (i, x) in x_1d.iter().enumerate() {
            for (j, _) in x_1d.iter().enumerate() {
                if *x > -PI * 0.2 && *x < PI * 0.2 {
                    u[i + (j * N_1D)] = 10.0;
                }
            }
        }
        sims.push(Simulation::new("Centred positive line", u, dur));
        // ====================================================================================================
        let mut u = base_u;
        for (i, x) in x_1d.iter().enumerate() {
            for (j, _) in x_1d.iter().enumerate() {
                if *x > -PI * 1.2 && *x < -PI * 0.8 {
                    u[i + (j * N_1D)] = 10.0;
                }
            }
        }
        sims.push(Simulation::new("Positive line placed on the left", u, dur));
        // ====================================================================================================
        let mut u = base_u;
        for (i, x) in x_1d.iter().enumerate() {
            for (j, _) in x_1d.iter().enumerate() {
                if *x > -PI * 1.2 && *x < -PI * 0.8 {
                    u[i + (j * N_1D)] = 10.0;
                } else if *x > PI * 0.8 && *x < PI * 1.2 {
                    u[i + (j * N_1D)] = -10.0;
                }
            }
        }
        sims.push(Simulation::new(
            "Positive and negative lines placed opposite each other",
            u,
            dur,
        ));
        // ====================================================================================================
        let mut u = base_u;
        for (i, x) in x_1d.iter().enumerate() {
            for (j, _) in x_1d.iter().enumerate() {
                if (*x > PI * -1.2 && *x < PI * -0.8) || (*x > PI * 0.8 && *x < PI * 1.2) {
                    u[i + (j * N_1D)] = 10.0;
                }
            }
        }
        sims.push(Simulation::new(
            "Positive lines placed opposite each other",
            u,
            dur,
        ));
        // ====================================================================================================
        let mut u = base_u;
        let point = vec2(-PI, 0.0);
        for (i, x) in x_1d.iter().enumerate() {
            for (j, y) in x_1d.iter().enumerate() {
                let dist = (vec2(*x, *y) - point).length();
                if dist < PI / 8.0 {
                    u[i + (j * N_1D)] = 50.0 * cos(x * 4.0);
                }
            }
        }
        sims.push(Simulation::new("Cosine peak placed on the left", u, dur));
        // ====================================================================================================
        let mut u = base_u;
        let point = vec2(-PI, 0.0);
        for (i, x) in x_1d.iter().enumerate() {
            for (j, y) in x_1d.iter().enumerate() {
                let dist = (vec2(*x, *y) - point).length();
                if dist < PI / 8.0 {
                    u[i + (j * N_1D)] = 50.0 * cos(x * 4.0);
                }
            }
        }
        let point = vec2(PI, 0.0);
        for (i, x) in x_1d.iter().enumerate() {
            for (j, y) in x_1d.iter().enumerate() {
                let dist = (vec2(*x, *y) - point).length();
                if dist < PI / 8.0 {
                    u[i + (j * N_1D)] = -50.0 * cos(x * 4.0);
                }
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
        for (i, x) in x_1d.iter().enumerate() {
            for (j, y) in x_1d.iter().enumerate() {
                let dist = (vec2(*x, *y) - point).length();
                if dist < PI / 8.0 {
                    u[i + (j * N_1D)] = 50.0 * cos(x * 4.0);
                }
            }
        }
        let point = vec2(PI, 0.0);
        for (i, x) in x_1d.iter().enumerate() {
            for (j, y) in x_1d.iter().enumerate() {
                let dist = (vec2(*x, *y) - point).length();
                if dist < PI / 8.0 {
                    u[i + (j * N_1D)] = 50.0 * cos(x * 4.0);
                }
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

        app.insert_resource(sim_config)
            // .add_systems(Startup, Self::spawn_wave_field_1d)
            .add_systems(Startup, (Self::spawn_wave_field_2d, Self::setup_title))
            .add_systems(Update, Self::toggle_paused)
            .add_systems(Update, Self::change_sim.run_if(Self::should_change_sim))
            .add_systems(
                Update,
                (
                    Self::step_wave_field_1d,
                    Self::step_wave_field_2d,
                    Self::tick_sim_config,
                )
                    .run_if(Self::can_step),
            );
    }
}
impl WavePlugin {
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
        mut wave_field: Query<&mut WaveField2D<N_2D>>,
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

    fn step_wave_field_1d(
        mut meshes: ResMut<Assets<Mesh>>,
        mut planes: Query<(&Mesh3d, &mut WaveField1D<N_1D>), With<DeformablePlane>>,
        time: Res<Time>,
    ) {
        for (mesh_handle, mut field) in planes.iter_mut() {
            if let Some(mut mesh) = meshes.get_mut(mesh_handle) {
                if let Some(VertexAttributeValues::Float32x3(positions)) =
                    mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
                {
                    field.step(time.delta_secs());
                    for [x, y, _] in positions.iter_mut() {
                        *y = field.get_x(*x, -50.0, 50.0);
                    }
                }
            }
        }
    }

    fn step_wave_field_2d(
        mut meshes: ResMut<Assets<Mesh>>,
        mut planes: Query<(&Mesh3d, &mut WaveField2D<N_2D>), With<DeformablePlane>>,
        time: Res<Time>,
    ) {
        for (mesh_handle, mut field) in planes.iter_mut() {
            if let Some(mut mesh) = meshes.get_mut(mesh_handle) {
                if let Some(VertexAttributeValues::Float32x3(positions)) =
                    mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
                {
                    field.step(time.delta_secs());
                    for [x, y, z] in positions.iter_mut() {
                        *y = field.get_xy(*x, *z, -50.0, 50.0);
                    }
                }
            }
        }
    }

    // fn spawn_wave_field_1d(
    //     mut commands: Commands,
    //     mut meshes: ResMut<Assets<Mesh>>,
    //     mut materials: ResMut<Assets<StandardMaterial>>,
    // ) {
    //     let mut x = [0.0; N_1D];
    //     let mut u = [0.0; N_1D];

    //     let start = PI * -2.0;
    //     let stop = PI * 2.0;
    //     let space_step = (stop - start) / (N_1D as f32);

    //     let mut i = 0;
    //     let mut cur = start;
    //     // Init x points
    //     while cur < stop {
    //         x[i] = cur;
    //         cur += space_step;
    //         i += 1;
    //     }

    //     // Init displacement
    //     for (i, x) in x.iter().enumerate() {
    //         let x = *x;
    //         if x < -PI / 8.0 || x > PI / 8.0 {
    //             u[i] = cos(x * 4.0);
    //         }
    //     }

    //     commands.spawn((
    //         WaveField1D::<N_1D>::new(u, space_step, 0.6, 0.9995),
    //         DeformablePlane,
    //         Mesh3d(
    //             meshes.add(
    //                 Plane3d::default()
    //                     .mesh()
    //                     .size(50.0, 50.0)
    //                     .subdivisions(N_1D as u32),
    //             ),
    //         ),
    //         MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    //         Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::IDENTITY),
    //     ));
    // }

    fn spawn_wave_field_2d(
        mut commands: Commands,
        sim_config: Res<SimConfig>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands.spawn((
            WaveField2D::<N_2D>::new(
                [0.0; N_2D],
                sim_config.space_step,
                sim_config.wave_speed,
                sim_config.damping,
            ),
            DeformablePlane,
            Mesh3d(
                meshes.add(
                    Plane3d::default()
                        .mesh()
                        .size(50.0, 50.0)
                        .subdivisions(N_1D as u32),
                ),
            ),
            MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
            Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::IDENTITY),
        ));
    }
}
