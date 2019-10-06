use crate::{
    traits::from_user::FromUser,
    types::{
        animation_instance::{InstanceKey, InstanceKeyBuilder},
        non_decode_user::NonDecodedUser,
    },
};
use amethyst::{
    core::Transform,
    renderer::{palette::rgb::Srgba, resources::Tint},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KeyFrame<U>
where
    U: Serialize,
{
    #[serde(
        bound(
            serialize = "Option<U>: Serialize",
            deserialize = "Option<U>: Deserialize<'de>"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    user: Option<U>,
    #[serde(skip_serializing_if = "skip_transform", default = "Transform::default")]
    transform: Transform,
    #[serde(skip_serializing_if = "skip_visible", default = "default_visible")]
    visible: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    cell: Option<(usize, usize)>,
    #[serde(skip_serializing_if = "skip_tint", default = "tint_default")]
    color: Tint,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_key: Option<InstanceKey>,
}

impl<U> KeyFrame<U>
where
    U: FromUser + Serialize,
{
    pub fn user(&self) -> Option<&U> {
        self.user.as_ref()
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn cell(&self) -> Option<(usize, usize)> {
        self.cell
    }

    pub fn color(&self) -> &Tint {
        &self.color
    }

    pub fn instance_key(&self) -> Option<&InstanceKey> {
        self.instance_key.as_ref()
    }
}

// キーフレーム生成
pub(crate) struct KeyFrameBuilder {
    user: Option<NonDecodedUser>,
    transform: Transform,
    visible: bool,
    cell: Option<(usize, usize)>,
    color: Tint,
    instance_key: Option<InstanceKeyBuilder>,
}

impl KeyFrameBuilder {
    pub(crate) fn new() -> Self {
        KeyFrameBuilder {
            user: None,
            transform: Default::default(),
            visible: Default::default(),
            cell: Default::default(),
            color: Default::default(),
            instance_key: Default::default(),
        }
    }

    pub(crate) fn user<I: Into<Option<NonDecodedUser>>>(mut self, val: I) -> Self {
        self.user = val.into();
        self
    }

    pub(crate) fn transform(mut self, val: Transform) -> Self {
        self.transform = val;
        self
    }

    pub(crate) fn visible(mut self, val: bool) -> Self {
        self.visible = val;
        self
    }

    pub(crate) fn cell(mut self, val: Option<(usize, usize)>) -> Self {
        self.cell = val;
        self
    }

    pub(crate) fn color(mut self, val: Tint) -> Self {
        self.color = val;
        self
    }

    pub(crate) fn instance_key(mut self, val: Option<InstanceKeyBuilder>) -> Self {
        self.instance_key = val;
        self
    }

    pub(crate) fn build<U>(self) -> KeyFrame<U>
    where
        U: FromUser + Serialize,
    {
        KeyFrame {
            user: self
                .user
                .and_then(|val| U::from_user(val.integer, val.point, val.rect, val.text)),
            transform: self.transform,
            visible: self.visible,
            cell: self.cell,
            color: self.color,
            instance_key: self.instance_key.map(|builder| builder.build()),
        }
    }
}

fn skip_visible(visible: &bool) -> bool {
    *visible
}
fn default_visible() -> bool {
    true
}

fn skip_transform(transform: &Transform) -> bool {
    transform == &Transform::default()
}

fn skip_tint(color: &Tint) -> bool {
    let color: [f32; 4] = color.clone().into();
    let default_color = [1., 1., 1., 1.];

    let eq_default = {
        let color = &color;
        let default_color = &default_color;

        color
            .iter()
            .zip(default_color.iter())
            .all(|(c, d)| *c == *d)
    };
    eq_default
}

fn tint_default() -> Tint {
    Tint(Srgba::new(1., 1., 1., 1.))
}
