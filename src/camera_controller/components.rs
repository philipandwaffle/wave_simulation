use bevy::ecs::component::Component;

#[derive(Component, Clone, Default)]
pub struct RotatingCamera;

#[derive(Component, Clone, Default)]
pub struct TranslatingCamera;

#[derive(Component, Clone, Default)]
pub struct MainCamera;
