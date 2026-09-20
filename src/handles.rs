use handles::{handles::Handles, plugin::HandlesPlugin};
use serde::{Deserialize, Serialize};

pub type HandlesPluginType = HandlesPlugin<ColorKey, MeshKey, StandardMaterialKey>;
pub type HandlesResource = Handles<StandardMaterialKey, MeshKey>;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum ColorKey {
    Red,
    Orange,
    Yellow,
    SpringGreen,
    Green,
    OceanGreen,
    Cyan,
    SkyBlue,
    Blue,
    Purple,
    Magenta,
    Crimson,
    White,
    LightGrey,
    Grey,
    DarkGrey,
    Black,
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum StandardMaterialKey {
    RedMaterial,
    OrangeMaterial,
    YellowMaterial,
    SpringGreenMaterial,
    GreenMaterial,
    OceanGreenMaterial,
    CyanMaterial,
    SkyBlueMaterial,
    BlueMaterial,
    PurpleMaterial,
    MagentaMaterial,
    CrimsonMaterial,
    WhiteMaterial,
    LightGreyMaterial,
    GreyMaterial,
    DarkGreyMaterial,
    BlackMaterial,
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum MeshKey {
    SurfacePixel,
}
