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
    transform: Option<Transform>,
    visible: Option<bool>,
    cell: Option<(usize, usize)>,
    color: Option<Tint>,
}

impl<U> KeyFrame<U>
where
    U: FromUser + Serialize,
{
    pub(crate) fn user(&self) -> Option<&U> {
        self.user.as_ref()
    }

    pub(crate) fn transform(&self) -> Option<&Transform> {
        self.transform.as_ref()
    }

    pub(crate) fn visible(&self) -> Option<bool> {
        self.visible
    }

    pub(crate) fn cell(&self) -> Option<(usize, usize)> {
        self.cell
    }

    pub(crate) fn color(&self) -> Option<&Tint> {
        self.color.as_ref()
    }
}

// キーフレーム生成
pub(crate) struct KeyFrameBuilder {
    user: Option<NonDecodedUser>,
    transform: Option<Transform>,
    visible: Option<bool>,
    cell: Option<(usize, usize)>,
    color: Option<Tint>,
}

impl KeyFrameBuilder {
    pub(crate) fn new() -> Self {
        KeyFrameBuilder {
            user: None,
            transform: None,
            visible: None,
            cell: None,
            color: None,
        }
    }

    pub(crate) fn user<I: Into<Option<NonDecodedUser>>>(mut self, val: I) -> Self {
        self.user = val.into();
        self
    }

    pub(crate) fn transform<T: Into<Option<Transform>>>(mut self, val: T) -> Self {
        self.transform = val.into();
        self
    }

    pub(crate) fn visible<T: Into<Option<bool>>>(mut self, val: T) -> Self {
        self.visible = val.into();
        self
    }

    pub(crate) fn cell<T: Into<Option<(usize, usize)>>>(mut self, val: T) -> Self {
        self.cell = val.into();
        self
    }
    pub(crate) fn color<T: Into<Option<Tint>>>(mut self, val: T) -> Self {
        self.color = val.into();
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
