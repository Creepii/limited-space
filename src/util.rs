pub trait Lerp {
    fn lerp(self, b: Self, factor: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(self, b: f32, factor: f32) -> f32 {
        b * factor + self * (1.0 - factor)
    }
}
