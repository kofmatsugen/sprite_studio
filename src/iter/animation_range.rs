use crate::{
    iter::AnimationNodes,
    resource::AnimationStore,
    traits::{AnimationKey, AnimationUser},
    SpriteAnimation,
};
use amethyst::assets::AssetStorage;

pub struct AnimationRange<'a, U>
where
    U: AnimationUser,
{
    nodes: Vec<AnimationNodes<'a, U>>,
}

impl<'a, U> AnimationRange<'a, U>
where
    U: AnimationUser,
{
    pub fn new<K>(
        data_key: (&'a K, usize, usize),
        start: f32,
        end: f32,
        animation_store: &'a AnimationStore<K, U>,
        storage: &'a AssetStorage<SpriteAnimation<U>>,
    ) -> Option<Self>
    where
        K: AnimationKey,
    {
        AnimationRange {
            nodes: AnimationNodes::range(data_key, start, end, animation_store, storage)?,
        }
        .into()
    }
}

impl<'a, U> Iterator for AnimationRange<'a, U>
where
    U: AnimationUser,
{
    type Item = AnimationNodes<'a, U>;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.pop()
    }
}
