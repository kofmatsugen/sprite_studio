use crate::{components::AnimationNodes, traits::animation_file::AnimationFile};
use amethyst::{
    core::Time,
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
};
use movement_transform::components::Movement;
use std::marker::PhantomData;

// アニメーションのキーを変更したときにイベントを発行する
pub struct RootTranslateSystem<T>
where
    T: 'static + Send + Sync,
{
    _translation: PhantomData<T>,
}

impl<T> RootTranslateSystem<T>
where
    T: 'static + Send + Sync,
{
    pub fn new() -> Self {
        RootTranslateSystem {
            _translation: PhantomData,
        }
    }
}

impl<'s, T> System<'s> for RootTranslateSystem<T>
where
    T: AnimationFile,
{
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, AnimationNodes<T::UserData>>,
        WriteStorage<'s, Movement>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, nodes, mut movements, time): Self::SystemData) {
        for (e, node) in (&*entities, &nodes).join() {
            let (x, y) = node.root_translate();
            if x == 0. && y == 0. {
                continue;
            }

            if let Some(movement) = movements.get_mut(e) {
                log::info!(
                    "[{} F] root translate: ({:.2}, {:.2})",
                    time.frame_number(),
                    x,
                    y
                );

                movement.transform_mut().translation_mut().x += x;
                movement.transform_mut().translation_mut().y += y;
            }
        }
    }
}
