use crate::{resource::pack::Pack, traits::animation_file::AnimationFile};
use amethyst::{
    assets::{Asset, Handle},
    ecs::DenseVecStorage,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// アニメーションデータ
// SpriteStudio のプロジェクトファイル一個に相当する
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationData<T>
where
    T: AnimationFile,
{
    #[serde(bound(
        deserialize = "BTreeMap<T::PackKey, Pack<T::UserData,T::PackKey, T::AnimationKey>>: Deserialize<'de>"
    ))]
    packs: BTreeMap<T::PackKey, Pack<T::UserData, T::PackKey, T::AnimationKey>>,
}

impl<T> AnimationData<T>
where
    T: AnimationFile,
{
    pub fn pack(
        &self,
        pack: &T::PackKey,
    ) -> Option<&Pack<T::UserData, T::PackKey, T::AnimationKey>> {
        self.packs.get(pack)
    }
}

impl<T> Asset for AnimationData<T>
where
    T: 'static + Send + Sync + AnimationFile,
{
    const NAME: &'static str = "SPRITE_ANIMATION";

    type Data = Self;
    type HandleStorage = DenseVecStorage<Handle<Self>>;
}

#[cfg(feature = "debug")]
impl<T> std::ops::Drop for AnimationData<T>
where
    T: AnimationFile,
{
    fn drop(&mut self) {
        log::debug!(
            "drop AnimationData: {:?}",
            self.packs.keys().collect::<Vec<_>>()
        );
    }
}

//----------------------------------------------------
// データ参照のみのためビルダーパターン
#[cfg(feature = "builder")]
pub struct AnimationDataBuilder<T>
where
    T: AnimationFile,
{
    packs: BTreeMap<T::PackKey, Pack<T::UserData, T::PackKey, T::AnimationKey>>,
}

#[cfg(feature = "builder")]
impl<T> AnimationDataBuilder<T>
where
    T: AnimationFile,
{
    pub fn new(
        packs: BTreeMap<T::PackKey, Pack<T::UserData, T::PackKey, T::AnimationKey>>,
    ) -> Self {
        AnimationDataBuilder { packs }
    }

    pub fn build(self) -> AnimationData<T> {
        AnimationData { packs: self.packs }
    }
}
