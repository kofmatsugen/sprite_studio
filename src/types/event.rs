use crate::traits::animation_file::AnimationFile;
use amethyst::{ecs::Entity, shrev::EventChannel};

pub type AnimationEventChannel<T> = EventChannel<AnimationEvent<T>>;

// アニメーション再生時と終了時のイベント
pub enum AnimationEvent<T>
where
    T: AnimationFile,
{
    // アニメーション再生が終わったときにそのキーを通知
    // 別アニメーション再生時も呼び出される
    End {
        entity: Entity,
        file_id: T::FileId,
        pack: T::PackKey,
        animation: T::AnimationKey,
    },
    // アニメーションのキーが変更されたときに変更先キーを通知
    ChangeKey {
        entity: Entity,
        file_id: T::FileId,
        pack: T::PackKey,
        animation: T::AnimationKey,
    },
}
