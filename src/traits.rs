pub mod animation_file;
pub(crate) mod interpolate;
pub mod translate_animation;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

// トレイトのエイリアス定義
// 大小比較，ハッシュ化，送信安全性とデバッグ表示が保証されればキーとして使える
pub trait AnimationKey:
    'static + Send + Sync + Debug + Ord + Hash + Copy + Serialize + for<'de> Deserialize<'de>
{
}

impl<T> AnimationKey for T where
    T: 'static + Send + Sync + Debug + Ord + Hash + Copy + Serialize + for<'de> Deserialize<'de>
{
}

// ユーザーデータ，シリアライズ，送信安全性とデバッグ表示が保証されればユーザーデータとして使える
pub trait AnimationUser:
    'static + Send + Sync + Serialize + for<'de> Deserialize<'de> + Debug + Copy
{
}

impl<T> AnimationUser for T where
    T: 'static + Send + Sync + Serialize + Debug + for<'de> Deserialize<'de> + Copy
{
}

pub trait FileId: 'static + Send + Sync + Ord + Hash + Debug + Copy {}

impl<T> FileId for T where T: 'static + Send + Sync + Ord + std::hash::Hash + std::fmt::Debug + Copy {}
