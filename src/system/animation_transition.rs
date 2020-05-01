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
    _marker: PhantomData<T>,
}

impl<T> AnimationTransitionSystem<T> {
    pub fn new() -> Self {
        AnimationTransitionSystem {
            _marker: PhantomData,
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
        WriteStorage<'s, PlayAnimationKey<T>>,
        Read<'s, AssetStorage<AnimationData<T>>>,
        Read<'s, AnimationStore<T>>,
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
            let current_time = match time {
                AnimationTime::Play { current_time, .. } => *current_time,
                AnimationTime::Stop { stopped_time, .. } => *stopped_time,
            };
            let (id, pack_id, anim_id) = match key.play_key() {
                Some((id, pack, anim)) => (id, pack, anim),
                None => continue,
            };
            let animation = match animation_store
                .get_animation_handle(id)
                .and_then(|handle| sprite_animation_storage.get(handle))
                .and_then(|data| data.pack(pack_id))
                .and_then(|pack| pack.animation(anim_id))
            {
                Some(animation) => animation,
                None => {
                    log::error!("animation not found: {:?}", (id, pack_id, anim_id));
                    continue;
                }
            };

            // ステート変化に関連する情報はルートにのみ入れる
            let frame = animation.sec_to_frame(current_time);
            let root_user = animation.user(0, frame);
            let rest_time = animation.total_secs() - current_time;

            match T::translate_animation(e, rest_time, (pack_id, anim_id), root_user, &optional) {
                Some((next_pack, next_anim, next_frame)) => {
                    let fps = animation.fps();
                    let next_time = 1.0 / (fps as f32) * (next_frame as f32);
                    time.set_play_time(next_time);
                    key.set_pack(next_pack);
                    key.set_animation(next_anim);
                }
                None => {}
            }
        }
    }
}
