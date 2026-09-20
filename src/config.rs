use std::path::Path;

use bevy::app::Plugin;
use config::config_tag::ConfigTag;
use config::wrapper::Wrapper;
use config::{ConfigTag, config_tag::Config};
use serde::{Deserialize, Serialize};

use crate::handles::HandlesPluginType;
use crate::scalar_wave::plugin::ScalarWavePlugin;

type WrappedConfig = Wrapper<Configuration>;

#[derive(ConfigTag, Serialize, Deserialize)]
pub struct Configuration {
    handles: HandlesPluginType,
    scalar_wave: Option<ScalarWavePlugin>,
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
