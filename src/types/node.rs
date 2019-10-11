use crate::{
    resource::animation_store::AnimationData,
    types::{key_frame::KeyFrame, part_info::PartInfo},
};

pub type Node<'a, U> = (u64, &'a PartInfo, &'a KeyFrame<U>, &'a AnimationData<U>);
