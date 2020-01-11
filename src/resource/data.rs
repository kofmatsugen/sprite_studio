use super::pack::Pack;
use amethyst::{
    assets::{Asset, Handle},
    ecs::DenseVecStorage,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// アニメーションデータ
// SpriteStudio のプロジェクトファイル一個に相当する
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationData<U> {
    packs: BTreeMap<String, Pack<U>>,
}

impl<U> AnimationData<U> {
    pub fn pack(&self, pack_name: &str) -> Option<&Pack<U>> {
        self.packs.get(pack_name)
    }
}

impl<U> Asset for AnimationData<U>
where
    U: 'static + Serialize + Sync + Send,
{
    const NAME: &'static str = "SPRITE_ANIMATION";

    type Data = Self;
    type HandleStorage = DenseVecStorage<Handle<Self>>;
}

//----------------------------------------------------
// データ参照のみのためビルダーパターン
pub struct AnimationDataBuilder<U> {
    packs: BTreeMap<String, Pack<U>>,
}

impl<U> AnimationDataBuilder<U> {
    pub fn new(packs: BTreeMap<String, Pack<U>>) -> Self {
        AnimationDataBuilder { packs }
    }

    pub fn build(self) -> AnimationData<U> {
        AnimationData { packs: self.packs }
    }
}
