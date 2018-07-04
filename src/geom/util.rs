/// Find if two floating points are about equal
///
/// Exact floating point equality will cause floating-point precision issues,
/// so using == on floats in general is a bad idea.
pub fn about_equal(a: f32, b: f32) -> bool { (a - b).abs() < 0.0001 }

/// Linearly interpolate between two values
pub fn lerp(current: f32, target: f32, fraction: f32) -> f32 {
    current + (target - current) * fraction
}

/// Linearly interpolate between two angles, trying to find the shortest path
///
/// Simply using [`lerp`] will yield odd turning patterns, because angles wrap
/// at 0 and 360.
///
/// [`lerp`]: fn.lerp.html
pub fn lerp_angle(current: f32, target: f32, fraction: f32) -> f32 {
    let delta = ((target - current + 360f32 + 180f32) % 360f32) - 180f32;
    (current + delta * fraction + 360f32) % 360f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        assert!(about_equal(0.5, 0.5));
        assert!(about_equal(0.5, 0.500000001));
        assert!(!about_equal(0.6, 0.500000001));
    }

    #[test]
    fn test_lerp() {
        assert!(about_equal(lerp(0f32, 10f32, 0.5), 5f32));
    }

    #[test]
    fn test_lerp_angle() {
        assert!(about_equal(lerp_angle(45f32, 315f32, 0.5), 0f32));
    }
}
