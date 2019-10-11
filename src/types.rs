pub(crate) mod animation_instance;
pub(crate) mod animation_ref;
pub(crate) mod bound_type;
pub(crate) mod key_frame;
pub(crate) mod linear_color;
pub(crate) mod node;
pub(crate) mod non_decode_user;
pub(crate) mod part_info;
pub(crate) mod part_type;

pub use animation_instance::{InstanceKey, InstanceKeyBuilder};
pub use animation_ref::RefferenceAnimation;
pub use bound_type::Bounds;
pub use linear_color::LinearColor;
pub use part_type::PartType;
