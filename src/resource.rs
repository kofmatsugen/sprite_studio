pub mod animation;
pub mod data;
pub mod name;
pub mod pack;
pub mod part;
mod part_timeline;
pub mod timeline;

use crate::traits::animation_file::AnimationFile;
use amethyst::{assets::Handle, renderer::sprite::SpriteSheetHandle};
use std::collections::BTreeMap;

pub type AnimationHandle<T> = Handle<data::AnimationData<T>>;
pub struct AnimationStore<T>
where
    T: AnimationFile,
{
    pub(crate) animations: BTreeMap<T::FileId, AnimationHandle<T>>,
    pub(crate) sprite_sheets: BTreeMap<T::FileId, Vec<SpriteSheetHandle>>,
}

impl<T> Default for AnimationStore<T>
where
    T: AnimationFile,
{
    fn default() -> Self {
        AnimationStore {
            animations: BTreeMap::new(),
            sprite_sheets: BTreeMap::new(),
        }
    }
}

impl<T> AnimationStore<T>
where
    T: AnimationFile,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_animation_handle(&self, id: &T::FileId) -> Option<&AnimationHandle<T>> {
        self.animations.get(id)
    }

    pub fn get_sprite_handle(&self, id: &T::FileId, map_id: usize) -> Option<&SpriteSheetHandle> {
        self.sprite_sheets
            .get(id)
            .and_then(|sprite_sheets| sprite_sheets.get(map_id))
    }

    // ステートの終わりなどで開放したい場合はここで
    pub fn unload_file(&mut self, id: &T::FileId) {
        self.animations.remove(id);
        self.sprite_sheets.remove(id);
    }
}
