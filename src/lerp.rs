use std::time::Duration;

use bevy::prelude::*;

pub trait Lerpable: Sized {
    fn lerp(range: (Self, Self), t: f32) -> Self;
}

impl Lerpable for f32 {
    fn lerp(range: (f32, f32), t: f32) -> f32 {
        let (a, b) = range;
        a + (b - a) * t
    }
}

impl Lerpable for f64 {
    fn lerp(range: (f64, f64), t: f32) -> f64 {
        let (a, b) = range;
        a + (b - a) * t as f64
    }
}

impl Lerpable for Vec2 {
    fn lerp(range: (Vec2, Vec2), t: f32) -> Vec2 {
        let (a, b) = range;
        a + (b - a) * t
    }
}

impl Lerpable for Vec3 {
    fn lerp(range: (Vec3, Vec3), t: f32) -> Vec3 {
        let (a, b) = range;
        a + (b - a) * t
    }
}

pub fn lerp<T: Lerpable>(range: (T, T), t: f32) -> T {
    T::lerp(range, t)
}

pub fn smooth_lerp<T: Lerpable>(a: T, b: T, dt: Duration, half_life: Duration) -> T {
    lerp((b, a), (-dt.as_secs_f32() / half_life.as_secs_f32()).exp2())
}

// pub fn remap<T: Lerpable>(value: T, from_range: (T, T), to_range: (T, T)) -> T {
// // Calculate the normalized value within the from_range
// let from_min = from_range.0;
// let from_max = from_range.1;
// let to_min = to_range.0;
// let to_max = to_range.1;

// // Normalize value to a 0-1 range within from_range
// let normalized = T::lerp((from_min, from_max), 0.0); // T::lerp((from_min, from_max), (value - from_min) / (from_max - from_min))

// // Scale the normalized value to the to_range
// T::lerp((to_min, to_max), normalized)
// }

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

    // #[test]
    // fn test_remap_f32() {
    //     assert_eq!(remap(5.0_f32, (0.0, 10.0), (0.0, 1.0)), 0.5);
    //     assert_eq!(remap(0.0_f32, (0.0, 10.0), (0.0, 1.0)), 0.0);
    //     assert_eq!(remap(10.0_f32, (0.0, 10.0), (0.0, 1.0)), 1.0);
    //     assert_eq!(remap(-5.0_f32, (0.0, 10.0), (0.0, 1.0)), -0.5);
    // }

    // #[test]
    // fn test_remap_vec2() {
    //     assert_eq!(
    //         remap(
    //             vec2(5.0, 5.0),
    //             (vec2(0.0, 0.0), vec2(10.0, 10.0)),
    //             (vec2(0.0, 0.0), vec2(1.0, 1.0))
    //         ),
    //         vec2(0.5, 0.5)
    //     );
    //     assert_eq!(
    //         remap(
    //             vec2(0.0, 0.0),
    //             (vec2(0.0, 0.0), vec2(10.0, 10.0)),
    //             (vec2(0.0, 0.0), vec2(1.0, 1.0))
    //         ),
    //         vec2(0.0, 0.0)
    //     );
    //     assert_eq!(
    //         remap(
    //             vec2(10.0, 10.0),
    //             (vec2(0.0, 0.0), vec2(10.0, 10.0)),
    //             (vec2(0.0, 0.0), vec2(1.0, 1.0))
    //         ),
    //         vec2(1.0, 1.0)
    //     );
    // }
}
