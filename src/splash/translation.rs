use super::id::{AnimationKey, FileId, PackKey};
use crate::traits::{animation_file::AnimationFile, translate_animation::TranslateAnimation};
use amethyst::ecs::Entity;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
// スプラッシュ画像の遷移
// アニメーション終了のイベントを汎用で作るか，オプションで作れるようにするか考える

#[derive(Debug)]
pub struct SplashTranslation;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DummyUser;

impl AnimationFile for SplashTranslation {
    type FileId = FileId;
    type PackKey = PackKey;
    type AnimationKey = AnimationKey;
    type UserData = DummyUser;

    fn to_file_name(file_id: &Self::FileId) -> &'static str {
        FILE_LIST[file_id].0
    }

    fn sprite_sheet_num(file_id: &Self::FileId) -> usize {
        FILE_LIST[file_id].1
    }
}

impl<'s> TranslateAnimation<'s> for SplashTranslation {
    type OptionalData = ();

    fn translate_animation(
        _: Entity,
        _: f32,
        _: (&Self::PackKey, &Self::AnimationKey),
        _: Option<&Self::UserData>,
        _: &Self::OptionalData,
    ) -> Option<(Self::PackKey, Self::AnimationKey, usize)> {
        None
    }
}

lazy_static::lazy_static! {
    static ref FILE_LIST: BTreeMap<FileId, (&'static str, usize)> = {
        let mut list = BTreeMap::new();
        list.insert(FileId::SpriteStudioSplash, ("splash1024", 1));
        list
    };
}
