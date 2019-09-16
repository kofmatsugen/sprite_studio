#[derive(Debug)]
pub(crate) struct NonDecodedUser {
    pub(crate) integer: Option<i32>,
    pub(crate) point: Option<(f32, f32)>,
    pub(crate) rect: Option<(f32, f32, f32, f32)>,
    pub(crate) text: Option<String>,
}

pub trait FromUser {
    fn from_user(
        integer: Option<i32>,
        point: Option<(f32, f32)>,
        rect: Option<(f32, f32, f32, f32)>,
        text: Option<String>,
    ) -> Self;
}

impl FromUser for () {
    fn from_user(
        _integer: Option<i32>,
        _point: Option<(f32, f32)>,
        _rect: Option<(f32, f32, f32, f32)>,
        _text: Option<String>,
    ) -> Self {
        ()
    }
}

impl FromUser for Option<i32> {
    fn from_user(
        integer: Option<i32>,
        _point: Option<(f32, f32)>,
        _rect: Option<(f32, f32, f32, f32)>,
        _text: Option<String>,
    ) -> Self {
        integer
    }
}

impl FromUser for Option<(f32, f32)> {
    fn from_user(
        _integer: Option<i32>,
        point: Option<(f32, f32)>,
        _rect: Option<(f32, f32, f32, f32)>,
        _text: Option<String>,
    ) -> Self {
        point
    }
}

impl FromUser for Option<(f32, f32, f32, f32)> {
    fn from_user(
        _integer: Option<i32>,
        _point: Option<(f32, f32)>,
        rect: Option<(f32, f32, f32, f32)>,
        _text: Option<String>,
    ) -> Self {
        rect
    }
}

impl FromUser for Option<String> {
    fn from_user(
        _integer: Option<i32>,
        _point: Option<(f32, f32)>,
        _rect: Option<(f32, f32, f32, f32)>,
        text: Option<String>,
    ) -> Self {
        text
    }
}
