use crate::traits::animation_file::AnimationFile;
use amethyst::ecs::{Entity, SystemData};

pub trait TranslateAnimation<'s>: 'static + Send + Sync + AnimationFile {
    type OptionalData: SystemData<'s>;

    // アニメーション遷移
    fn translate_animation(
        _entity: Entity,          // アニメーションを再生してるエンティティ
        rest_time: Option<usize>, // 現在再生中の残り再生フレーム数(Noneの場合は再生終了済み)
        (&current_pack, &current_anim): (&Self::PackKey, &Self::AnimationKey), // 再生中のキー
        _user: Option<&Self::UserData>, // 現在のフレームのユーザーデータ
        _optional: &Self::OptionalData, // 遷移するために必要なシステムデータ
    ) -> Option<(Self::PackKey, Self::AnimationKey, usize)> {
        if rest_time.is_none() {
            log::trace!("default next key: {:?}", (current_pack, current_anim, 0));
            Some((current_pack, current_anim, 0))
        } else {
            None
        }
    }
}
