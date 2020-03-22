use crate::traits::animation_file::AnimationFile;
use amethyst::ecs::{Component, FlaggedStorage};

pub struct PlayAnimationKey<T>
where
    T: AnimationFile,
{
    file_id: T::FileId,
    pack_name: Option<T::PackKey>,
    animation_name: Option<T::AnimationKey>,
}

impl<T> PlayAnimationKey<T>
where
    T: AnimationFile,
{
    pub fn new(file_id: T::FileId) -> Self {
        PlayAnimationKey {
            file_id,
            pack_name: None,
            animation_name: None,
        }
    }

    pub fn set_pack(&mut self, pack_name: T::PackKey) {
        self.pack_name = Some(pack_name);
    }

    pub fn set_animation(&mut self, animation_name: T::AnimationKey) {
        self.animation_name = Some(animation_name);
    }

    pub fn file_id(&self) -> &T::FileId {
        &self.file_id
    }

    pub fn pack_name(&self) -> Option<&T::PackKey> {
        self.pack_name.as_ref()
    }

    pub fn animation_name(&self) -> Option<&T::AnimationKey> {
        self.animation_name.as_ref()
    }
}

impl<T> Component for PlayAnimationKey<T>
where
    T: AnimationFile,
{
    type Storage = FlaggedStorage<Self>;
}
