use bevy::{
    app::Plugin,
    asset::Handle,
    ecs::{
        component::Component,
        entity::Entity,
        event::{EntityEvent, Event},
        observer::On,
    },
    math::Vec3,
    mesh::{Mesh, Mesh3d},
    pbr::{MeshMaterial3d, StandardMaterial},
    scene::{Scene, bsn},
    sprite_render::ColorMaterial,
    transform::components::Transform,
};

#[derive(Component, Clone, Default)]
pub struct SurfacePixel;

pub struct SurfacePlugin;
impl SurfacePlugin {
    fn pixel(pos: Vec3, mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> impl Scene {
        bsn! {
            SurfacePixel
            Transform::from_translation(pos)
            Mesh3d(mesh)
            MeshMaterial3d<StandardMaterial>(material)
        }
    }

    fn spawn_surface() {}

    fn apply_update(update: On<SurfaceUpdate>) {
        for val in update.surface.iter() {}
    }
}
impl Plugin for SurfacePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_observer(Self::apply_update);
    }
}

#[derive(Event)]
struct SurfaceUpdate {
    surface: Vec<f32>,
}
