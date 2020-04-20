use crate::{traits::interpolate::Interpolate, types::interpolate::Interpolation};
use serde::{Deserialize, Serialize};

// KeyFrameのリストがタイムライン
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeLine<T> {
    key_frames: Vec<KeyFrame<T>>,
}

// デフォルト実装．補完できないものでもステップ形式のキーフレーム取得はサポートする
// コピー実装は不要なので参照を返す
impl<T> TimeLine<T> {
    pub fn get_step_key(&self, frame: usize) -> Option<&T> {
        self.key_frames
            .iter()
            .rev()
            .find(|k| k.frame <= frame)
            .map(|k| &k.value)
    }
    // シリアライズ時のスキップ条件
    pub(crate) fn is_empty(&self) -> bool {
        self.key_frames.is_empty()
    }

    // ステップのキーのあるフレーム数も一緒に取得
    pub fn get_step_key_with_frame(&self, frame: usize) -> Option<(usize, &T)> {
        self.key_frames
            .iter()
            .rev()
            .find(|k| k.frame <= frame)
            .map(|k| (k.frame, &k.value))
    }

    // デシリアライズ時に存在しなければデフォルト値にするための関数
    // Default trait だと外部から生成できてしまうためcrate内関数
    pub(crate) fn default() -> Self {
        TimeLine { key_frames: vec![] }
    }
}

// 補完可能なら補完処理に対応
// 計算処理上コピーが必要なので参照ではなく値を返す
impl<T> TimeLine<T>
where
    T: Interpolate,
{
    // 補完のためのキーフレーム取得(指定フレームを超えない最後のフレーム)
    fn left_key_frame(&self, frame: usize) -> Option<&KeyFrame<T>> {
        self.key_frames.iter().rev().find(|k| k.frame <= frame)
    }

    // 補完のためのキーフレーム取得(指定フレームを超える最初のフレーム)
    fn right_key_frame(&self, frame: usize) -> Option<&KeyFrame<T>> {
        self.key_frames.iter().find(|k| k.frame > frame)
    }

    pub fn get_interpolation_key(&self, frame: usize) -> Option<T> {
        let left_key = self.left_key_frame(frame);
        let right_key = self.right_key_frame(frame);

        match (left_key, right_key) {
            (Some(left_key), Some(right_key)) => {
                let rate = Self::easing_rate(
                    left_key.frame,
                    right_key.frame,
                    frame,
                    &left_key.interpolation,
                );
                Some((right_key.value - left_key.value) * rate + left_key.value)
            }
            (Some(left_key), None) => Some(left_key.value), // 後ろのキーがなければ補完は関係ない
            _ => None, // キーが存在しないか手前のキーがなければ何もキーがない
        }
    }

    fn easing_rate(
        l_frame: usize,
        r_frame: usize,
        current_frame: usize,
        function: &Interpolation,
    ) -> f32 {
        if l_frame == r_frame {
            return 1.0f32;
        }
        let left = l_frame as f32;
        let right = r_frame as f32;
        let current = current_frame as f32;

        let rate = (current - left) / (right - left);

        function.calc_rate(rate)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct KeyFrame<T> {
    frame: usize,
    interpolation: Interpolation,
    #[serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))]
    value: T,
}

pub struct TimeLineBuilder<T> {
    key_frames: Vec<KeyFrame<T>>,
}

impl<T> TimeLineBuilder<T> {
    pub fn new() -> Self {
        TimeLineBuilder { key_frames: vec![] }
    }

    pub fn add_key(&mut self, frame: usize, interpolation: Interpolation, value: T) {
        self.key_frames.push(KeyFrame {
            frame,
            interpolation,
            value,
        })
    }

    pub fn build(mut self) -> TimeLine<T> {
        // キーフレームをフレーム数に合わせてソートする
        self.key_frames.sort_by_key(|k| k.frame);
        TimeLine {
            key_frames: self.key_frames,
        }
    }
}
