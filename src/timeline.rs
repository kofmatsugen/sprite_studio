mod bound_type;
mod from_user;
mod key_frame;
mod linear_color;

pub use from_user::FromUser;
pub use linear_color::LinearColor;

use amethyst::{
    assets::{Asset, Handle},
    core::Transform,
    ecs::DenseVecStorage,
    renderer::resources::Tint,
};
use from_user::NonDecodedUser;
use itertools::izip;
use key_frame::{KeyFrame, KeyFrameBuilder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SpriteAnimation<U>
where
    U: FromUser + Serialize,
{
    fps: u32,
    total_frame: usize,
    #[serde(bound(
        serialize = "Vec<TimeLine<U>>: Serialize",
        deserialize = "Vec<TimeLine<U>>: Deserialize<'de>"
    ))]
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

    pub fn timelines(&self) -> impl Iterator<Item = &TimeLine<U>> {
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

#[derive(Serialize, Deserialize)]
pub struct TimeLine<U>
where
    U: FromUser + Serialize,
{
    id: usize,
    parent: Option<usize>,
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
    pub fn part_id(&self) -> usize {
        self.id
    }

    pub fn parent_id(&self) -> Option<usize> {
        self.parent
    }

    pub fn key_frame(&self, frame_no: usize) -> &KeyFrame<U> {
        &self.key_frames[frame_no]
    }

    pub fn users(&self) -> impl Iterator<Item = Option<&U>> {
        self.key_frames.iter().map(|k| k.user())
    }

    pub fn transforms(&self) -> impl Iterator<Item = &Transform> {
        self.key_frames.iter().map(|k| k.transform())
    }

    pub fn visibles<'a>(&'a self) -> impl 'a + Iterator<Item = bool> {
        self.key_frames.iter().map(|k| k.visible())
    }

    pub fn cells<'a>(&'a self) -> impl 'a + Iterator<Item = Option<(usize, usize)>> {
        self.key_frames.iter().map(|k| k.cell())
    }

    pub fn colors(&self) -> impl Iterator<Item = &Tint> {
        self.key_frames.iter().map(|k| k.color())
    }
}

// TimeLine生成用
pub struct TimeLineBuilder {
    frame_count: usize,
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
}

impl TimeLineBuilder {
    pub fn new(frame_count: usize) -> Self {
        TimeLineBuilder {
            frame_count,
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

    pub fn build<U>(mut self, id: usize, parent: impl Into<Option<usize>>) -> TimeLine<U>
    where
        U: FromUser + Serialize,
    {
        let mut timeline = TimeLine {
            key_frames: Vec::with_capacity(self.frame_count),
            id: id,
            parent: parent.into(),
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
        );

        let mut transform = Transform::default();

        for (u, x, y, z, scale_x, scale_y, rotated, visible, cell, color) in frames {
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

            let key_frame = KeyFrameBuilder::new()
                .user(u)
                .transform(transform)
                .visible(visible)
                .cell(cell)
                .color(color.into())
                .build();

            timeline.key_frames.push(key_frame);
        }
        timeline
    }
}
