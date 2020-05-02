use crate::{
    components::PlayAnimationKey,
    traits::animation_file::AnimationFile,
    types::event::{AnimationEvent, AnimationEventChannel},
};
use amethyst::{
    core::timing::Time,
    ecs::{
        storage::ComponentEvent, BitSet, Entities, Join, Read, ReaderId, System, Write,
        WriteStorage,
    },
};
use std::marker::PhantomData;

// アニメーションのキーを変更したときにイベントを発行する
pub struct KeyChangeEventSystem<T>
where
    T: 'static + Send + Sync,
{
    _translation: PhantomData<T>,
    reader: Option<ReaderId<ComponentEvent>>,
    updated: BitSet,
}

impl<T> KeyChangeEventSystem<T>
where
    T: 'static + Send + Sync,
{
    pub fn new() -> Self {
        KeyChangeEventSystem {
            _translation: PhantomData,
            reader: None,
            updated: BitSet::default(),
        }
    }
}

impl<'s, T> System<'s> for KeyChangeEventSystem<T>
where
    T: AnimationFile,
{
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, PlayAnimationKey<T>>,
        Write<'s, AnimationEventChannel<T>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut keys, mut channel, time): Self::SystemData) {
        if self.reader.is_none() {
            self.reader = keys.register_reader().into();
        }

        self.updated.clear();
        for event in keys.channel().read(self.reader.as_mut().unwrap()) {
            match event {
                ComponentEvent::Modified(id) => {
                    self.updated.add(*id);
                }
                ComponentEvent::Inserted(id) => {
                    self.updated.add(*id);
                }
                _ => {}
            }
        }

        for (_, entity, key) in (&self.updated, &*entities, &keys).join() {
            if let Some((&file_id, &pack, &animation)) = key.play_key() {
                log::trace!(
                    "[{} F] change key: entity id = {}, changed = {:?}",
                    time.frame_number(),
                    entity.id(),
                    (file_id, pack, animation,)
                );
                channel.single_write(AnimationEvent::ChangeKey {
                    entity,
                    file_id,
                    pack,
                    animation,
                });
            }
        }
    }
}
