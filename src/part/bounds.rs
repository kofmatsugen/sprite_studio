use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PartType {
    Null,
    Normal,
    Text,
    Instance,
}
