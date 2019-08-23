use super::from_user::{FromUser, NonDecodedUser};
use amethyst::core::Transform;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

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
}

impl<U> KeyFrame<U>
where
    U: FromUser + Serialize + DeserializeOwned,
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
}

// キーフレーム生成
pub(crate) struct KeyFrameBuilder {
    user: Option<NonDecodedUser>,
    transform: Option<Transform>,
    visible: Option<bool>,
}

impl KeyFrameBuilder {
    pub(crate) fn new() -> Self {
        KeyFrameBuilder {
            user: None,
            transform: None,
            visible: None,
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
        }
    }
}
