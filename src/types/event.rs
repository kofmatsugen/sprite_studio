use crate::traits::animation_file::AnimationFile;
use amethyst::{ecs::Entity, shrev::EventChannel};

pub type AnimationEventChannel<T> = EventChannel<AnimationEvent<T>>;

// アニメーション再生時と終了時のイベント
pub enum AnimationEvent<T>
where
    T: AnimationFile,
{
    // 再生時間がアニメーション再生の最初のフレームの場合に呼ばれる
    // 実測のfpsとアニメーションfps が一致しない場合二度以上呼ばれる可能性がある
    Start {
        entity: Entity,
        file_id: T::FileId,
        pack: T::PackKey,
        animation: T::AnimationKey,
    },
    // 再生時間がアニメーション再生時間を超えていた場合に呼ばれる
    // 実測のfpsとアニメーションfps が一致しない場合二度以上呼ばれる可能性がある
    End {
        entity: Entity,
        file_id: T::FileId,
        pack: T::PackKey,
        animation: T::AnimationKey,
    },
    // アニメーションのキーが変更されたとき
    ChangeKey {
        entity: Entity,
        file_id: T::FileId,
        pack: T::PackKey,
        animation: T::AnimationKey,
    },
}
