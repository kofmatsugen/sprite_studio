use amethyst::ecs::{Component, FlaggedStorage};

pub type AnimationId = usize;
pub type PackId = usize;

pub struct PlayAnimationKey<K> {
    animation_key: Option<(K, PackId, AnimationId)>,
}

impl<K> PlayAnimationKey<K> {
    pub fn new() -> Self {
        PlayAnimationKey {
            animation_key: None,
        }
    }

    pub fn set_key<T>(&mut self, key: T)
    where
        T: Into<Option<(K, PackId, AnimationId)>>,
    {
        self.animation_key = key.into();
    }

    pub fn key(&self) -> Option<&(K, PackId, AnimationId)> {
        self.animation_key.as_ref()
    }
}

impl<K> Component for PlayAnimationKey<K>
where
    K: 'static + Send + Sync,
{
    type Storage = FlaggedStorage<Self>;
}
