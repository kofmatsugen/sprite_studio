const DEFAULT_COLOR: [f32; 4] = [1., 0., 0., 1.];

pub trait CollisionColor {
    // Option<Self> 型用のデフォルトカラー
    fn default_color() -> [f32; 4] {
        // 実装がなければ赤
        DEFAULT_COLOR
    }

    fn color(&self) -> [f32; 4] {
        // 実装がなければデフォルトカラー
        Self::default_color()
    }
}

// Option 型の場合は，Noneならデフォルトカラーを使用するようにする
impl<T: CollisionColor> CollisionColor for Option<&T> {
    fn color(&self) -> [f32; 4] {
        match self {
            Some(val) => val.color(),
            None => T::default_color(),
        }
    }
}

// Unit 型はデフォルトで実装しておく
impl CollisionColor for () {}
