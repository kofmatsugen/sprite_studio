use crate::traits::animation_file::AnimationFile;
use amethyst::{ecs::Entity, shrev::EventChannel};

pub type AnimationEventChannel<T> = EventChannel<AnimationEvent<T>>;

// アニメーション再生時と終了時のイベント
pub enum AnimationEvent<T>
where
    T: AnimationFile,
{
    // 再生開始されたイベント
    Start {
        entity: Entity,
        file_id: T::FileId,
        pack: T::PackKey,
        animation: T::AnimationKey,
    },
    // 終了時のイベント
    End {
        entity: Entity,
        file_id: T::FileId,
        pack: T::PackKey,
        animation: T::AnimationKey,
    },
}
