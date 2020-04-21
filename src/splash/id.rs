#[cfg(feature = "serialize")]
use failure::Fail;
use serde::{Deserialize, Serialize};
#[cfg(feature = "serialize")]
use std::str::FromStr;

// アニメーションID一式
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
// ファイルID
pub enum FileId {
    SpriteStudioSplash,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PackKey {
    SpriteStudioSplashInstance,
    SpriteStudioSplash,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AnimationKey {
    SplashIn,
    SplashOut,
    SplashInOut,
}

// データ化用のシリアライズ関数
#[cfg(feature = "serialize")]
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "unknown pack name: {}", _0)]
    UnknownPackName(String),
    #[fail(display = "unknown animation name: {}", _0)]
    UnknownAnimationName(String),
}

#[cfg(feature = "serialize")]
impl FromStr for PackKey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "splash1024_instance" => Ok(PackKey::SpriteStudioSplashInstance),
            "splash1024" => Ok(PackKey::SpriteStudioSplash),
            _ => Err(Error::UnknownPackName(s.into())),
        }
    }
}

#[cfg(feature = "serialize")]
impl FromStr for AnimationKey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "in" => Ok(AnimationKey::SplashIn),
            "out" => Ok(AnimationKey::SplashOut),
            "inout" => Ok(AnimationKey::SplashInOut),
            _ => Err(Error::UnknownAnimationName(s.into())),
        }
    }
}
