use crate::{
    resource::animation_store::AnimationData,
    traits::FromUser,
    types::{key_frame::KeyFrame, part_info::PartInfo},
};

pub struct Node<'a, U>
where
    U: 'static + serde::Serialize + Send + Sync + std::fmt::Debug + FromUser,
{
    pub pack_id: usize,
    pub anim_id: usize,
    pub part_info: &'a PartInfo,
    pub key_frame: &'a KeyFrame<U>,
    pub animation: &'a AnimationData<U>,
}
