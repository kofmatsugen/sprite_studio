use amethyst::ecs::{Component, DenseVecStorage};
use std::time::Duration;

pub struct AnimationPlayer<K> {
    pack_key: K,
    current_time: Duration,
    prev_time: Option<Duration>,
}

impl<K> AnimationPlayer<K> {
    pub fn new(pack_key: K) -> Self {
        AnimationPlayer {
            pack_key,
            current_time: Duration::default(),
            prev_time: None,
        }
    }

    pub fn set_pack_key(&mut self, pack_key: K) {
        self.pack_key = pack_key;
    }

    pub fn set_time(&mut self, time: Duration) {
        self.prev_time = self.current_time.into();
        self.current_time = time;
    }
}

impl<K> Component for AnimationPlayer<K>
where
    K: 'static + Send + Sync,
{
    type Storage = DenseVecStorage<Self>;
}
