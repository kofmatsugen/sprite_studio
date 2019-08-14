use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BoundsType {
    Null,
    Quad,
    Aabb,
    Circle,
    CircleMin,
    CircleMax,
}
