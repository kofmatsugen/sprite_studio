pub trait FromUser
where
    Self: std::marker::Sized,
{
    fn from_user(
        integer: Option<i32>,
        point: Option<(f32, f32)>,
        rect: Option<(f32, f32, f32, f32)>,
        text: Option<String>,
    ) -> Option<Self>;
}

impl FromUser for () {
    fn from_user(
        _integer: Option<i32>,
        _point: Option<(f32, f32)>,
        _rect: Option<(f32, f32, f32, f32)>,
        _text: Option<String>,
    ) -> Option<Self> {
        Some(())
    }
}

impl FromUser for i32 {
    fn from_user(
        integer: Option<i32>,
        _point: Option<(f32, f32)>,
        _rect: Option<(f32, f32, f32, f32)>,
        _text: Option<String>,
    ) -> Option<Self> {
        integer
    }
}

impl FromUser for (f32, f32) {
    fn from_user(
        _integer: Option<i32>,
        point: Option<(f32, f32)>,
        _rect: Option<(f32, f32, f32, f32)>,
        _text: Option<String>,
    ) -> Option<Self> {
        point
    }
}

impl FromUser for (f32, f32, f32, f32) {
    fn from_user(
        _integer: Option<i32>,
        _point: Option<(f32, f32)>,
        rect: Option<(f32, f32, f32, f32)>,
        _text: Option<String>,
    ) -> Option<Self> {
        rect
    }
}

impl FromUser for String {
    fn from_user(
        _integer: Option<i32>,
        _point: Option<(f32, f32)>,
        _rect: Option<(f32, f32, f32, f32)>,
        text: Option<String>,
    ) -> Option<Self> {
        text
    }
}
