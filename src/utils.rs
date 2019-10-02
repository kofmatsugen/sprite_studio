use crate::resource::AnimationStore;
use amethyst::ecs::Read;

use crate::traits::{AnimationKey, AnimationUser};

pub fn get_timeline<K, U>(_animation_store: &Read<AnimationStore<K, U>>)
where
    K: AnimationKey,
    U: AnimationUser,
{
}
