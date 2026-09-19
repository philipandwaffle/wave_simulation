use bevy::{ecs::entity::Entities, platform::collections::HashMap};

// pub trait Medium<K,V> {
//     pub
// }
pub struct Medium<K> {
    pixels: HashMap<K, Entities>,
}
