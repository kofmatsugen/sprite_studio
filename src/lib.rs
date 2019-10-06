pub mod components;
pub mod iter;
pub mod renderer;
pub mod resource;
pub(crate) mod shaders;
pub mod system;
pub mod timeline;
pub mod traits;
pub mod types;
pub mod utils;

use amethyst::{
    assets::{Asset, Handle},
    ecs::DenseVecStorage,
};
use serde::*;
use timeline::TimeLine;
use traits::from_user::FromUser;

#[derive(Serialize, Deserialize)]
pub struct SpriteAnimation<U>
where
    U: FromUser + Serialize,
{
    fps: u32,
    total_frame: usize,
    #[serde(
        bound(
            serialize = "Vec<TimeLine<U>>: Serialize",
            deserialize = "Vec<TimeLine<U>>: Deserialize<'de>"
        ),
        skip_serializing_if = "Vec::is_empty"
    )]
    timelines: Vec<TimeLine<U>>,
}

pub type SpriteAnimationHandle<U> = Handle<SpriteAnimation<U>>;

impl<U> SpriteAnimation<U>
where
    U: FromUser + Serialize,
{
    pub fn new(fps: u32, total_frame: usize) -> Self {
        SpriteAnimation {
            fps,
            total_frame,
            timelines: vec![],
        }
    }

    pub fn add_timeline(&mut self, timeline: TimeLine<U>) {
        self.timelines.push(timeline);
    }

    pub fn fps(&self) -> u32 {
        self.fps
    }

    pub fn timelines(&self) -> impl DoubleEndedIterator<Item = &TimeLine<U>> {
        self.timelines.iter()
    }

    pub fn total_frame(&self) -> usize {
        self.total_frame
    }
}

impl<U> Asset for SpriteAnimation<U>
where
    U: 'static + FromUser + Serialize + Sync + Send,
{
    const NAME: &'static str = "SPRITE_ANIMATION";

    type Data = Self;
    type HandleStorage = DenseVecStorage<Handle<Self>>;
}
