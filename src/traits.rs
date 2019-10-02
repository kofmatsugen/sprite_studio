pub(crate) mod collision_color;
pub(crate) mod from_user;

pub use collision_color::CollisionColor;
pub use from_user::FromUser;

use serde::Serialize;

// トレイトのエイリアス定義
// 大小比較，ハッシュ化，送信安全性とデバッグ表示が保証されればキーとして使える
pub trait AnimationKey
where
    Self: 'static + Send + Sync + std::hash::Hash + PartialOrd + Ord + std::fmt::Debug,
{
}

impl<T> AnimationKey for T where
    T: 'static + Send + Sync + std::hash::Hash + PartialOrd + Ord + std::fmt::Debug
{
}

// ユーザーデータ，シリアライズ，送信安全性とデバッグ表示が保証されればユーザーデータとして使える
pub trait AnimationUser
where
    Self: 'static + Send + Sync + FromUser + Serialize + std::fmt::Debug,
{
}

impl<T> AnimationUser for T where T: 'static + Send + Sync + FromUser + Serialize + std::fmt::Debug {}
