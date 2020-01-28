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
pub struct AnimationData<U, P = String, A = String>
where
    P: Ord + std::hash::Hash + Serialize,
    A: Ord + std::hash::Hash + Serialize,
{
    #[serde(bound(deserialize = "BTreeMap<P, Pack<U, A>>: Deserialize<'de>"))]
    packs: BTreeMap<P, Pack<U, A>>,
}

impl<U, P, A> AnimationData<U, P, A>
where
    P: Ord + std::hash::Hash + Serialize,
    A: Ord + std::hash::Hash + Serialize,
{
    pub fn pack(&self, pack: &P) -> Option<&Pack<U, A>> {
        self.packs.get(pack)
    }
}

impl<U, P, A> Asset for AnimationData<U, P, A>
where
    U: 'static + Serialize + Sync + Send,
    P: 'static + Sync + Send + Ord + std::hash::Hash + Serialize,
    A: 'static + Sync + Send + Ord + std::hash::Hash + Serialize,
{
    const NAME: &'static str = "SPRITE_ANIMATION";

    type Data = Self;
    type HandleStorage = DenseVecStorage<Handle<Self>>;
}

//----------------------------------------------------
// データ参照のみのためビルダーパターン
pub struct AnimationDataBuilder<U, P, A>
where
    P: Ord + std::hash::Hash + Serialize,
    A: Ord + std::hash::Hash + Serialize,
{
    packs: BTreeMap<P, Pack<U, A>>,
}

impl<U, P, A> AnimationDataBuilder<U, P, A>
where
    P: Ord + std::hash::Hash + Serialize,
    A: Ord + std::hash::Hash + Serialize,
{
    pub fn new(packs: BTreeMap<P, Pack<U, A>>) -> Self {
        AnimationDataBuilder { packs }
    }

    pub fn build(self) -> AnimationData<U, P, A> {
        AnimationData { packs: self.packs }
    }
}
