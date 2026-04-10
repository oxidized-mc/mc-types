//! Minecraft math utilities matching vanilla's `Mth` class.
//!
//! These are exact ports of the commonly used functions from
//! `net.minecraft.util.Mth`, preserving Java's edge-case behaviour.

/// Floors a `f64` to `i32`, matching Java `Mth.floor(double)`.
///
/// Subtracts 1 from the cast result when the value has a fractional part and
/// is negative, because Rust/C truncation rounds toward zero rather than
/// toward negative infinity. Non-finite inputs (NaN, ±infinity) return the
/// saturating cast to avoid overflow panics.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::mth;
///
/// assert_eq!(mth::floor(5.7), 5);
/// assert_eq!(mth::floor(-3.1), -4);
/// assert_eq!(mth::floor(0.0), 0);
/// ```
#[inline]
pub fn floor(value: f64) -> i32 {
    let i = value as i32;
    if !value.is_finite() {
        return i;
    }
    if value < i as f64 { i - 1 } else { i }
}

/// Ceils a `f64` to `i32`, matching Java `Mth.ceil(double)`.
///
/// Non-finite inputs return the saturating cast.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::mth;
///
/// assert_eq!(mth::ceil(5.1), 6);
/// assert_eq!(mth::ceil(-3.7), -3);
/// assert_eq!(mth::ceil(5.0), 5);
/// ```
#[inline]
pub fn ceil(value: f64) -> i32 {
    let i = value as i32;
    if !value.is_finite() {
        return i;
    }
    if value > i as f64 { i + 1 } else { i }
}

/// Floors a `f32` to `i32`, matching Java `Mth.floor(float)`.
///
/// Non-finite inputs return the saturating cast.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::mth;
///
/// assert_eq!(mth::floor_f(5.7), 5);
/// assert_eq!(mth::floor_f(-3.1), -4);
/// ```
#[inline]
pub fn floor_f(value: f32) -> i32 {
    let i = value as i32;
    if !value.is_finite() {
        return i;
    }
    if value < i as f32 { i - 1 } else { i }
}

/// Ceils a `f32` to `i32`, matching Java `Mth.ceil(float)`.
///
/// Non-finite inputs return the saturating cast.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::mth;
///
/// assert_eq!(mth::ceil_f(5.1), 6);
/// assert_eq!(mth::ceil_f(-3.7), -3);
/// ```
#[inline]
pub fn ceil_f(value: f32) -> i32 {
    let i = value as i32;
    if !value.is_finite() {
        return i;
    }
    if value > i as f32 { i + 1 } else { i }
}

/// Clamps a `f64` value between `min` and `max`.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::mth;
///
/// assert_eq!(mth::clamp(5.0, 0.0, 10.0), 5.0);
/// assert_eq!(mth::clamp(-1.0, 0.0, 10.0), 0.0);
/// assert_eq!(mth::clamp(15.0, 0.0, 10.0), 10.0);
/// ```
#[inline]
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Clamps an `i32` value between `min` and `max`.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::mth;
///
/// assert_eq!(mth::clamp_i32(5, 0, 10), 5);
/// assert_eq!(mth::clamp_i32(-5, 0, 10), 0);
/// assert_eq!(mth::clamp_i32(15, 0, 10), 10);
/// ```
#[inline]
pub fn clamp_i32(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Linear interpolation between `start` and `end` by `delta`.
///
/// `delta = 0.0` → `start`, `delta = 1.0` → `end`.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::mth;
///
/// assert_eq!(mth::lerp(0.0, 10.0, 20.0), 10.0);
/// assert_eq!(mth::lerp(1.0, 10.0, 20.0), 20.0);
/// assert!((mth::lerp(0.5, 10.0, 20.0) - 15.0).abs() < 1e-10);
/// ```
#[inline]
pub fn lerp(delta: f64, start: f64, end: f64) -> f64 {
    start + delta * (end - start)
}

/// Returns a non-negative modulo result, matching Java `Mth.positiveModulo(int, int)`.
///
/// The result is always in `[0, y)` for positive `y`.
///
/// # Panics
///
/// Panics if `y` is zero (division by zero).
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::mth;
///
/// assert_eq!(mth::positive_modulo(7, 3), 1);
/// assert_eq!(mth::positive_modulo(-1, 16), 15);
/// assert_eq!(mth::positive_modulo(0, 5), 0);
/// ```
#[inline]
pub fn positive_modulo(x: i32, y: i32) -> i32 {
    let r = x % y;
    if r < 0 { r + y } else { r }
}

/// Wraps an angle in degrees to the range `[-180, 180)`.
///
/// Matches Java `Mth.wrapDegrees(double)`.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::mth;
///
/// assert!((mth::wrapping_degrees(270.0) - (-90.0)).abs() < 1e-10);
/// assert!((mth::wrapping_degrees(-270.0) - 90.0).abs() < 1e-10);
/// ```
#[inline]
pub fn wrapping_degrees(degrees: f64) -> f64 {
    let mut d = degrees % 360.0;
    if d >= 180.0 {
        d -= 360.0;
    }
    if d < -180.0 {
        d += 360.0;
    }
    d
}

/// Converts degrees to radians.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::mth;
///
/// let r = mth::deg_to_rad(90.0);
/// assert!((r - std::f32::consts::FRAC_PI_2).abs() < 1e-6);
/// ```
#[inline]
pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * (std::f32::consts::PI / 180.0)
}

/// Converts radians to degrees.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::mth;
///
/// let d = mth::rad_to_deg(std::f32::consts::PI);
/// assert!((d - 180.0).abs() < 1e-3);
/// ```
#[inline]
pub fn rad_to_deg(radians: f32) -> f32 {
    radians * (180.0 / std::f32::consts::PI)
}

/// Square root of a `f32`, matching Java `Mth.sqrt(float)`.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::mth;
///
/// assert!((mth::sqrt(9.0) - 3.0).abs() < 1e-6);
/// assert_eq!(mth::sqrt(0.0), 0.0);
/// ```
#[inline]
pub fn sqrt(value: f32) -> f32 {
    value.sqrt()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    // ── floor / ceil ────────────────────────────────────────────────────

    #[test]
    fn test_floor_positive_integer() {
        assert_eq!(floor(5.0), 5);
    }

    #[test]
    fn test_floor_positive_fraction() {
        assert_eq!(floor(5.7), 5);
    }

    #[test]
    fn test_floor_negative_integer() {
        assert_eq!(floor(-3.0), -3);
    }

    #[test]
    fn test_floor_negative_fraction() {
        assert_eq!(floor(-3.1), -4);
    }

    #[test]
    fn test_floor_zero() {
        assert_eq!(floor(0.0), 0);
    }

    #[test]
    fn test_ceil_positive_integer() {
        assert_eq!(ceil(5.0), 5);
    }

    #[test]
    fn test_ceil_positive_fraction() {
        assert_eq!(ceil(5.1), 6);
    }

    #[test]
    fn test_ceil_negative_integer() {
        assert_eq!(ceil(-3.0), -3);
    }

    #[test]
    fn test_ceil_negative_fraction() {
        assert_eq!(ceil(-3.7), -3);
    }

    #[test]
    fn test_floor_f_positive_fraction() {
        assert_eq!(floor_f(5.7), 5);
    }

    #[test]
    fn test_floor_f_negative_fraction() {
        assert_eq!(floor_f(-3.1), -4);
    }

    #[test]
    fn test_ceil_f_positive_fraction() {
        assert_eq!(ceil_f(5.1), 6);
    }

    #[test]
    fn test_ceil_f_negative_fraction() {
        assert_eq!(ceil_f(-3.7), -3);
    }

    // Non-finite inputs must not panic (saturating cast to i32 bounds)
    #[test]
    fn test_floor_negative_infinity_no_panic() {
        let _ = floor(f64::NEG_INFINITY);
    }

    #[test]
    fn test_ceil_positive_infinity_no_panic() {
        let _ = ceil(f64::INFINITY);
    }

    #[test]
    fn test_floor_nan() {
        assert_eq!(floor(f64::NAN), 0);
    }

    #[test]
    fn test_ceil_nan() {
        assert_eq!(ceil(f64::NAN), 0);
    }

    #[test]
    fn test_floor_f_negative_infinity_no_panic() {
        let _ = floor_f(f32::NEG_INFINITY);
    }

    #[test]
    fn test_ceil_f_positive_infinity_no_panic() {
        let _ = ceil_f(f32::INFINITY);
    }

    // ── clamp ───────────────────────────────────────────────────────────

    #[test]
    fn test_clamp_in_range() {
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
    }

    #[test]
    fn test_clamp_below_min() {
        assert_eq!(clamp(-1.0, 0.0, 10.0), 0.0);
    }

    #[test]
    fn test_clamp_above_max() {
        assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
    }

    #[test]
    fn test_clamp_i32_in_range() {
        assert_eq!(clamp_i32(5, 0, 10), 5);
    }

    #[test]
    fn test_clamp_i32_below_min() {
        assert_eq!(clamp_i32(-5, 0, 10), 0);
    }

    #[test]
    fn test_clamp_i32_above_max() {
        assert_eq!(clamp_i32(15, 0, 10), 10);
    }

    // ── lerp ────────────────────────────────────────────────────────────

    #[test]
    fn test_lerp_zero() {
        assert_eq!(lerp(0.0, 10.0, 20.0), 10.0);
    }

    #[test]
    fn test_lerp_one() {
        assert_eq!(lerp(1.0, 10.0, 20.0), 20.0);
    }

    #[test]
    fn test_lerp_half() {
        assert!((lerp(0.5, 10.0, 20.0) - 15.0).abs() < 1e-10);
    }

    // ── positive_modulo ─────────────────────────────────────────────────

    #[test]
    fn test_positive_modulo_positive() {
        assert_eq!(positive_modulo(7, 3), 1);
    }

    #[test]
    fn test_positive_modulo_negative() {
        assert_eq!(positive_modulo(-1, 16), 15);
    }

    #[test]
    fn test_positive_modulo_zero() {
        assert_eq!(positive_modulo(0, 5), 0);
    }

    #[test]
    fn test_positive_modulo_exact_multiple() {
        assert_eq!(positive_modulo(16, 16), 0);
    }

    // ── wrapping_degrees ────────────────────────────────────────────────

    #[test]
    fn test_wrapping_degrees_in_range() {
        assert!((wrapping_degrees(90.0) - 90.0).abs() < 1e-10);
    }

    #[test]
    fn test_wrapping_degrees_above_180() {
        assert!((wrapping_degrees(270.0) - (-90.0)).abs() < 1e-10);
    }

    #[test]
    fn test_wrapping_degrees_below_neg180() {
        assert!((wrapping_degrees(-270.0) - 90.0).abs() < 1e-10);
    }

    #[test]
    fn test_wrapping_degrees_360() {
        assert!(wrapping_degrees(360.0).abs() < 1e-10);
    }

    #[test]
    fn test_wrapping_degrees_neg180_boundary() {
        // -180 should stay as -180 (inclusive lower bound)
        assert!((wrapping_degrees(-180.0) - (-180.0)).abs() < 1e-10);
    }

    // ── deg / rad ───────────────────────────────────────────────────────

    #[test]
    fn test_deg_to_rad_90() {
        let result = deg_to_rad(90.0);
        assert!((result - std::f32::consts::FRAC_PI_2).abs() < 1e-6);
    }

    #[test]
    fn test_rad_to_deg_pi() {
        let result = rad_to_deg(std::f32::consts::PI);
        assert!((result - 180.0).abs() < 1e-3);
    }

    #[test]
    fn test_deg_rad_roundtrip() {
        let deg = 45.0_f32;
        let result = rad_to_deg(deg_to_rad(deg));
        assert!((result - deg).abs() < 1e-4);
    }

    // ── sqrt ────────────────────────────────────────────────────────────

    #[test]
    fn test_sqrt_positive() {
        assert!((sqrt(9.0) - 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_sqrt_zero() {
        assert_eq!(sqrt(0.0), 0.0);
    }

    #[test]
    fn test_sqrt_one() {
        assert!((sqrt(1.0) - 1.0).abs() < 1e-6);
    }
}
