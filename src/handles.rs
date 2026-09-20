use handles::{handles::Handles, plugin::HandlesPlugin};
use serde::{Deserialize, Serialize};

pub type HandlesPluginType = HandlesPlugin<ColorKey, MeshKey, StandardMaterialKey>;
pub type HandlesResource = Handles<StandardMaterialKey, MeshKey>;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum ColorKey {
    DefaultSurfacePixelColor,
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum StandardMaterialKey {
    SurfacePixel,
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum MeshKey {
    SurfacePixel,
}
