use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Debug, Clone)]
pub struct AnimationTime {
    current_time: f32,
    prev_time: f32,
    play_speed: f32,
}

impl AnimationTime {
    pub fn new() -> Self {
        AnimationTime {
            current_time: 0.0,
            prev_time: 0.0,
            play_speed: 1.0,
        }
    }

    pub fn set_speed(&mut self, play_speed: f32) {
        self.play_speed = play_speed;
    }

    pub fn set_time(&mut self, time: f32) {
        self.current_time = time;
        self.prev_time = time;
    }

    pub fn add_time(&mut self, delta: f32) {
        self.prev_time = self.current_time;
        self.current_time += delta * (self.play_speed);
    }

    pub fn add_second(&mut self, delta_sec: f32) {
        self.prev_time = self.current_time;
        self.current_time += delta_sec;
    }

    pub fn current_time(&self) -> f32 {
        self.current_time
    }

    pub fn prev_time(&self) -> f32 {
        self.prev_time
    }
}

impl Component for AnimationTime {
    type Storage = DenseVecStorage<Self>;
}
