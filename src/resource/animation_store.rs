use crate::timeline::{FromUser, SpriteAnimationHandle};
use amethyst::renderer::sprite::SpriteSheetHandle;
use serde::Serialize;
use std::collections::BTreeMap;

pub struct AnimationStore<K, U>
where
    K: std::hash::Hash + PartialOrd + Ord,
    U: FromUser + Serialize,
{
    stores: BTreeMap<K, AnimationData<U>>,
}

impl<K, U> AnimationStore<K, U>
where
    K: std::hash::Hash + PartialOrd + Ord,
    U: FromUser + Serialize,
{
    pub fn insert_animation<N: Into<K>>(
        &mut self,
        data_key: N,
        animation: Vec<SpriteAnimationHandle<U>>,
    ) {
        let key = data_key.into();
        match self.stores.get_mut(&key) {
            Some(data) => data.animations.push(animation),
            None => {
                let mut data = AnimationData::<U>::default();
                data.animations.push(animation);
                self.stores.insert(key, data);
            }
        }
    }

    pub fn insert_sprite_sheet<N: Into<K>>(
        &mut self,
        data_key: N,
        sprite_sheet: SpriteSheetHandle,
    ) {
        let key = data_key.into();
        match self.stores.get_mut(&key) {
            Some(data) => data.sprite_sheets.push(sprite_sheet),
            None => {
                let mut data = AnimationData::<U>::default();
                data.sprite_sheets.push(sprite_sheet);
                self.stores.insert(key, data);
            }
        }
    }

    pub fn animation(&self, data_key: &K) -> Option<&AnimationData<U>> {
        self.stores.get(data_key)
    }
}

impl<K, U> Default for AnimationStore<K, U>
where
    K: std::hash::Hash + PartialOrd + Ord,
    U: FromUser + Serialize,
{
    fn default() -> Self {
        AnimationStore {
            stores: BTreeMap::default(),
        }
    }
}

pub struct AnimationData<U>
where
    U: FromUser + Serialize,
{
    animations: Vec<Vec<SpriteAnimationHandle<U>>>,
    sprite_sheets: Vec<SpriteSheetHandle>,
}

impl<U> Default for AnimationData<U>
where
    U: FromUser + Serialize,
{
    fn default() -> Self {
        AnimationData {
            animations: vec![],
            sprite_sheets: vec![],
        }
    }
}

impl<U> AnimationData<U>
where
    U: FromUser + Serialize,
{
    pub fn animation(&self, pack_id: usize, anim_id: usize) -> Option<&SpriteAnimationHandle<U>> {
        self.animations.get(pack_id).and_then(|anims| anims.get(anim_id))
    }

    pub fn sprite_sheet(&self, id: usize) -> Option<&SpriteSheetHandle> {
        self.sprite_sheets.get(id).map(|handle| handle)
    }
}
