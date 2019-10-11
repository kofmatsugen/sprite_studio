use crate::{traits::AnimationUser, SpriteAnimation};
use std::ops::Range;

pub fn convert_time_to_frame<'a, U>(animation_time: f32, data: &'a SpriteAnimation<U>) -> usize
where
    U: AnimationUser,
{
    let fps = data.fps();
    let current = (animation_time * (fps as f32)).floor() as usize;
    let current = current % data.total_frame();

    current
}

pub fn convert_time_to_frame_range<'a, U>(
    animation_start: f32,
    animation_end: f32,
    data: &'a SpriteAnimation<U>,
) -> Range<usize>
where
    U: AnimationUser,
{
    let fps = data.fps();
    let animation_start = (animation_start * (fps as f32)).floor() as usize;
    let animation_start = animation_start % data.total_frame();
    let animation_end = (animation_end * (fps as f32)).floor() as usize;
    let animation_end = animation_end % data.total_frame();

    animation_start..animation_end + 1
}
