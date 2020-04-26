use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EffectKey {
    start_time: usize,
    independent: bool,
    speed: f32,
}

impl EffectKey {
    pub fn start_time(&self) -> usize {
        self.start_time
    }

    pub fn independent(&self) -> bool {
        self.independent
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }
}

#[cfg(feature = "builder")]
pub struct EffectKeyBuilder {
    start_time: Option<usize>,
    independent: Option<bool>,
    speed: Option<f32>,
}

#[cfg(feature = "builder")]
impl EffectKeyBuilder {
    pub fn new() -> Self {
        EffectKeyBuilder {
            start_time: None,
            independent: None,
            speed: None,
        }
    }

    pub fn start_time(mut self, time: usize) -> Self {
        self.start_time = Some(time);
        self
    }
    pub fn independent(mut self, independent: bool) -> Self {
        self.independent = Some(independent);
        self
    }
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = Some(speed);
        self
    }

    pub fn build(self) -> EffectKey {
        EffectKey {
            start_time: self.start_time.unwrap_or(0),
            independent: self.independent.unwrap_or(false),
            speed: self.speed.unwrap_or(1.),
        }
    }
}
