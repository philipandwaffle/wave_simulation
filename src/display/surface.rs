use bevy::{
    app::Plugin,
    asset::Assets,
    ecs::{
        component::Component,
        entity::Entity,
        event::Event,
        observer::On,
        query::With,
        resource::Resource,
        system::{Commands, Query, Res, ResMut},
    },
    math::{USizeVec2, Vec2, Vec3, usizevec2, vec2, vec3},
    mesh::{Mesh, Mesh3d, VertexAttributeValues},
    pbr::{MeshMaterial3d, StandardMaterial},
    scene::{CommandsSceneExt, Scene, bsn},
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
    fn pixel(
        pos: Vec3,
        scale: Vec2,
        meshes: &mut Assets<Mesh>,
        handles: &HandlesResource,
    ) -> impl Scene {
        let unique_mesh = if let Some(mesh) = meshes.get(handles.mesh(&MeshKey::SurfacePixel).id())
        {
            meshes.add(mesh.clone())
        } else {
            panic!("Unable to get common mesh for surface pixel")
        };

        let material = handles.standard_material(&StandardMaterialKey::SurfacePixel);
        let translation = pos;
        let world_scale = vec3(scale.x, 1.0, scale.y);

        bsn! {
            SurfacePixel
            Mesh3d(unique_mesh)
            Transform {
                translation: {translation},
                scale: {world_scale}
            }
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
        let scale = surface_spawn.size / surface_spawn.resolution.as_vec2();

        for j in 0..y_len {
            for i in 0..x_len {
                new_surface.pixels[i + j * x_len] = commands
                    .spawn_scene(Self::pixel(
                        vec3(cur_pos.x, 0.0, cur_pos.y),
                        scale,
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
        surface_pixels: Query<&Mesh3d, With<SurfacePixel>>,
    ) {
        let resolution = update.resolution;
        for (index, pixel_ent) in surface.pixels.iter().enumerate() {
            let Ok(mesh_handle) = surface_pixels.get(*pixel_ent) else {
                continue;
            };
            let Some(mut mesh_asset) = meshes.get_mut(mesh_handle) else {
                continue;
            };
            if let Some(VertexAttributeValues::Float32x3(positions)) =
                mesh_asset.attribute_mut(Mesh::ATTRIBUTE_POSITION)
            {
                let cur_pos = usizevec2(index % resolution.x, index / resolution.x).as_vec2(); // see note below
                for [x, y, z] in positions.iter_mut() {
                    let fx = cur_pos.x + *x;
                    let fz = cur_pos.y + *z;
                    *y = Self::sample_surface(&update.surface, resolution, fx, fz);
                }
            }
        }
    }

    fn sample_surface(surface: &[f32], resolution: USizeVec2, fx: f32, fz: f32) -> f32 {
        let fx = fx.clamp(0.0, (resolution.x - 1) as f32);
        let fz = fz.clamp(0.0, (resolution.y - 1) as f32);
        let x0 = fx.floor() as usize;
        let z0 = fz.floor() as usize;
        let x1 = (x0 + 1).min(resolution.x - 1);
        let z1 = (z0 + 1).min(resolution.y - 1);
        let tx = fx - x0 as f32;
        let tz = fz - z0 as f32;

        let s00 = surface[x0 + z0 * resolution.x];
        let s10 = surface[x1 + z0 * resolution.x];
        let s01 = surface[x0 + z1 * resolution.x];
        let s11 = surface[x1 + z1 * resolution.x];

        let sx0 = s00 * (1.0 - tx) + s10 * tx;
        let sx1 = s01 * (1.0 - tx) + s11 * tx;
        sx0 * (1.0 - tz) + sx1 * tz
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
    world_size: Vec2,
    world_centre: Vec2,
    resolution: USizeVec2,
}
impl SurfaceUpdate {
    pub fn new(
        surface: Vec<f32>,
        world_size: Vec2,
        world_centre: Vec2,
        resolution: USizeVec2,
    ) -> Self {
        Self {
            surface,
            world_size,
            world_centre,
            resolution,
        }
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
