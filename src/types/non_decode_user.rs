#[derive(Debug)]
pub(crate) struct NonDecodedUser {
    pub(crate) integer: Option<i32>,
    pub(crate) point: Option<(f32, f32)>,
    pub(crate) rect: Option<(f32, f32, f32, f32)>,
    pub(crate) text: Option<String>,
}
