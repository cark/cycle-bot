use bevy::prelude::*;
use std::time::Duration;

/// Trait for types that support linear interpolation and normalization.
pub trait Lerpable: Sized {
    fn lerp(range: (Self, Self), t: f32) -> Self;
    fn unlerp(range: (Self, Self), value: Self) -> f32;
}

impl Lerpable for f32 {
    fn lerp(range: (f32, f32), t: f32) -> f32 {
        let (start, end) = range;
        start + (end - start) * t
    }

    fn unlerp(range: (f32, f32), value: f32) -> f32 {
        let (start, end) = range;
        (value - start) / (end - start)
    }
}

impl Lerpable for f64 {
    fn lerp(range: (f64, f64), t: f32) -> f64 {
        let (start, end) = range;
        start + (end - start) * t as f64
    }

    fn unlerp(range: (f64, f64), value: f64) -> f32 {
        let (start, end) = range;
        ((value - start) / (end - start)) as f32
    }
}

impl Lerpable for Vec2 {
    fn lerp(range: (Vec2, Vec2), t: f32) -> Vec2 {
        let (start, end) = range;
        start + (end - start) * t
    }

    fn unlerp(range: (Vec2, Vec2), value: Vec2) -> f32 {
        let (start, end) = range;
        (value - start).length() / (end - start).length()
    }
}

impl Lerpable for Vec3 {
    fn lerp(range: (Vec3, Vec3), t: f32) -> Vec3 {
        let (start, end) = range;
        start + (end - start) * t
    }

    fn unlerp(range: (Vec3, Vec3), value: Vec3) -> f32 {
        let (start, end) = range;
        (value - start).length() / (end - start).length()
    }
}

/// Linearly interpolates a value within a given range based on a factor `t`.
pub fn lerp<T: Lerpable>(range: (T, T), t: f32) -> T {
    T::lerp(range, t)
}

/// Calculates the normalized position of a value within a given range.
pub fn unlerp<T: Lerpable>(range: (T, T), value: T) -> f32 {
    T::unlerp(range, value)
}

/// Smoothly interpolates between two values using exponential decay.
pub fn smooth_lerp<T: Lerpable>(start: T, end: T, dt: Duration, half_life: Duration) -> T {
    lerp(
        (end, start),
        (-dt.as_secs_f32() / half_life.as_secs_f32()).exp2(),
    )
}

/// Remaps a value from one range to another.
pub fn remap<T: Lerpable>(value: T, from_range: (T, T), to_range: (T, T)) -> T {
    let normalized = unlerp(from_range, value);
    lerp(to_range, normalized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::math::vec2;
    use bevy::math::vec3;

    #[test]
    fn test_lerp_f32() {
        assert_eq!(lerp((0.0_f32, 10.0_f32), 0.5), 5.0);
        assert_eq!(lerp((0.0_f32, 10.0_f32), 0.0), 0.0);
        assert_eq!(lerp((0.0_f32, 10.0_f32), 1.0), 10.0);
        assert_eq!(lerp((-5.0_f32, 5.0_f32), 0.5), 0.0);
    }

    #[test]
    fn test_lerp_f64() {
        assert_eq!(lerp((0.0_f64, 10.0_f64), 0.5), 5.0);
        assert_eq!(lerp((0.0_f64, 10.0_f64), 0.0), 0.0);
        assert_eq!(lerp((0.0_f64, 10.0_f64), 1.0), 10.0);
        assert_eq!(lerp((-5.0_f64, 5.0_f64), 0.5), 0.0);
    }

    #[test]
    fn test_lerp_vec2() {
        assert_eq!(
            lerp((vec2(0.0, 0.0), vec2(10.0, 10.0)), 0.5),
            vec2(5.0, 5.0)
        );
        assert_eq!(
            lerp((vec2(0.0, 0.0), vec2(10.0, 10.0)), 0.0),
            vec2(0.0, 0.0)
        );
        assert_eq!(
            lerp((vec2(0.0, 0.0), vec2(10.0, 10.0)), 1.0),
            vec2(10.0, 10.0)
        );
        assert_eq!(
            lerp((vec2(-5.0, -5.0), vec2(5.0, 5.0)), 0.5),
            vec2(0.0, 0.0)
        );
    }

    #[test]
    fn test_lerp_vec3() {
        assert_eq!(
            lerp((vec3(0.0, 0.0, 0.0), vec3(10.0, 10.0, 10.0)), 0.5),
            vec3(5.0, 5.0, 5.0)
        );
        assert_eq!(
            lerp((vec3(0.0, 0.0, 0.0), vec3(10.0, 10.0, 10.0)), 0.0),
            vec3(0.0, 0.0, 0.0)
        );
        assert_eq!(
            lerp((vec3(0.0, 0.0, 0.0), vec3(10.0, 10.0, 10.0)), 1.0),
            vec3(10.0, 10.0, 10.0)
        );
        assert_eq!(
            lerp((vec3(-5.0, -5.0, -5.0), vec3(5.0, 5.0, 5.0)), 0.5),
            vec3(0.0, 0.0, 0.0)
        );
    }

    #[test]
    fn test_remap_f32() {
        assert_eq!(remap(5.0_f32, (0.0, 10.0), (0.0, 1.0)), 0.5);
        assert_eq!(remap(0.0_f32, (0.0, 10.0), (0.0, 1.0)), 0.0);
        assert_eq!(remap(10.0_f32, (0.0, 10.0), (0.0, 1.0)), 1.0);
        assert_eq!(remap(-5.0_f32, (0.0, 10.0), (0.0, 1.0)), -0.5);
    }

    #[test]
    fn test_remap_vec2() {
        assert_eq!(
            remap(
                vec2(5.0, 5.0),
                (vec2(0.0, 0.0), vec2(10.0, 10.0)),
                (vec2(0.0, 0.0), vec2(1.0, 1.0))
            ),
            vec2(0.5, 0.5)
        );
        assert_eq!(
            remap(
                vec2(0.0, 0.0),
                (vec2(0.0, 0.0), vec2(10.0, 10.0)),
                (vec2(0.0, 0.0), vec2(1.0, 1.0))
            ),
            vec2(0.0, 0.0)
        );
        assert_eq!(
            remap(
                vec2(10.0, 10.0),
                (vec2(0.0, 0.0), vec2(10.0, 10.0)),
                (vec2(0.0, 0.0), vec2(1.0, 1.0))
            ),
            vec2(1.0, 1.0)
        );
    }
}
