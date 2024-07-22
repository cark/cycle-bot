use std::time::Duration;

use bevy::prelude::*;

pub trait Lerpable: Sized {
    fn lerp(a: Self, b: Self, t: f32) -> Self;
}

impl Lerpable for f32 {
    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }
}

impl Lerpable for f64 {
    fn lerp(a: f64, b: f64, t: f32) -> f64 {
        a + (b - a) * t as f64
    }
}

impl Lerpable for Vec2 {
    fn lerp(a: Vec2, b: Vec2, t: f32) -> Vec2 {
        a + (b - a) * t
    }
}

impl Lerpable for Vec3 {
    fn lerp(a: Vec3, b: Vec3, t: f32) -> Vec3 {
        a + (b - a) * t
    }
}

pub fn lerp<T: Lerpable>(a: T, b: T, t: f32) -> T {
    T::lerp(a, b, t)
}

pub fn smooth_lerp<T: Lerpable>(a: T, b: T, dt: Duration, half_life: Duration) -> T {
    lerp(b, a, (-dt.as_secs_f32() / half_life.as_secs_f32()).exp2())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::math::vec2;
    use bevy::math::vec3;

    #[test]
    fn test_lerp_f32() {
        assert_eq!(lerp(0.0_f32, 10.0_f32, 0.5), 5.0);
        assert_eq!(lerp(0.0_f32, 10.0_f32, 0.0), 0.0);
        assert_eq!(lerp(0.0_f32, 10.0_f32, 1.0), 10.0);
        assert_eq!(lerp(-5.0_f32, 5.0_f32, 0.5), 0.0);
    }

    #[test]
    fn test_lerp_f64() {
        assert_eq!(lerp(0.0_f64, 10.0_f64, 0.5), 5.0);
        assert_eq!(lerp(0.0_f64, 10.0_f64, 0.0), 0.0);
        assert_eq!(lerp(0.0_f64, 10.0_f64, 1.0), 10.0);
        assert_eq!(lerp(-5.0_f64, 5.0_f64, 0.5), 0.0);
    }

    #[test]
    fn test_lerp_vec2() {
        assert_eq!(lerp(vec2(0.0, 0.0), vec2(10.0, 10.0), 0.5), vec2(5.0, 5.0));
        assert_eq!(lerp(vec2(0.0, 0.0), vec2(10.0, 10.0), 0.0), vec2(0.0, 0.0));
        assert_eq!(
            lerp(vec2(0.0, 0.0), vec2(10.0, 10.0), 1.0),
            vec2(10.0, 10.0)
        );
        assert_eq!(lerp(vec2(-5.0, -5.0), vec2(5.0, 5.0), 0.5), vec2(0.0, 0.0));
    }

    #[test]
    fn test_lerp_vec3() {
        assert_eq!(
            lerp(vec3(0.0, 0.0, 0.0), vec3(10.0, 10.0, 10.0), 0.5),
            vec3(5.0, 5.0, 5.0)
        );
        assert_eq!(
            lerp(vec3(0.0, 0.0, 0.0), vec3(10.0, 10.0, 10.0), 0.0),
            vec3(0.0, 0.0, 0.0)
        );
        assert_eq!(
            lerp(vec3(0.0, 0.0, 0.0), vec3(10.0, 10.0, 10.0), 1.0),
            vec3(10.0, 10.0, 10.0)
        );
        assert_eq!(
            lerp(vec3(-5.0, -5.0, -5.0), vec3(5.0, 5.0, 5.0), 0.5),
            vec3(0.0, 0.0, 0.0)
        );
    }
}
