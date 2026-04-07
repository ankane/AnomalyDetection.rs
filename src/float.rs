// TODO seal?
#[doc(hidden)]
pub trait Float: stlrs::Float {
    fn is_nan(&self) -> bool;
}

impl Float for f32 {
    fn is_nan(&self) -> bool {
        f32::is_nan(*self)
    }
}

impl Float for f64 {
    fn is_nan(&self) -> bool {
        f64::is_nan(*self)
    }
}
