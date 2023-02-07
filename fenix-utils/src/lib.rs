//! Useful math functions.

pub mod math {
    use std::f32::consts;

    /// Takes an `angle` in degrees and converts it to radians.
    #[inline(always)]
    pub fn radians(angle: f32) -> f32 {
        angle * (consts::PI / 180.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stupid_test() {
        assert_eq!(math::radians(90.0), 90.0 * (std::f32::consts::PI / 180.0));
    }
}
