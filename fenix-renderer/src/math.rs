//! Useful math functions.

use std::f32::consts;

/// Takes an `angle` in degrees and converts it to radians.
#[inline(always)]
pub fn radians(angle: f32) -> f32 {
    angle * (consts::PI / 180.0)
}