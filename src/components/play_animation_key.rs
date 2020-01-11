use amethyst::ecs::{Component, FlaggedStorage};

pub struct PlayAnimationKey<ID, P, A> {
    file_id: ID,
    pack_name: Option<P>,
    animation_name: Option<A>,
}

impl<ID, P, A> PlayAnimationKey<ID, P, A> {
    pub fn new(file_id: ID) -> Self {
        PlayAnimationKey {
            file_id,
            pack_name: None,
            animation_name: None,
        }
    }

    pub fn set_pack(&mut self, pack_name: P) {
        self.pack_name = Some(pack_name);
    }

    pub fn set_animation(&mut self, animation_name: A) {
        self.animation_name = Some(animation_name);
    }

    pub fn file_id(&self) -> &ID {
        &self.file_id
    }

    pub fn pack_name(&self) -> Option<&P> {
        self.pack_name.as_ref()
    }

    pub fn animation_name(&self) -> Option<&A> {
        self.animation_name.as_ref()
    }
}

impl<ID, P, A> Component for PlayAnimationKey<ID, P, A>
where
    ID: 'static + Send + Sync,
    P: 'static + Send + Sync,
    A: 'static + Send + Sync,
{
    type Storage = FlaggedStorage<Self>;
}
