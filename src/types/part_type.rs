use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PartType {
    Null,
    Normal,
    Text,
    Instance,
}
