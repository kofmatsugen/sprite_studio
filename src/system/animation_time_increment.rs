use crate::components::AnimationTime;
use amethyst::{
    core::timing::Time,
    ecs::{Join, Read, System, SystemData, World, WriteStorage},
};

pub struct AnimationTimeIncrementSystem;

impl AnimationTimeIncrementSystem {
    pub fn new() -> Self {
        AnimationTimeIncrementSystem
    }
}

impl<'s> System<'s> for AnimationTimeIncrementSystem {
    type SystemData = (Read<'s, Time>, WriteStorage<'s, AnimationTime>);

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }

    fn run(&mut self, (time, mut animation_times): Self::SystemData) {
        #[cfg(not(feature = "count_frame"))]
        let delta_sec = time.delta_seconds();
        #[cfg(feature = "count_frame")]
        let delta_sec = 60.;
        for (anim_time,) in (&mut animation_times,).join() {
            anim_time.add_time(delta_sec); // 現実時間で再生
        }
    }
}
