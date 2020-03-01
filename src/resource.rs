pub mod animation;
pub mod data;
pub mod name;
pub mod pack;
pub mod part;
mod part_timeline;
pub mod timeline;

use crate::traits::{AnimationKey, AnimationUser, FileId};
use amethyst::{assets::Handle, renderer::sprite::SpriteSheetHandle};
use std::collections::BTreeMap;

pub type AnimationHandle<U, P, A> = Handle<data::AnimationData<U, P, A>>;
pub struct AnimationStore<ID, U, P, A>
where
    ID: FileId,
    P: AnimationKey,
    A: AnimationKey,
{
    pub(crate) animations: BTreeMap<ID, AnimationHandle<U, P, A>>,
    pub(crate) sprite_sheets: BTreeMap<ID, Vec<SpriteSheetHandle>>,
}

impl<ID, U, P, A> Default for AnimationStore<ID, U, P, A>
where
    ID: FileId,
    U: AnimationUser,
    P: AnimationKey,
    A: AnimationKey,
{
    fn default() -> Self {
        AnimationStore {
            animations: BTreeMap::new(),
            sprite_sheets: BTreeMap::new(),
        }
    }
}

impl<ID, U, P, A> AnimationStore<ID, U, P, A>
where
    ID: FileId,
    U: AnimationUser,
    P: AnimationKey,
    A: AnimationKey,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_animation_handle(&self, id: &ID) -> Option<&AnimationHandle<U, P, A>> {
        self.animations.get(id)
    }

    pub fn get_sprite_handle(&self, id: &ID, map_id: usize) -> Option<&SpriteSheetHandle> {
        self.sprite_sheets
            .get(id)
            .and_then(|sprite_sheets| sprite_sheets.get(map_id))
    }
}
