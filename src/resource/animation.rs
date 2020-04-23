use super::part_timeline::PartTimeline;
#[cfg(feature = "builder")]
use super::part_timeline::PartTimelineBuilder;
use crate::types::{cell::Cell, InstanceKey, VertexKey};
#[cfg(feature = "builder")]
use crate::types::{interpolate::Interpolation, LinearColor};
use amethyst::{core::Transform, renderer::resources::Tint};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Animation<U> {
    fps: usize,
    total_frame: usize,
    parts_timelines: Vec<PartTimeline<U>>,
}

impl<U> Animation<U> {
    pub fn fps(&self) -> usize {
        self.fps
    }

    pub fn total_frame(&self) -> usize {
        self.total_frame
    }

    pub fn total_secs(&self) -> f32 {
        let float_fps = self.fps() as f32;
        let float_frame = self.total_frame() as f32;
        1.0 / float_fps * float_frame
    }

    pub fn sec_to_frame(&self, seconds: f32) -> usize {
        let float_fps = self.fps() as f32;
        (seconds * float_fps) as usize
    }

    pub fn sec_to_frame_loop(&self, seconds: f32) -> usize {
        let float_fps = self.fps() as f32;
        ((seconds * float_fps) as usize) % self.total_frame()
    }

    pub fn hide(&self, part_id: usize, frame: usize) -> bool {
        log::trace!("[hide] id: {}, frame: {}", part_id, frame);
        self.parts_timelines[part_id].hide(frame)
    }
    pub fn cell(&self, part_id: usize, frame: usize) -> Option<&Cell> {
        log::trace!("[cell] id: {}, frame: {}", part_id, frame);
        self.parts_timelines[part_id].cell(frame)
    }

    pub fn local_transform(&self, part_id: usize, frame: usize) -> Transform {
        log::trace!("[local_transform] id: {}, frame: {}", part_id, frame);
        self.parts_timelines[part_id].local_transform(frame)
    }

    pub fn local_color(&self, part_id: usize, frame: usize) -> Tint {
        log::trace!("[local_color] id: {}, frame: {}", part_id, frame);
        self.parts_timelines[part_id].color(frame)
    }

    pub fn user(&self, part_id: usize, frame: usize) -> Option<&U> {
        log::trace!("[user] id: {}, frame: {}", part_id, frame);
        self.parts_timelines[part_id].user(frame)
    }

    pub fn instance(&self, part_id: usize, frame: usize) -> Option<(usize, &InstanceKey)> {
        log::trace!("[instance] id: {}, frame: {}", part_id, frame);
        self.parts_timelines[part_id].instance(frame)
    }

    pub fn vertex(&self, part_id: usize, frame: usize) -> Option<VertexKey> {
        self.parts_timelines[part_id].vertex(frame)
    }
}

#[cfg(feature = "builder")]
pub struct AnimationBuilder<U> {
    fps: usize,
    total_frame: usize,
    parts_timelines: Vec<PartTimelineBuilder<U>>,
}

#[cfg(feature = "builder")]
impl<U> AnimationBuilder<U> {
    pub fn new(part_num: usize, total_frame: usize, fps: usize) -> Self {
        AnimationBuilder {
            fps,
            total_frame,
            parts_timelines: (0..part_num).map(|_| PartTimelineBuilder::new()).collect(),
        }
    }
    pub fn add_hide(
        &mut self,
        part_id: usize,
        frame: usize,
        interpolation: Interpolation,
        visible: bool,
    ) {
        self.parts_timelines[part_id].add_hide(frame, interpolation, visible);
    }

    pub fn add_cell(
        &mut self,
        part_id: usize,
        frame: usize,
        interpolation: Interpolation,
        cell: Cell,
    ) {
        self.parts_timelines[part_id].add_cell(frame, interpolation, cell);
    }

    pub fn add_pos_x(
        &mut self,
        part_id: usize,
        frame: usize,
        interpolation: Interpolation,
        pos_x: f32,
    ) {
        self.parts_timelines[part_id].add_pos_x(frame, interpolation, pos_x);
    }

    pub fn add_pos_y(
        &mut self,
        part_id: usize,
        frame: usize,
        interpolation: Interpolation,
        pos_y: f32,
    ) {
        self.parts_timelines[part_id].add_pos_y(frame, interpolation, pos_y);
    }

    pub fn add_pos_z(
        &mut self,
        part_id: usize,
        frame: usize,
        interpolation: Interpolation,
        pos_z: f32,
    ) {
        self.parts_timelines[part_id].add_pos_z(frame, interpolation, pos_z);
    }

    pub fn add_scale_x(
        &mut self,
        part_id: usize,
        frame: usize,
        interpolation: Interpolation,
        scale_x: f32,
    ) {
        self.parts_timelines[part_id].add_scale_x(frame, interpolation, scale_x);
    }
    pub fn add_scale_y(
        &mut self,
        part_id: usize,
        frame: usize,
        interpolation: Interpolation,
        scale_y: f32,
    ) {
        self.parts_timelines[part_id].add_scale_y(frame, interpolation, scale_y);
    }
    pub fn add_rotated(
        &mut self,
        part_id: usize,
        frame: usize,
        interpolation: Interpolation,
        rotated: f32,
    ) {
        self.parts_timelines[part_id].add_rotated(frame, interpolation, rotated);
    }
    // 反転情報
    pub fn add_flip_v(
        &mut self,
        part_id: usize,
        frame: usize,
        interpolation: Interpolation,
        flip_v: bool,
    ) {
        self.parts_timelines[part_id].add_flip_v(frame, interpolation, flip_v);
    }
    pub fn add_flip_h(
        &mut self,
        part_id: usize,
        frame: usize,
        interpolation: Interpolation,
        flip_h: bool,
    ) {
        self.parts_timelines[part_id].add_flip_h(frame, interpolation, flip_h);
    }
    // 色情報
    pub fn add_alpha(
        &mut self,
        part_id: usize,
        frame: usize,
        interpolation: Interpolation,
        alpha: f32,
    ) {
        self.parts_timelines[part_id].add_alpha(frame, interpolation, alpha);
    }
    pub fn add_color(
        &mut self,
        part_id: usize,
        frame: usize,
        interpolation: Interpolation,
        color: LinearColor,
    ) {
        self.parts_timelines[part_id].add_color(frame, interpolation, color);
    }

    // カスタムデータ
    pub fn add_user(
        &mut self,
        part_id: usize,
        frame: usize,
        interpolation: Interpolation,
        user: U,
    ) {
        self.parts_timelines[part_id].add_user(frame, interpolation, user);
    }

    // アニメーションインスタンス
    pub fn add_instance(
        &mut self,
        part_id: usize,
        frame: usize,
        interpolation: Interpolation,
        instance: InstanceKey,
    ) {
        self.parts_timelines[part_id].add_instance(frame, interpolation, instance);
    }

    pub fn add_vertex(
        &mut self,
        part_id: usize,
        frame: usize,
        interpolation: Interpolation,
        vertex: VertexKey,
    ) {
        self.parts_timelines[part_id].add_vertex(frame, interpolation, vertex);
    }

    pub fn build(self) -> Animation<U> {
        Animation {
            fps: self.fps,
            total_frame: self.total_frame,
            parts_timelines: self
                .parts_timelines
                .into_iter()
                .map(|builder| builder.build())
                .collect(),
        }
    }
}
