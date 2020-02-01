pub mod animation_file;
pub(crate) mod interpolate;
pub mod translate_animation;
use serde::{de::DeserializeOwned, Serialize};

// トレイトのエイリアス定義
// 大小比較，ハッシュ化，送信安全性とデバッグ表示が保証されればキーとして使える
pub trait AnimationKey
where
    Self: 'static
        + Send
        + Sync
        + std::fmt::Debug
        + Ord
        + std::hash::Hash
        + Copy
        + Clone
        + Serialize
        + DeserializeOwned,
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
        + DeserializeOwned
{
}

// ユーザーデータ，シリアライズ，送信安全性とデバッグ表示が保証されればユーザーデータとして使える
pub trait AnimationUser
where
    Self: 'static + Send + Sync + Serialize + DeserializeOwned + std::fmt::Debug,
{
}

impl<T> AnimationUser for T where
    T: 'static + Send + Sync + Serialize + std::fmt::Debug + DeserializeOwned
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
