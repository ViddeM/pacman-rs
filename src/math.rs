/// Linear interpolation between a & b by [interpolation].
/// interpolation should be a value between 0.0 and 1.0,
/// values larger (or equal) to 1.0 will return b
/// values smaller than 0 will return a
pub fn lerp(a: f32, b: f32, interpolation: f32) -> f32 {
    if interpolation >= 1.0 {
        return b;
    }

    if interpolation <= 0.0 {
        return a;
    }

    let diff = b - a;
    let progress = diff * interpolation;

    return a + progress;
}
