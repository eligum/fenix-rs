//! Math utilities module.

pub fn radians(degrees: f32) -> f32 {
    degrees * (std::f32::consts::PI / 180.0)
}
