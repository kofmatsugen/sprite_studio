use crate::{
    components::{AnimationTime, PlayAnimationKey},
    resource::{data::AnimationData, AnimationStore},
    traits::animation_file::AnimationFile,
};
use amethyst::{
    assets::AssetStorage,
    core::{Time, Transform},
    ecs::{Entities, Entity, Join, Read, ReadStorage, System, WriteStorage},
};
use std::marker::PhantomData;

// アニメーションのキーを変更したときにイベントを発行する
pub struct RootTranslateSystem<T>
where
    T: 'static + Send + Sync,
{
    _translation: PhantomData<T>,
}

impl<T> RootTranslateSystem<T>
where
    T: 'static + Send + Sync,
{
    pub fn new() -> Self {
        RootTranslateSystem {
            _translation: PhantomData,
        }
    }
}

impl<'s, T> System<'s> for RootTranslateSystem<T>
where
    T: AnimationFile,
{
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, AnimationTime>,
        ReadStorage<'s, PlayAnimationKey<T>>,
        WriteStorage<'s, Transform>,
        Read<'s, AssetStorage<AnimationData<T>>>,
        Read<'s, AnimationStore<T>>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, play_time, key, mut transforms, storage, store, time): Self::SystemData,
    ) {
        for (e, play_time, key) in (&*entities, &play_time, &key).join() {
            update_root_translate(
                e,
                play_time,
                key,
                &mut transforms,
                &store,
                &storage,
                time.frame_number(),
            );
        }
    }
}

fn update_root_translate<T>(
    entity: Entity,
    time: &AnimationTime,
    key: &PlayAnimationKey<T>,
    transforms: &mut WriteStorage<Transform>,
    store: &AnimationStore<T>,
    animation_storage: &AssetStorage<AnimationData<T>>,
    frame_number: u64,
) -> Option<()>
where
    T: AnimationFile,
{
    let (id, pack_id, animation_id) = key.play_key()?;

    let handle = store.get_animation_handle(id)?;
    let pack = animation_storage.get(handle)?.pack(pack_id)?;
    let animation = pack.animation(animation_id)?;

    let (current_time, prev_time) = match time {
        AnimationTime::Play {
            current_time,
            prev_time,
            ..
        } => (*current_time, *prev_time),
        AnimationTime::Stop { .. } => None?,
    };

    let current = animation.sec_to_frame(current_time);
    let prev = prev_time.map(|prev_time| animation.sec_to_frame(prev_time));

    let current_transform = animation.local_transform(crate::constant::ROOT_PART_ID, current);
    let prev_transform = prev
        .map(|prev| animation.local_transform(crate::constant::ROOT_PART_ID, prev))
        .unwrap_or(Transform::default());

    let vx = current_transform.translation().x - prev_transform.translation().x;
    let vy = current_transform.translation().y - prev_transform.translation().y;

    if vx == 0. && vy == 0. {
        return None;
    }

    if let Some(transform) = transforms.get_mut(entity) {
        let vx = transform.scale().x * vx;
        let vy = transform.scale().y * vy;

        log::info!(
            "[{} F] root translate [{:?} F => {} F]: ({:.2}, {:.2})",
            frame_number,
            prev,
            current,
            vx,
            vy,
        );

        transform.translation_mut().x += vx;
        transform.translation_mut().y += vy;
    }

    Some(())
}
