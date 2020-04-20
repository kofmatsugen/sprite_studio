pub mod animation_file;
pub(crate) mod interpolate;
pub mod translate_animation;
use serde::{Deserialize, Serialize};

// トレイトのエイリアス定義
// 大小比較，ハッシュ化，送信安全性とデバッグ表示が保証されればキーとして使える
pub trait AnimationKey:
    'static
    + Send
    + Sync
    + std::fmt::Debug
    + Ord
    + std::hash::Hash
    + Copy
    + Clone
    + Serialize
    + for<'de> Deserialize<'de>
{
}

impl<T> AnimationKey for T where
    T: 'static
        + Send
        + Sync
        + std::fmt::Debug
        + Ord
        + std::hash::Hash
        + Copy
        + Clone
        + Serialize
        + for<'de> Deserialize<'de>
{
}

// ユーザーデータ，シリアライズ，送信安全性とデバッグ表示が保証されればユーザーデータとして使える
pub trait AnimationUser:
    'static + Send + Sync + Serialize + for<'de> Deserialize<'de> + std::fmt::Debug + Copy
{
}

impl<T> AnimationUser for T where
    T: 'static + Send + Sync + Serialize + std::fmt::Debug + for<'de> Deserialize<'de> + Copy
{
}

pub trait FileId:
    'static + Send + Sync + Ord + std::hash::Hash + std::fmt::Debug + Clone + Copy
{
}

impl<T> FileId for T where
    T: 'static + Send + Sync + Ord + std::hash::Hash + std::fmt::Debug + Clone + Copy
{
}
