use crate::types::{
    animation_instance::InstanceKeyBuilder,
    bound_type::Bounds,
    from_user::FromUser,
    from_user::NonDecodedUser,
    key_frame::{KeyFrame, KeyFrameBuilder},
    linear_color::LinearColor,
    part_info::{PartInfo, PartInfoBuilder},
    part_type::PartType,
};
use amethyst::core::Transform;
use itertools::izip;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TimeLine<U>
where
    U: FromUser + Serialize,
{
    part_info: PartInfo,
    #[serde(bound(
        serialize = "Vec<KeyFrame<U>>: Serialize",
        deserialize = "Vec<KeyFrame<U>>: Deserialize<'de>"
    ))]
    key_frames: Vec<KeyFrame<U>>,
}

impl<U> TimeLine<U>
where
    U: FromUser + Serialize,
{
    pub fn part_info(&self) -> &PartInfo {
        &self.part_info
    }

    pub fn key_frame(&self, frame_no: usize) -> &KeyFrame<U> {
        &self.key_frames[frame_no]
    }
}

// TimeLine生成用
pub struct TimeLineBuilder {
    frame_count: usize,
    part_info: PartInfoBuilder,
    users: Vec<Option<NonDecodedUser>>,
    pos_x: Vec<f32>,
    pos_y: Vec<f32>,
    pos_z: Vec<f32>,
    scale_x: Vec<f32>,
    scale_y: Vec<f32>,
    rotated: Vec<f32>,
    visible: Vec<bool>,
    cell: Vec<Option<(usize, usize)>>,
    color: Vec<LinearColor>,
    alpha: Vec<f32>,
    instance: Vec<Option<InstanceKeyBuilder>>,
}

impl TimeLineBuilder {
    pub fn new(frame_count: usize) -> Self {
        TimeLineBuilder {
            frame_count,
            part_info: PartInfoBuilder::default(),
            users: Vec::with_capacity(frame_count),
            pos_x: Vec::with_capacity(frame_count),
            pos_y: Vec::with_capacity(frame_count),
            pos_z: Vec::with_capacity(frame_count),
            scale_x: Vec::with_capacity(frame_count),
            scale_y: Vec::with_capacity(frame_count),
            rotated: Vec::with_capacity(frame_count),
            visible: Vec::with_capacity(frame_count),
            cell: Vec::with_capacity(frame_count),
            color: Vec::with_capacity(frame_count),
            alpha: Vec::with_capacity(frame_count),
            instance: Vec::with_capacity(frame_count),
        }
    }

    pub fn add_user(
        &mut self,
        integer: Option<i32>,
        point: Option<(f32, f32)>,
        rect: Option<(f32, f32, f32, f32)>,
        text: Option<String>,
    ) {
        if self.users.len() >= self.frame_count {
            panic!(
                "over limit {} users: {}",
                self.frame_count,
                self.users.len(),
            );
        }
        let user = match (integer, point, rect, text) {
            (None, None, None, None) => None,
            (integer, point, rect, text) => NonDecodedUser {
                integer,
                point,
                rect,
                text,
            }
            .into(),
        };
        self.users.push(user);
    }

    pub fn add_pos_x(&mut self, x: f32) {
        if self.pos_x.len() >= self.frame_count {
            panic!(
                "over limit {} pos x: {}",
                self.frame_count,
                self.pos_x.len(),
            );
        }
        self.pos_x.push(x.into());
    }

    pub fn add_pos_y(&mut self, y: f32) {
        if self.pos_y.len() >= self.frame_count {
            panic!(
                "over limit {} pos y: {}",
                self.frame_count,
                self.pos_y.len(),
            );
        }
        self.pos_y.push(y.into());
    }

    pub fn add_pos_z(&mut self, z: f32) {
        if self.pos_z.len() >= self.frame_count {
            panic!(
                "over limit {} pos z: {}",
                self.frame_count,
                self.pos_z.len(),
            );
        }
        self.pos_z.push(z.into());
    }

    pub fn add_scale_x(&mut self, x: f32) {
        if self.scale_x.len() >= self.frame_count {
            panic!(
                "over limit {} scale x: {}",
                self.frame_count,
                self.scale_x.len(),
            );
        }
        self.scale_x.push(x.into());
    }

    pub fn add_scale_y(&mut self, y: f32) {
        if self.scale_y.len() >= self.frame_count {
            panic!(
                "over limit {} scale y: {}",
                self.frame_count,
                self.scale_y.len(),
            );
        }
        self.scale_y.push(y.into());
    }

    pub fn add_rotated(&mut self, rotate: f32) {
        if self.rotated.len() >= self.frame_count {
            panic!(
                "over limit {} rotated: {}",
                self.frame_count,
                self.rotated.len(),
            );
        }
        self.rotated.push(rotate.into());
    }

    pub fn add_visible(&mut self, visible: bool) {
        if self.visible.len() >= self.frame_count {
            panic!(
                "over limit {} visible: {}",
                self.frame_count,
                self.visible.len(),
            );
        }
        self.visible.push(visible.into());
    }

    pub fn add_cell(&mut self, cell: Option<(usize, usize)>) {
        if self.cell.len() >= self.frame_count {
            panic!("over limit {} cell: {}", self.frame_count, self.cell.len(),);
        }
        self.cell.push(cell);
    }

    pub fn add_color(&mut self, color: LinearColor) {
        if self.color.len() >= self.frame_count {
            panic!(
                "over limit {} color: {}",
                self.frame_count,
                self.color.len(),
            );
        }
        self.color.push(color.into());
    }

    pub fn add_alpha(&mut self, alpha: f32) {
        if self.alpha.len() >= self.frame_count {
            panic!(
                "over limit {} alpha: {}",
                self.frame_count,
                self.alpha.len(),
            );
        }
        self.alpha.push(alpha);
    }

    pub fn add_instance(&mut self, instance: Option<InstanceKeyBuilder>) {
        if self.instance.len() >= self.frame_count {
            panic!(
                "over limit {} instance: {}",
                self.frame_count,
                self.instance.len(),
            );
        }
        self.instance.push(instance);
    }

    pub fn part_id(mut self, id: usize) -> Self {
        self.part_info.id(id);
        self
    }

    pub fn parent_id(mut self, parent_id: impl Into<Option<usize>>) -> Self {
        self.part_info.parent(parent_id);
        self
    }

    pub fn part_type(mut self, part_type: PartType) -> Self {
        self.part_info.part_type(part_type);
        self
    }

    pub fn bounds(mut self, bounds: impl Into<Option<Bounds>>) -> Self {
        self.part_info.bounds(bounds);
        self
    }

    pub fn ref_pack_id(mut self, pack_id: impl Into<Option<usize>>) -> Self {
        self.part_info.pack_id(pack_id);
        self
    }

    pub fn ref_anim_id(mut self, anim_id: impl Into<Option<usize>>) -> Self {
        self.part_info.animation_id(anim_id);
        self
    }

    pub fn build<U>(mut self) -> TimeLine<U>
    where
        U: FromUser + Serialize,
    {
        let part_info = self.part_info.build();
        let mut timeline = TimeLine {
            part_info,
            key_frames: Vec::with_capacity(self.frame_count),
        };

        // フレームカウントに満たない場合はNoneで埋める
        for _ in 0..(self.frame_count - self.pos_x.len()) {
            self.pos_x.push(self.pos_x.last().map(|v| *v).unwrap_or(0.));
        }
        for _ in 0..(self.frame_count - self.pos_y.len()) {
            self.pos_y.push(self.pos_y.last().map(|v| *v).unwrap_or(0.));
        }
        for _ in 0..(self.frame_count - self.pos_z.len()) {
            self.pos_z.push(self.pos_z.last().map(|v| *v).unwrap_or(0.));
        }
        for _ in 0..(self.frame_count - self.scale_x.len()) {
            self.scale_x
                .push(self.scale_x.last().map(|v| *v).unwrap_or(1.));
        }
        for _ in 0..(self.frame_count - self.scale_y.len()) {
            self.scale_y
                .push(self.scale_y.last().map(|v| *v).unwrap_or(1.));
        }
        for _ in 0..(self.frame_count - self.rotated.len()) {
            self.rotated
                .push(self.rotated.last().map(|v| *v).unwrap_or(0.));
        }
        for _ in 0..(self.frame_count - self.users.len()) {
            self.users.push(None);
        }
        for _ in 0..(self.frame_count - self.visible.len()) {
            self.visible
                .push(self.visible.last().map(|v| *v).unwrap_or(true));
        }
        for _ in 0..(self.frame_count - self.cell.len()) {
            self.cell.push(self.cell.last().and_then(|v| *v));
        }
        for _ in 0..(self.frame_count - self.color.len()) {
            self.color.push(
                self.color
                    .last()
                    .map(|v| v.clone())
                    .unwrap_or(Default::default()),
            );
        }
        for _ in 0..(self.frame_count - self.instance.len()) {
            let next_last = self
                .instance
                .last()
                .and_then(|i| i.as_ref())
                .and_then(InstanceKeyBuilder::next_key);
            self.instance.push(next_last);
        }
        for _ in 0..(self.frame_count - self.alpha.len()) {
            self.alpha
                .push(self.alpha.last().map(|v| *v).unwrap_or(1.0));
        }

        // 全部同じサイズになってるのでこれでタイムラインを構成
        let frames = izip!(
            self.users.into_iter(),
            self.pos_x.into_iter(),
            self.pos_y.into_iter(),
            self.pos_z.into_iter(),
            self.scale_x.into_iter(),
            self.scale_y.into_iter(),
            self.rotated.into_iter(),
            self.visible.into_iter(),
            self.cell.into_iter(),
            self.color.into_iter(),
            self.instance.into_iter(),
            self.alpha.into_iter(),
        );

        let mut transform = Transform::default();
        let mut linear_color = LinearColor(1.0, 1.0, 1.0, 1.0);

        for (
            _idx,
            (u, x, y, z, scale_x, scale_y, rotated, visible, cell, color, instance, alpha),
        ) in frames.enumerate()
        {
            // transform は，直前のものを利用しつつ何らか値が入ったら変動値として扱う
            let transform = {
                transform.set_translation_x(x);
                transform.set_translation_y(y);
                transform.set_translation_z(-z);
                transform.set_rotation_2d(rotated);
                let mut scale = transform.scale().clone();
                scale.x = scale_x;
                scale.y = scale_y;
                transform.set_scale(scale);
                transform.clone()
            };

            linear_color.0 = color.0;
            linear_color.1 = color.1;
            linear_color.2 = color.2;
            linear_color.3 = alpha;

            let key_frame = KeyFrameBuilder::new()
                .user(u)
                .transform(transform)
                .visible(visible)
                .cell(cell)
                .color(linear_color.into())
                .instance_key(instance)
                .build();

            timeline.key_frames.push(key_frame);
        }
        timeline
    }
}
