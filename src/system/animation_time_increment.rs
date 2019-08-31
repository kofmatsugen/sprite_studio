use crate::components::AnimationTime;
use amethyst::{
    core::timing::Time,
    ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, SystemData, World},
};

pub struct AnimationTimeIncrementSystem;

impl AnimationTimeIncrementSystem {
    pub fn new() -> Self {
        AnimationTimeIncrementSystem
    }
}

impl<'s> System<'s> for AnimationTimeIncrementSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Read<'s, Time>,
        ReadStorage<'s, AnimationTime>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }

    fn run(&mut self, (entities, lazy, time, animation_times): Self::SystemData) {
        let delta_sec = time.delta_seconds();

        for (e, anim_time) in (&*entities, &animation_times).join() {
            let mut anim_time = anim_time.clone();
            anim_time.add_time(delta_sec);
            lazy.insert(e, anim_time);
        }
    }
}
