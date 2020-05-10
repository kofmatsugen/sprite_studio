use crate::{
    components::{AnimationTime, PlayAnimationKey},
    resource::{data::AnimationData, AnimationStore},
    traits::translate_animation::TranslateAnimation,
    types::event::{AnimationEvent, AnimationEventChannel},
};
use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Join, Read, System, Write, WriteStorage},
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
        Write<'s, AnimationEventChannel<T>>,
        T::OptionalData,
    );

    fn run(
        &mut self,
        (
            entities,
            mut animation_times,
            mut play_key,
            sprite_animation_storage,
            animation_store,
            mut channel,
            optional,
        ): Self::SystemData,
    ) {
        for (e, time) in (&*entities, &mut animation_times).join() {
            if time.is_play() == false {
                // アニメーションが停止中なら遷移もしない
                continue;
            }
            let (id, pack_id, anim_id) = match play_key.get(e).and_then(|key| key.play_key()) {
                Some((&id, &pack, &anim)) => (id, pack, anim),
                None => continue,
            };
            let animation = match animation_store
                .get_animation_handle(&id)
                .and_then(|handle| sprite_animation_storage.get(handle))
                .and_then(|data| data.pack(&pack_id))
                .and_then(|pack| pack.animation(&anim_id))
            {
                Some(animation) => animation,
                None => {
                    log::error!("animation not found: {:?}", (id, pack_id, anim_id));
                    continue;
                }
            };

            let frame = time.play_frame(animation.fps() as f32);
            // ステート変化に関連する情報はルートにのみ入れる
            let root_user = animation.user(crate::constant::ROOT_PART_ID, frame);

            // 現在のフレームが総フレーム以上だった場合はアニメーション自体は終了している
            let rest_frame = if frame >= animation.total_frame() {
                None
            } else {
                Some(animation.total_frame() - frame)
            };

            match T::translate_animation(e, rest_frame, (&pack_id, &anim_id), root_user, &optional)
            {
                Some((next_pack, next_anim, next_frame)) => {
                    let fps = animation.fps() as f32;
                    // 次のアニメーションのフレーム数が来るので実時間に変換
                    let next_time = 1.0 / fps * (next_frame as f32);
                    // 次アニメーションに遷移する際に現在のフレームから超過した時間は次のアニメーションの開始オフセットになる
                    let offset_time = time.play_time() - frame as f32 / fps;
                    time.set_play_time(next_time + offset_time);
                    if let Some(key) = play_key.get_mut(e) {
                        key.set_pack(next_pack);
                        key.set_animation(next_anim);
                    }

                    // 切り替わったので今再生中のアニメーションは終了
                    channel.single_write(AnimationEvent::End {
                        entity: e,
                        file_id: id,
                        pack: pack_id,
                        animation: anim_id,
                    });

                    // アニメーションキー変更
                    channel.single_write(AnimationEvent::ChangeKey {
                        entity: e,
                        file_id: id,
                        pack: next_pack,
                        animation: next_anim,
                    });
                }
                None => {
                    if rest_frame.is_none() {
                        // 再生時間を超えてたらイベント通知
                        channel.single_write(AnimationEvent::End {
                            entity: e,
                            file_id: id,
                            pack: pack_id,
                            animation: anim_id,
                        });
                    }
                }
            }
        }
    }
}
