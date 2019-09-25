use crate::{
    components::{AnimationTime, PlayAnimationKey},
    resource::AnimationStore,
    types::from_user::FromUser,
    SpriteAnimation,
};
use amethyst::{
    assets::AssetStorage,
    core::ParentHierarchy,
    ecs::{
        storage::ComponentEvent, BitSet, Entities, LazyUpdate, Read, ReadExpect, ReadStorage,
        ReaderId, System, SystemData, World, WriteStorage,
    },
};
use serde::Serialize;
use std::marker::PhantomData;

pub struct TimeLineApplySystem<K, U> {
    _key: PhantomData<K>,
    _user: PhantomData<U>,
    dirty: BitSet,
    reader_id: Option<ReaderId<ComponentEvent>>,
}

impl<K, U> TimeLineApplySystem<K, U> {
    pub fn new() -> Self {
        TimeLineApplySystem {
            _key: PhantomData,
            _user: PhantomData,
            dirty: BitSet::new(),
            reader_id: None,
        }
    }
}

impl<'s, K, U> System<'s> for TimeLineApplySystem<K, U>
where
    K: 'static + Send + Sync + std::hash::Hash + PartialOrd + Ord + std::fmt::Debug,
    U: 'static + Send + Sync + FromUser + Serialize,
{
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Read<'s, AnimationStore<K, U>>,
        Read<'s, AssetStorage<SpriteAnimation<U>>>,
        ReadExpect<'s, ParentHierarchy>,
        ReadStorage<'s, AnimationTime>,
        ReadStorage<'s, PlayAnimationKey<K>>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader_id = Some(WriteStorage::<PlayAnimationKey<K>>::fetch(world).register_reader());
    }

    fn run(
        &mut self,
        (
            _entities,
            _lazy,
            _animation_store,
            _sprite_animation_storage,
            _parent_hierarchy,
            _animation_times,
            animation_keys,
        ): Self::SystemData,
    ) {
        self.dirty.clear();

        let events = animation_keys
            .channel()
            .read(self.reader_id.as_mut().unwrap());
        for e in events {
            match e {
                ComponentEvent::Modified(id) | ComponentEvent::Inserted(id) => {
                    self.dirty.add(*id);
                }
                ComponentEvent::Removed(_) => {}
            }
        }
    }
}
