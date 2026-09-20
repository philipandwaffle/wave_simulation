use std::path::Path;

use bevy::app::Plugin;
use bevy::math::Vec2;
use config::config_tag::ConfigTag;
use config::wrapper::Wrapper;
use config::{ConfigTag, config_tag::Config};
use serde::{Deserialize, Serialize};

use crate::handles::HandlesPluginType;
use crate::scalar_wave::plugin::ScalarWavePlugin;
use crate::scalar_wave::sim_config::Simulation;

type WrappedConfig = Wrapper<Configuration>;

#[derive(ConfigTag, Serialize, Deserialize)]
pub struct Configuration {
    handles: HandlesPluginType,
    scalar_wave: Option<ScalarWavePlugin>,
    simulations: Option<Vec<SimulationConfig>>,
}

#[derive(ConfigTag, Serialize, Deserialize)]
pub struct SimulationConfig {
    title: String,
    features: Vec<Feature>,
}
impl Into<Simulation> for SimulationConfig {
    fn into(self) -> Simulation {
        todo!()
    }
}

#[derive(ConfigTag, Serialize, Deserialize)]
pub enum Feature {
    CosineDot {
        max_magnitude: f32,
        centre: Vec2,
        radius: f32,
    },
    CosineRing {
        max_magnitude: f32,
        centre: Vec2,
        width: f32,
        radius: f32,
    },
    CosineLine {
        max_magnitude: f32,
        dist: f32,
        width: f32,
        x_axis: bool,
    },
}

pub struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let config = WrappedConfig::load_cfg(Path::new("config.json")).config;
        app.add_plugins(config.handles);

        if let Some(scalar_wave_plugin) = config.scalar_wave {
            app.add_plugins(scalar_wave_plugin);
        }
    }
}
