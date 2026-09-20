use bevy::{
    animation::transition,
    app::Plugin,
    asset::{Assets, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        event::{EntityEvent, Event},
        observer::On,
        query::With,
        resource::Resource,
        system::{Commands, Query, Res, ResMut},
    },
    math::{USizeVec2, Vec2, Vec3, vec2, vec3},
    mesh::{Mesh, Mesh3d},
    pbr::{MeshMaterial3d, StandardMaterial},
    scene::{CommandsSceneExt, Scene, bsn},
    sprite_render::ColorMaterial,
    transform::components::Transform,
};

use crate::{
    display::surface,
    handles::{HandlesResource, MeshKey, StandardMaterialKey},
};

#[derive(Resource, Clone, Default)]
pub struct Surface {
    pixels: Vec<Entity>,
}
impl Surface {
    pub fn new(num_pixels: usize) -> Self {
        Self {
            pixels: vec![Entity::PLACEHOLDER; num_pixels],
        }
    }
}

#[derive(Component, Clone, Default)]
pub struct SurfacePixel;

pub struct SurfacePlugin;
impl SurfacePlugin {
    fn pixel(pos: Vec3, meshes: &mut Assets<Mesh>, handles: &HandlesResource) -> impl Scene {
        let unique_mesh = if let Some(mesh) = meshes.get(handles.mesh(&MeshKey::SurfacePixel).id())
        {
            meshes.add(mesh.clone())
        } else {
            panic!("Unable to get common mesh for surface pixel")
        };

        let material = handles.standard_material(&StandardMaterialKey::GreenMaterial);

        bsn! {
            SurfacePixel
            Transform::from_translation(pos)
            Mesh3d(unique_mesh)
            MeshMaterial3d<StandardMaterial>(material)
        }
    }

    fn spawn_surface(
        surface_spawn: On<SurfaceSpawn>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut surface: ResMut<Surface>,
        handles: Res<HandlesResource>,
    ) {
        let mut new_surface = Surface::new(surface_spawn.resolution.x * surface_spawn.resolution.y);
        let (x_len, y_len) = (surface_spawn.resolution.x, surface_spawn.resolution.y);

        let mut cur_pos = surface_spawn.centre - (surface_spawn.size * 0.5);
        let step = vec2(
            surface_spawn.size.x / x_len as f32,
            surface_spawn.size.y / y_len as f32,
        );
        for j in 0..y_len {
            for i in 0..x_len {
                new_surface.pixels[i + j * x_len] = commands
                    .spawn_scene(Self::pixel(
                        vec3(cur_pos.x, 0.0, cur_pos.y),
                        &mut meshes,
                        &handles,
                    ))
                    .id();
                cur_pos.x += step.x;
            }
            cur_pos.x = (surface_spawn.size * -0.5).x;
            cur_pos.y += step.y;
        }

        *surface = new_surface;
    }

    fn apply_update(
        update: On<SurfaceUpdate>,
        mut meshes: ResMut<Assets<Mesh>>,
        surface: Res<Surface>,
        // surface_pixels: Query<&Mesh3d, With<SurfacePixel>>,
        mut surface_pixels: Query<&mut Transform, With<SurfacePixel>>,
    ) {
        for (index, val) in update.surface.iter().enumerate() {
            let Ok(mut tranform) = surface_pixels.get_mut(surface.pixels[index]) else {
                continue;
            };
            tranform.translation.y = *val;
        }
    }
}
impl Plugin for SurfacePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<Surface>()
            .add_observer(Self::apply_update)
            .add_observer(Self::spawn_surface);
    }
}

#[derive(Event)]
pub struct SurfaceUpdate {
    surface: Vec<f32>,
}
impl SurfaceUpdate {
    pub fn new(surface: Vec<f32>) -> Self {
        Self { surface }
    }
}

#[derive(Event)]
pub struct SurfaceSpawn {
    size: Vec2,
    centre: Vec2,
    resolution: USizeVec2,
}
impl SurfaceSpawn {
    pub fn new(size: Vec2, centre: Vec2, resolution: USizeVec2) -> Self {
        Self {
            size,
            centre,
            resolution,
        }
    }
}
