// use std::collections::HashMap;

// use bevy::{
//     asset::{Assets, Handle},
//     color::{Color, Hue},
//     ecs::resource::Resource,
//     math::primitives::{Circle, Rectangle, RegularPolygon},
//     mesh::{Mesh, Mesh2d},
//     pbr::{MeshMaterial3d, StandardMaterial},
//     sprite_render::{ColorMaterial, MeshMaterial2d},
// };

// #[derive(Eq, Hash, PartialEq)]
// pub enum MeshKey {
//     Circle,
//     Rectangle,
//     Hex,
//     Triangle,
// }
// #[derive(Eq, Hash, PartialEq)]
// pub enum ColourMaterialKey {
//     Red,
//     Orange,
//     Yellow,
//     SpringGreen,
//     Green,
//     OceanGreen,
//     Cyan,
//     SkyBlue,
//     Blue,
//     Purple,
//     Magenta,
//     Crimson,
//     White,
//     LightGrey,
//     Grey,
//     DarkGrey,
//     Black,
// }

// #[derive(Resource)]
// pub struct Handles {
//     pub meshes: HashMap<MeshKey, Handle<Mesh>>,
//     pub colour_materials: HashMap<ColourMaterialKey, Handle<ColorMaterial>>,
//     pub standard_materials: HashMap<ColourMaterialKey, Handle<StandardMaterial>>,
//     pub colours: HashMap<ColourMaterialKey, Color>,
// }
// impl Handles {
//     pub fn new() -> Self {
//         return Handles {
//             meshes: HashMap::new(),
//             colour_materials: HashMap::new(),
//             standard_materials: HashMap::new(),
//             colours: HashMap::new(),
//         };
//     }

//     pub fn get_mesh_handle(&self, k: &MeshKey) -> Handle<Mesh> {
//         return self.meshes[k].clone();
//     }
//     pub fn get_mesh2d(&self, k: &MeshKey) -> Mesh2d {
//         return Mesh2d(self.get_mesh_handle(k));
//     }

//     pub fn get_colour_material_handle(&self, k: &ColourMaterialKey) -> Handle<ColorMaterial> {
//         return self.colour_materials[k].clone();
//     }
//     pub fn get_mesh_material_2d(&self, k: &ColourMaterialKey) -> MeshMaterial2d<ColorMaterial> {
//         return MeshMaterial2d(self.get_colour_material_handle(k));
//     }

//     pub fn get_standard_material_handle(&self, k: &ColourMaterialKey) -> Handle<StandardMaterial> {
//         return self.standard_materials[k].clone();
//     }
//     pub fn get_standard_material_3d(
//         &self,
//         k: &ColourMaterialKey,
//     ) -> MeshMaterial3d<StandardMaterial> {
//         return MeshMaterial3d(self.get_standard_material_handle(k));
//     }

//     pub fn insert_circle_mesh(&mut self, key: MeshKey, meshes: &mut Assets<Mesh>, radius: f32) {
//         self.meshes
//             .insert(key, meshes.add(Circle::new(radius)).into());
//     }
//     pub fn insert_rectangle_mesh(
//         &mut self,
//         key: MeshKey,
//         meshes: &mut Assets<Mesh>,
//         w: f32,
//         h: f32,
//     ) {
//         self.meshes
//             .insert(key, meshes.add(Rectangle::new(w, h)).into());
//     }

//     pub fn insert_poly_mesh(
//         &mut self,
//         key: MeshKey,
//         meshes: &mut Assets<Mesh>,
//         r: f32,
//         sides: u32,
//     ) {
//         self.meshes
//             .insert(key, meshes.add(RegularPolygon::new(r, sides)).into());
//     }

//     pub fn insert_color_material(
//         &mut self,
//         key: ColourMaterialKey,
//         mats: &mut Assets<ColorMaterial>,
//         colour: Color,
//     ) {
//         self.colour_materials.insert(key, mats.add(colour).into());
//     }

//     pub fn insert_standard_material(
//         &mut self,
//         key: ColourMaterialKey,
//         mats: &mut Assets<StandardMaterial>,
//         colour: Color,
//     ) {
//         self.standard_materials.insert(key, mats.add(colour).into());
//     }

//     pub fn setup_meshes(&mut self, meshes: &mut Assets<Mesh>) {
//         self.insert_circle_mesh(MeshKey::Circle, meshes, 1.0);
//         self.insert_rectangle_mesh(MeshKey::Rectangle, meshes, 1.0, 1.0);
//         self.insert_poly_mesh(MeshKey::Hex, meshes, 1.0, 6);
//         self.insert_poly_mesh(MeshKey::Triangle, meshes, 1.0, 3);
//     }

//     pub fn setup_materials(&mut self, mats: &mut Assets<ColorMaterial>) {
//         let mut color = Color::hsla(0.0, 1.0, 0.5, 1.0);

//         self.insert_color_material(ColourMaterialKey::Red, mats, color);
//         self.colours.insert(ColourMaterialKey::Red, color);
//         self.insert_standard_material(ColourMaterialKey::Red, mats, color);
//         self.colours.insert(ColourMaterialKey::Red, color);
//         color = color.rotate_hue(30.0);

//         self.insert_color_material(ColourMaterialKey::Orange, mats, color);
//         self.colours.insert(ColourMaterialKey::Orange, color);
//         color = color.rotate_hue(30.0);

//         self.insert_color_material(ColourMaterialKey::Yellow, mats, color);
//         self.colours.insert(ColourMaterialKey::Yellow, color);
//         color = color.rotate_hue(30.0);

//         self.insert_color_material(ColourMaterialKey::SpringGreen, mats, color);
//         self.colours.insert(ColourMaterialKey::SpringGreen, color);
//         color = color.rotate_hue(30.0);

//         self.insert_color_material(ColourMaterialKey::Green, mats, color);
//         self.colours.insert(ColourMaterialKey::Green, color);
//         color = color.rotate_hue(30.0);

//         self.insert_color_material(ColourMaterialKey::OceanGreen, mats, color);
//         self.colours.insert(ColourMaterialKey::OceanGreen, color);
//         color = color.rotate_hue(30.0);

//         self.insert_color_material(ColourMaterialKey::Cyan, mats, color);
//         self.colours.insert(ColourMaterialKey::Cyan, color);
//         color = color.rotate_hue(30.0);

//         self.insert_color_material(ColourMaterialKey::SkyBlue, mats, color);
//         self.colours.insert(ColourMaterialKey::SkyBlue, color);
//         color = color.rotate_hue(30.0);

//         self.insert_color_material(ColourMaterialKey::Blue, mats, color);
//         self.colours.insert(ColourMaterialKey::Blue, color);
//         color = color.rotate_hue(30.0);

//         self.insert_color_material(ColourMaterialKey::Purple, mats, color);
//         self.colours.insert(ColourMaterialKey::Purple, color);
//         color = color.rotate_hue(30.0);

//         self.insert_color_material(ColourMaterialKey::Magenta, mats, color);
//         self.colours.insert(ColourMaterialKey::Magenta, color);
//         color = color.rotate_hue(30.0);

//         self.insert_color_material(ColourMaterialKey::Crimson, mats, color);
//         self.colours.insert(ColourMaterialKey::Crimson, color);

//         self.insert_color_material(
//             ColourMaterialKey::White,
//             mats,
//             Color::hsla(0.0, 0.0, 1.0, 1.0),
//         );
//         self.insert_color_material(
//             ColourMaterialKey::LightGrey,
//             mats,
//             Color::hsla(0.0, 0.0, 0.75, 1.0),
//         );
//         self.insert_color_material(
//             ColourMaterialKey::Grey,
//             mats,
//             Color::hsla(0.0, 0.0, 0.5, 1.0),
//         );
//         self.insert_color_material(
//             ColourMaterialKey::DarkGrey,
//             mats,
//             Color::hsla(0.0, 0.0, 0.25, 1.0),
//         );
//         self.insert_color_material(
//             ColourMaterialKey::Black,
//             mats,
//             Color::hsla(0.0, 0.0, 0.0, 1.0),
//         );
//     }
// }
