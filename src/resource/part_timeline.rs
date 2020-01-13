use super::timeline::{TimeLine, TimeLineBuilder};
use crate::types::{cell::Cell, interpolate::Interpolation, InstanceKey, LinearColor};
use amethyst::{
    core::{
        math::{Translation3, UnitQuaternion, Vector3},
        Transform,
    },
    renderer::resources::Tint,
};
use serde::{Deserialize, Serialize};

// パーツごとのタイムライン
#[derive(Debug, Serialize, Deserialize)]
pub struct PartTimeline<U> {
    // 非表示
    #[serde(
        default = "TimeLine::default",
        skip_serializing_if = "TimeLine::is_empty"
    )]
    hide: TimeLine<bool>,

    // セル情報
    #[serde(
        default = "TimeLine::default",
        skip_serializing_if = "TimeLine::is_empty"
    )]
    cell: TimeLine<Cell>,

    // 座標情報
    #[serde(
        default = "TimeLine::default",
        skip_serializing_if = "TimeLine::is_empty"
    )]
    pos_x: TimeLine<f32>,
    #[serde(
        default = "TimeLine::default",
        skip_serializing_if = "TimeLine::is_empty"
    )]
    pos_y: TimeLine<f32>,
    #[serde(
        default = "TimeLine::default",
        skip_serializing_if = "TimeLine::is_empty"
    )]
    pos_z: TimeLine<f32>,
    #[serde(
        default = "TimeLine::default",
        skip_serializing_if = "TimeLine::is_empty"
    )]
    scale_x: TimeLine<f32>,
    #[serde(
        default = "TimeLine::default",
        skip_serializing_if = "TimeLine::is_empty"
    )]
    scale_y: TimeLine<f32>,
    #[serde(
        default = "TimeLine::default",
        skip_serializing_if = "TimeLine::is_empty"
    )]
    rotated: TimeLine<f32>,
    // 反転情報
    #[serde(
        default = "TimeLine::default",
        skip_serializing_if = "TimeLine::is_empty"
    )]
    flip_v: TimeLine<bool>,
    #[serde(
        default = "TimeLine::default",
        skip_serializing_if = "TimeLine::is_empty"
    )]
    flip_h: TimeLine<bool>,
    // 色情報
    #[serde(
        default = "TimeLine::default",
        skip_serializing_if = "TimeLine::is_empty"
    )]
    alpha: TimeLine<f32>,
    #[serde(
        default = "TimeLine::default",
        skip_serializing_if = "TimeLine::is_empty"
    )]
    color: TimeLine<LinearColor>,

    // カスタムデータ
    #[serde(
        default = "TimeLine::default",
        skip_serializing_if = "TimeLine::is_empty"
    )]
    user: TimeLine<U>,
    // アニメーションインスタンス
    #[serde(
        default = "TimeLine::default",
        skip_serializing_if = "TimeLine::is_empty"
    )]
    instance: TimeLine<InstanceKey>,
}

impl<U> PartTimeline<U> {
    // 表示ON/OFF取得
    // キーが存在しなければ表示無し
    pub fn hide(&self, frame: usize) -> bool {
        self.hide.get_step_key(frame).map(|v| *v).unwrap_or(true)
    }

    pub fn cell(&self, frame: usize) -> Option<&Cell> {
        self.cell.get_step_key(frame)
    }

    // パーツのローカル座標を取得
    // 座標に関連するフレームは常に存在するはずなので，存在しなければクラッシュ
    pub fn local_transform(&self, frame: usize) -> Transform {
        let pos_x = self.pos_x.get_interpolation_key(frame).unwrap_or(0.);
        let pos_y = self.pos_y.get_interpolation_key(frame).unwrap_or(0.);
        let pos_z = self.pos_z.get_interpolation_key(frame).unwrap_or(0.);
        let scale_x = self.scale_x.get_interpolation_key(frame).unwrap_or(1.);
        let scale_y = self.scale_y.get_interpolation_key(frame).unwrap_or(1.);
        let rotated = self.rotated.get_interpolation_key(frame).unwrap_or(0.);

        let position = Translation3::new(pos_x, pos_y, pos_z);
        let rotation = UnitQuaternion::from_euler_angles(0., 0., rotated.to_radians());
        let scale = Vector3::new(scale_x, scale_y, 1.0);

        Transform::new(position, rotation, scale)
    }

    // スプライトの色情報取得
    // キーフレームがなければ色変更なし
    pub fn color(&self, frame: usize) -> Tint {
        let LinearColor(r, g, b, _) = self
            .color
            .get_interpolation_key(frame)
            .unwrap_or(LinearColor(1., 1., 1., 1.));
        let alpha = self.alpha.get_interpolation_key(frame).unwrap_or(1.);

        LinearColor(r, g, b, alpha).into()
    }

    // ユーザーパラメータの取得
    pub fn user(&self, frame: usize) -> Option<&U> {
        self.user.get_step_key(frame)
    }

    pub fn instance(&self, frame: usize) -> Option<&InstanceKey> {
        self.instance.get_step_key(frame)
    }
}

pub(crate) struct PartTimelineBuilder<U> {
    // 非表示
    hide: TimeLineBuilder<bool>,

    // セル情報
    cell: TimeLineBuilder<Cell>,

    // 座標情報
    pos_x: TimeLineBuilder<f32>,
    pos_y: TimeLineBuilder<f32>,
    pos_z: TimeLineBuilder<f32>,
    scale_x: TimeLineBuilder<f32>,
    scale_y: TimeLineBuilder<f32>,
    rotated: TimeLineBuilder<f32>,
    // 反転情報
    flip_v: TimeLineBuilder<bool>,
    flip_h: TimeLineBuilder<bool>,
    // 色情報
    alpha: TimeLineBuilder<f32>,
    color: TimeLineBuilder<LinearColor>,

    // カスタムデータ
    user: TimeLineBuilder<U>,

    // アニメーションインスタンス
    instance: TimeLineBuilder<InstanceKey>,
}

impl<U> PartTimelineBuilder<U> {
    pub fn new() -> Self {
        PartTimelineBuilder {
            // 表示ON/OFF
            hide: TimeLineBuilder::new(),
            // セル情報
            cell: TimeLineBuilder::new(),
            // 座標情報
            pos_x: TimeLineBuilder::new(),
            pos_y: TimeLineBuilder::new(),
            pos_z: TimeLineBuilder::new(),
            scale_x: TimeLineBuilder::new(),
            scale_y: TimeLineBuilder::new(),
            rotated: TimeLineBuilder::new(),
            // 反転情報
            flip_v: TimeLineBuilder::new(),
            flip_h: TimeLineBuilder::new(),
            // 色情報
            alpha: TimeLineBuilder::new(),
            color: TimeLineBuilder::new(),
            // カスタムデータ
            user: TimeLineBuilder::new(),
            // アニメーションインスタンス
            instance: TimeLineBuilder::new(),
        }
    }

    pub fn add_hide(&mut self, frame: usize, interpolation: Interpolation, hide: bool) {
        self.hide.add_key(frame, interpolation, hide);
    }

    pub fn add_cell(&mut self, frame: usize, interpolation: Interpolation, cell: Cell) {
        self.cell.add_key(frame, interpolation, cell);
    }

    pub fn add_pos_x(&mut self, frame: usize, interpolation: Interpolation, pos_x: f32) {
        self.pos_x.add_key(frame, interpolation, pos_x);
    }
    pub fn add_pos_y(&mut self, frame: usize, interpolation: Interpolation, pos_y: f32) {
        self.pos_y.add_key(frame, interpolation, pos_y);
    }
    pub fn add_pos_z(&mut self, frame: usize, interpolation: Interpolation, pos_z: f32) {
        self.pos_z.add_key(frame, interpolation, pos_z);
    }

    pub fn add_scale_x(&mut self, frame: usize, interpolation: Interpolation, scale_x: f32) {
        self.scale_x.add_key(frame, interpolation, scale_x);
    }
    pub fn add_scale_y(&mut self, frame: usize, interpolation: Interpolation, scale_y: f32) {
        self.scale_y.add_key(frame, interpolation, scale_y);
    }
    pub fn add_rotated(&mut self, frame: usize, interpolation: Interpolation, rotated: f32) {
        self.rotated.add_key(frame, interpolation, rotated);
    }
    // 反転情報
    pub fn add_flip_v(&mut self, frame: usize, interpolation: Interpolation, flip_v: bool) {
        self.flip_v.add_key(frame, interpolation, flip_v);
    }
    pub fn add_flip_h(&mut self, frame: usize, interpolation: Interpolation, flip_h: bool) {
        self.flip_h.add_key(frame, interpolation, flip_h);
    }
    // 色情報
    pub fn add_alpha(&mut self, frame: usize, interpolation: Interpolation, alpha: f32) {
        self.alpha.add_key(frame, interpolation, alpha);
    }
    pub fn add_color(&mut self, frame: usize, interpolation: Interpolation, color: LinearColor) {
        self.color.add_key(frame, interpolation, color);
    }

    // カスタムデータ
    pub fn add_user(&mut self, frame: usize, interpolation: Interpolation, user: U) {
        self.user.add_key(frame, interpolation, user);
    }

    // アニメーションインスタンス
    pub fn add_instance(
        &mut self,
        frame: usize,
        interpolation: Interpolation,
        instance: InstanceKey,
    ) {
        self.instance.add_key(frame, interpolation, instance);
    }

    pub fn build(self) -> PartTimeline<U> {
        PartTimeline {
            // 表示ON/OFF
            hide: self.hide.build(),
            // セル情報
            cell: self.cell.build(),
            // 座標情報
            pos_x: self.pos_x.build(),
            pos_y: self.pos_y.build(),
            pos_z: self.pos_z.build(),
            scale_x: self.scale_x.build(),
            scale_y: self.scale_y.build(),
            rotated: self.rotated.build(),
            // 反転情報
            flip_v: self.flip_v.build(),
            flip_h: self.flip_h.build(),
            // 色情報
            alpha: self.alpha.build(),
            color: self.color.build(),
            // カスタムデータ
            user: self.user.build(),
            // アニメーションインスタンス
            instance: self.instance.build(),
        }
    }
}
