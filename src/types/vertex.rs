use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul, Sub};

// 頂点アニメーションのキー
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct VertexKey {
    lt: (f32, f32), // 左上
    rt: (f32, f32), // 右上
    lb: (f32, f32), // 左下
    rb: (f32, f32), // 右下
}

impl VertexKey {
    pub fn lt(&self) -> (f32, f32) {
        self.lt
    }
    pub fn rt(&self) -> (f32, f32) {
        self.rt
    }
    pub fn lb(&self) -> (f32, f32) {
        self.lb
    }
    pub fn rb(&self) -> (f32, f32) {
        self.rb
    }
}

impl Add<VertexKey> for VertexKey {
    type Output = Self;

    fn add(self, rhs: VertexKey) -> Self::Output {
        VertexKey {
            lt: (self.lt.0 + rhs.lt.0, self.lt.1 + rhs.lt.1),
            rt: (self.rt.0 + rhs.rt.0, self.rt.1 + rhs.rt.1),
            lb: (self.lb.0 + rhs.lb.0, self.lb.1 + rhs.lb.1),
            rb: (self.rb.0 + rhs.rb.0, self.rb.1 + rhs.rb.1),
        }
    }
}

impl Sub<VertexKey> for VertexKey {
    type Output = Self;

    fn sub(self, rhs: VertexKey) -> Self::Output {
        VertexKey {
            lt: (self.lt.0 - rhs.lt.0, self.lt.1 - rhs.lt.1),
            rt: (self.rt.0 - rhs.rt.0, self.rt.1 - rhs.rt.1),
            lb: (self.lb.0 - rhs.lb.0, self.lb.1 - rhs.lb.1),
            rb: (self.rb.0 - rhs.rb.0, self.rb.1 - rhs.rb.1),
        }
    }
}

impl Mul<f32> for VertexKey {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        VertexKey {
            lt: (self.lt.0 * rhs, self.lt.1 * rhs),
            rt: (self.rt.0 * rhs, self.rt.1 * rhs),
            lb: (self.lb.0 * rhs, self.lb.1 * rhs),
            rb: (self.rb.0 * rhs, self.rb.1 * rhs),
        }
    }
}

#[cfg(feature = "builder")]
pub struct VertexKeyBuilder {
    lt: Option<(f32, f32)>, // 左上
    rt: Option<(f32, f32)>, // 右上
    lb: Option<(f32, f32)>, // 左下
    rb: Option<(f32, f32)>, // 右下
}

#[cfg(feature = "builder")]
impl VertexKeyBuilder {
    pub fn new() -> Self {
        VertexKeyBuilder {
            lt: None,
            rt: None,
            lb: None,
            rb: None,
        }
    }

    pub fn lt(mut self, lt: (f32, f32)) -> Self {
        self.lt = lt.into();
        self
    }
    pub fn rt(mut self, rt: (f32, f32)) -> Self {
        self.rt = rt.into();
        self
    }
    pub fn lb(mut self, lb: (f32, f32)) -> Self {
        self.lb = lb.into();
        self
    }
    pub fn rb(mut self, rb: (f32, f32)) -> Self {
        self.rb = rb.into();
        self
    }

    pub fn build(self) -> VertexKey {
        VertexKey {
            lt: self.lt.unwrap_or((0., 0.)),
            rt: self.rt.unwrap_or((0., 0.)),
            lb: self.lb.unwrap_or((0., 0.)),
            rb: self.rb.unwrap_or((0., 0.)),
        }
    }
}
