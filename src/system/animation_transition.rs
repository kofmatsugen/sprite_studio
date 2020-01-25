use crate::{
    components::{AnimationTime, PlayAnimationKey},
    resource::{data::AnimationData, AnimationStore},
    traits::translate_animation::TranslateAnimation,
};
use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Join, Read, System, SystemData, World, WriteStorage},
};
use std::marker::PhantomData;

pub struct AnimationTransitionSystem<T> {
    _translate: PhantomData<T>,
}

impl<T> AnimationTransitionSystem<T> {
    pub fn new() -> Self {
        AnimationTransitionSystem {
            _translate: PhantomData,
        }
    }
}

impl<'s, T> System<'s> for AnimationTransitionSystem<T>
where
    T: TranslateAnimation<'s>,
{
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, AnimationTime>,
        WriteStorage<'s, PlayAnimationKey<T::FileId, T::PackKey, T::AnimationKey>>,
        Read<'s, AssetStorage<AnimationData<T::UserData>>>,
        Read<'s, AnimationStore<T::FileId, T::UserData>>,
        T::OptionalData,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }

    fn run(
        &mut self,
        (
            entities,
            mut animation_times,
            mut play_key,
            sprite_animation_storage,
            animation_store,
            optional,
        ): Self::SystemData,
    ) {
        for (e, time, key) in (&*entities, &mut animation_times, &mut play_key).join() {
            let (id, pack_id, anim_id) =
                match (key.file_id(), key.pack_name(), key.animation_name()) {
                    (id, Some(pack), Some(anim)) => (id, pack, anim),
                    _ => continue,
                };
            let current_time = time.current_time();
            let animation = animation_store
                .get_animation_handle(id)
                .and_then(|handle| sprite_animation_storage.get(handle))
                .and_then(|data| data.pack(&pack_id.to_string()))
                .and_then(|pack| pack.animation(&anim_id.to_string()))
                .unwrap();

            // ステート変化に関連する情報はルートにのみ入れる
            let frame = animation.sec_to_frame(current_time);
            let root_user = animation.user(0, frame);
            let rest_time = animation.total_secs() - current_time;

            match T::translate_animation(e, rest_time, (pack_id, anim_id), root_user, &optional) {
                Some((next_pack, next_anim, next_frame)) => {
                    let fps = animation.fps();
                    let next_time = 1.0 / (fps as f32) * (next_frame as f32);
                    time.set_time(next_time);
                    key.set_pack(next_pack);
                    key.set_animation(next_anim);
                }
                None => {}
            }
        }
    }
}
