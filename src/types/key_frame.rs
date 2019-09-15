use super::from_user::{FromUser, NonDecodedUser};
use amethyst::{core::Transform, renderer::resources::Tint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KeyFrame<U>
where
    U: Serialize,
{
    #[serde(bound(
        serialize = "Option<U>: Serialize",
        deserialize = "Option<U>: Deserialize<'de>"
    ))]
    user: Option<U>,
    transform: Transform,
    visible: bool,
    cell: Option<(usize, usize)>,
    color: Tint,
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
}

// キーフレーム生成
pub(crate) struct KeyFrameBuilder {
    user: Option<NonDecodedUser>,
    transform: Transform,
    visible: bool,
    cell: Option<(usize, usize)>,
    color: Tint,
}

impl KeyFrameBuilder {
    pub(crate) fn new() -> Self {
        KeyFrameBuilder {
            user: None,
            transform: Default::default(),
            visible: Default::default(),
            cell: Default::default(),
            color: Default::default(),
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

    pub(crate) fn build<U>(self) -> KeyFrame<U>
    where
        U: FromUser + Serialize,
    {
        KeyFrame {
            user: self
                .user
                .map(|val| U::from_user(val.integer, val.point, val.rect, val.text)),
            transform: self.transform,
            visible: self.visible,
            cell: self.cell,
            color: self.color,
        }
    }
}
