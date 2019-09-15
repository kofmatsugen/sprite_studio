use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Bounds {
    Quad,
    Aabb,
    Circle,
    CircleMin,
    CircleMax,
}
