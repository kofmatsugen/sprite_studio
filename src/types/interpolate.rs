use serde::{Deserialize, Serialize};
// 補完関数
#[derive(Debug, Serialize, Deserialize)]
pub enum Interpolation {
    Linear,
    Hermite,
    Bezier,
    Acceleration,
    Deceleration,
    Step,
}

// 補完関数に合わせてf32の値を返す
impl Interpolation {
    // 0.0 ~ 1.0 の値でその時間に合わせた補完割合を返す
    pub fn calc_rate(&self, time: f32) -> f32 {
        // もし0.0 ~ 1.0 の外にいたら中になるように補正する
        let time = num::clamp(time, 0.0, 1.0);

        match self {
            Interpolation::Linear => lerp(time),
            Interpolation::Hermite => hermite(time),
            Interpolation::Bezier => bezier(time),
            Interpolation::Acceleration => acceleration(time),
            Interpolation::Deceleration => deceleration(time),
            Interpolation::Step => step(time),
        }
    }
}

// 線形補間なのでtime をそのまま返す
fn lerp(time: f32) -> f32 {
    time
}

// ステップは補完しないので 0
fn step(_: f32) -> f32 {
    0.0
}

// 加速補完
// 0.0 -> 1.0 になる係数 1 の二次関数
fn acceleration(time: f32) -> f32 {
    time * time
}

// 減速補完
// 0.0 -> 1.0 になる係数 -1 の二次関数
fn deceleration(time: f32) -> f32 {
    -1. * (time - 1.) * (time - 1.) + 1.
}

// エルミート補完
fn hermite(_: f32) -> f32 {
    // エルミート補完がよくわからんので仮実装
    unimplemented!("unimplement hermite")
}

// ベジエ曲線補完
fn bezier(_: f32) -> f32 {
    // ベジエの補完がよくわからんので仮実装
    unimplemented!("unimplement bezier")
}
