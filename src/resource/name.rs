use crate::traits::AnimationKey;
use serde::{Deserialize, Serialize};

//----------------------------------------------------
// アニメーション情報の取得キー
#[derive(Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = "Option<P>: Deserialize<'de>"))]
pub enum AnimationName<P, A>
where
    P: AnimationKey,
    A: AnimationKey,
{
    FullName {
        pack: P,
        animation: A,
    },
    PackName(P),
    AnimName(A),
    WithPartId {
        pack: String,
        animation: A,
        part_id: u32,
    },
}
