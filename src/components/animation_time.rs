use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Debug, Clone)]
pub enum AnimationTime {
    Play {
        current_time: f32,
        prev_time: Option<f32>,
        play_speed: f32,
    },
    Stop {
        stopped_time: f32,      // 停止時点のでアニメーション再生時間
        stop_time: Option<f32>, // 停止を行う時間
        play_speed: f32,        // 再生時の再生速度を一応覚えておく
    },
}

impl AnimationTime {
    pub fn new() -> Self {
        AnimationTime::Stop {
            stopped_time: 0.,
            stop_time: None,
            play_speed: 1.,
        }
    }

    pub fn is_play(&self) -> bool {
        match self {
            AnimationTime::Play { .. } => true,
            _ => false,
        }
    }

    pub fn is_stop(&self) -> bool {
        match self {
            AnimationTime::Stop { .. } => true,
            _ => false,
        }
    }

    pub fn play<T: Into<Option<f32>>>(&mut self, speed: T) {
        let (current_time, play_speed) = match self {
            &mut AnimationTime::Play {
                current_time,
                play_speed,
                ..
            } => (current_time, play_speed),
            &mut AnimationTime::Stop {
                stopped_time,
                play_speed,
                ..
            } => (stopped_time, play_speed),
        };
        *self = AnimationTime::Play {
            current_time,
            prev_time: None,
            play_speed: speed.into().unwrap_or(play_speed),
        }
    }

    pub fn stop<T: Into<Option<f32>>>(&mut self, stop_time: T) {
        let stop_time = stop_time.into();
        let (play_speed, stopped_time) = match self {
            AnimationTime::Play {
                current_time,
                play_speed,
                ..
            } => (*play_speed, *current_time),
            AnimationTime::Stop {
                stopped_time,
                play_speed,
                ..
            } => (*play_speed, *stopped_time),
        };
        log::debug!(
            "stop: time = {:?}, stopped_time = {:.2}",
            stop_time,
            stopped_time
        );
        *self = AnimationTime::Stop {
            stopped_time,
            stop_time,
            play_speed,
        }
    }

    pub fn play_time(&self) -> f32 {
        match self {
            &AnimationTime::Play { current_time, .. } => current_time,
            &AnimationTime::Stop { stopped_time, .. } => stopped_time,
        }
    }

    pub fn prev_time(&self) -> Option<f32> {
        match self {
            &AnimationTime::Play { prev_time, .. } => prev_time,
            &AnimationTime::Stop { .. } => None,
        }
    }

    pub fn play_frame(&self, fps: f32) -> usize {
        let play_time = self.play_time();
        let float_frame = play_time * fps;

        float_frame.floor() as usize
    }

    pub fn prev_frame(&self, fps: f32) -> Option<usize> {
        let prev_time = self.prev_time()?;
        let float_frame = prev_time * fps;

        Some(float_frame.floor() as usize)
    }

    pub fn set_play_speed(&mut self, speed: f32) {
        if let AnimationTime::Play { play_speed, .. } = self {
            *play_speed = speed;
        } else {
            log::warn!("play speed set failed: {:?}", self);
        }
    }

    pub fn set_play_time(&mut self, time: f32) {
        match self {
            AnimationTime::Play {
                current_time,
                prev_time,
                ..
            } => {
                *current_time = time;
                *prev_time = None;
            }
            AnimationTime::Stop { stopped_time, .. } => {
                *stopped_time = time;
            }
        }
    }

    // 再生中なら加算，停止中なら停止時間を減算
    pub(crate) fn add_time(&mut self, delta: f32) {
        let mut stop_end_time = None;
        match self {
            AnimationTime::Play {
                prev_time,
                current_time,
                play_speed,
                ..
            } => {
                // 通常再生は速度を考慮
                *prev_time = Some(*current_time);
                *current_time += delta * *play_speed;
            }
            AnimationTime::Stop {
                stop_time: Some(time),
                play_speed,
                stopped_time,
            } => {
                if *time > delta {
                    *time -= delta;
                } else {
                    // 停止時間を超えてたら再生開始
                    // 超過分は再生速度を考慮する
                    let play_frame = *stopped_time + (delta - *time) * *play_speed;
                    stop_end_time = Some((*play_speed, play_frame));
                }
            }
            _ => {}
        }

        if let Some((play_speed, current_time)) = stop_end_time {
            log::debug!("end stop: start from {}", current_time);
            *self = AnimationTime::Play {
                current_time,
                prev_time: Some(current_time),
                play_speed,
            };
        }
    }

    // 再生速度に関係なく秒数加算
    pub fn add_second(&mut self, delta_sec: f32) {
        match self {
            AnimationTime::Play {
                prev_time,
                current_time,
                ..
            } => {
                *prev_time = Some(*current_time);
                *current_time += delta_sec;
            }
            AnimationTime::Stop {
                stop_time: Some(time),
                ..
            } => {
                // 停止時間に速度は関係ない
                *time -= delta_sec;
            }
            _ => {}
        }
    }
}

impl Component for AnimationTime {
    type Storage = DenseVecStorage<Self>;
}
