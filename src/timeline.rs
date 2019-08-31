mod from_user;
mod key_frame;

pub use from_user::FromUser;

use amethyst::{
    assets::{Asset, Handle},
    core::Transform,
    ecs::DenseVecStorage,
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

    pub fn users(&self) -> impl Iterator<Item = Option<&U>> {
        self.key_frames.iter().map(|k| k.user())
    }

    pub fn transforms(&self) -> impl Iterator<Item = Option<&Transform>> {
        self.key_frames.iter().map(|k| k.transform())
    }

    pub fn visibles<'a>(&'a self) -> impl 'a + Iterator<Item = Option<bool>> {
        self.key_frames.iter().map(|k| k.visible())
    }

    pub fn cells<'a>(&'a self) -> impl 'a + Iterator<Item = Option<(usize, usize)>> {
        self.key_frames.iter().map(|k| k.cell())
    }
}

// TimeLine生成用
pub struct TimeLineBuilder {
    frame_count: usize,
    users: Vec<Option<NonDecodedUser>>,
    pos_x: Vec<Option<f32>>,
    pos_y: Vec<Option<f32>>,
    pos_z: Vec<Option<f32>>,
    scale_x: Vec<Option<f32>>,
    scale_y: Vec<Option<f32>>,
    rotated: Vec<Option<f32>>,
    visible: Vec<Option<bool>>,
    cell: Vec<Option<(usize, usize)>>,
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

    pub fn add_pos_x<T: Into<Option<f32>>>(&mut self, x: T) {
        if self.pos_x.len() >= self.frame_count {
            panic!(
                "over limit {} pos x: {}",
                self.frame_count,
                self.pos_x.len(),
            );
        }
        self.pos_x.push(x.into());
    }

    pub fn add_pos_y<T: Into<Option<f32>>>(&mut self, y: T) {
        if self.pos_y.len() >= self.frame_count {
            panic!(
                "over limit {} pos y: {}",
                self.frame_count,
                self.pos_y.len(),
            );
        }
        self.pos_y.push(y.into());
    }

    pub fn add_pos_z<T: Into<Option<f32>>>(&mut self, z: T) {
        if self.pos_z.len() >= self.frame_count {
            panic!(
                "over limit {} pos z: {}",
                self.frame_count,
                self.pos_z.len(),
            );
        }
        self.pos_z.push(z.into());
    }

    pub fn add_scale_x<T: Into<Option<f32>>>(&mut self, x: T) {
        if self.scale_x.len() >= self.frame_count {
            panic!(
                "over limit {} scale x: {}",
                self.frame_count,
                self.scale_x.len(),
            );
        }
        self.scale_x.push(x.into());
    }

    pub fn add_scale_y<T: Into<Option<f32>>>(&mut self, y: T) {
        if self.scale_y.len() >= self.frame_count {
            panic!(
                "over limit {} scale y: {}",
                self.frame_count,
                self.scale_y.len(),
            );
        }
        self.scale_y.push(y.into());
    }

    pub fn add_rotated<T: Into<Option<f32>>>(&mut self, rotate: T) {
        if self.rotated.len() >= self.frame_count {
            panic!(
                "over limit {} rotated: {}",
                self.frame_count,
                self.rotated.len(),
            );
        }
        self.rotated.push(rotate.into());
    }

    pub fn add_visible<T: Into<Option<bool>>>(&mut self, visible: T) {
        if self.visible.len() >= self.frame_count {
            panic!(
                "over limit {} visible: {}",
                self.frame_count,
                self.visible.len(),
            );
        }
        self.visible.push(visible.into());
    }

    pub fn add_cell<T: Into<Option<(usize, usize)>>>(&mut self, cell: T) {
        if self.cell.len() >= self.frame_count {
            panic!("over limit {} cell: {}", self.frame_count, self.cell.len(),);
        }
        self.cell.push(cell.into());
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
            self.pos_x.push(None);
        }
        for _ in 0..(self.frame_count - self.pos_y.len()) {
            self.pos_y.push(None);
        }
        for _ in 0..(self.frame_count - self.pos_z.len()) {
            self.pos_z.push(None);
        }
        for _ in 0..(self.frame_count - self.scale_x.len()) {
            self.scale_x.push(None);
        }
        for _ in 0..(self.frame_count - self.scale_y.len()) {
            self.scale_y.push(None);
        }
        for _ in 0..(self.frame_count - self.rotated.len()) {
            self.rotated.push(None);
        }
        for _ in 0..(self.frame_count - self.users.len()) {
            self.users.push(None);
        }
        for _ in 0..(self.frame_count - self.visible.len()) {
            self.visible.push(None);
        }
        for _ in 0..(self.frame_count - self.cell.len()) {
            self.cell.push(None);
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
        );

        let mut transform = Transform::default();

        for (u, x, y, z, scale_x, scale_y, rotated, visible, cell) in frames {
            // transform は，直前のものを利用しつつ何らか値が入ったら変動値として扱う
            let transform = match (x, y, z, scale_x, scale_y, rotated) {
                (None, None, None, None, None, None) => None,
                (x, y, z, scale_x, scale_y, rotated) => {
                    x.map(|x| transform.set_translation_x(x));
                    y.map(|y| transform.set_translation_y(y));
                    z.map(|z| transform.set_translation_z(-z));
                    rotated.map(|rotated| transform.set_rotation_2d(rotated));
                    let mut scale = transform.scale().clone();
                    scale_x.map(|scale_x| scale.x = scale_x);
                    scale_y.map(|scale_y| scale.y = scale_y);
                    transform.set_scale(scale);
                    Some(transform.clone())
                }
            };

            let key_frame = KeyFrameBuilder::new()
                .user(u)
                .transform(transform)
                .visible(visible)
                .cell(cell)
                .build();

            timeline.key_frames.push(key_frame);
        }
        timeline
    }
}
