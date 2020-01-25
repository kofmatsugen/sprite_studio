use crate::traits::{AnimationKey, AnimationUser, FileId};
use amethyst::ecs::{Entity, SystemData};

pub trait TranslateAnimation<'s>: 'static + Send + Sync {
    type FileId: FileId;
    type PackKey: AnimationKey;
    type AnimationKey: AnimationKey;
    type UserData: AnimationUser;
    type OptionalData: SystemData<'s>;

    // アニメーション遷移
    fn translate_animation(
        _entity: Entity, // アニメーションを再生してるエンティティ
        rest_time: f32,  // 現在再生中の残り再生時間(負の値の場合は再生終了済み)
        (&current_pack, &current_anim): (&Self::PackKey, &Self::AnimationKey), // 再生中のキー
        _user: Option<&Self::UserData>, // 現在のフレームのユーザーデータ
        _optional: &Self::OptionalData, // 遷移するために必要なシステムデータ
    ) -> Option<(Self::PackKey, Self::AnimationKey, usize)> {
        if rest_time < 0. {
            log::trace!("default next key: {:?}", (current_pack, current_anim, 0));
            Some((current_pack, current_anim, 0))
        } else {
            None
        }
    }
}
