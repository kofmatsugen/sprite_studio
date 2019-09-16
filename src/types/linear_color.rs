use amethyst::renderer::{palette::rgb::Srgba, resources::Tint};

#[derive(Debug, Clone, Copy)]
pub struct LinearColor(pub f32, pub f32, pub f32, pub f32);

impl Default for LinearColor {
    fn default() -> Self {
        LinearColor(1., 1., 1., 1.)
    }
}

impl Into<Tint> for LinearColor {
    fn into(self) -> Tint {
        Tint(Srgba::new(self.0, self.1, self.2, self.3))
    }
}

impl std::ops::Mul<f32> for LinearColor {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        LinearColor(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}
impl std::ops::Add<LinearColor> for LinearColor {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        LinearColor(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}
