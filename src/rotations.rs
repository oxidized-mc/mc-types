//! [`Rotations`] — three-axis rotation in degrees.
//!
//! Used for armor stand body-part orientations. Each component is reduced
//! modulo 360 on construction (preserving sign, matching Java's `%` operator);
//! NaN and infinite values are clamped to zero.

use bytes::{Bytes, BytesMut};

use oxidized_codec::types::{self, TypeError};

/// Three-axis rotation in degrees (pitch, yaw, roll).
///
/// Matches vanilla `net.minecraft.core.Rotations`. Components are reduced
/// modulo 360 on construction (preserving sign, like Java's `%` operator),
/// and NaN/infinite inputs become `0.0`.
///
/// # Wire format
///
/// Three consecutive big-endian `f32` values (12 bytes total).
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::Rotations;
///
/// let r = Rotations::new(45.0, 90.0, 0.0);
/// assert_eq!(r.x, 45.0);
///
/// // Values are reduced mod 360, preserving sign
/// let wrapped = Rotations::new(370.0, -10.0, 0.0);
/// assert!((wrapped.x - 10.0).abs() < 1e-6);
/// assert!((wrapped.y - (-10.0)).abs() < 1e-6);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rotations {
    /// Rotation around the X axis (pitch) in degrees.
    pub x: f32,
    /// Rotation around the Y axis (yaw) in degrees.
    pub y: f32,
    /// Rotation around the Z axis (roll) in degrees.
    pub z: f32,
}

/// Sanitize a single rotation component: NaN/infinite → 0, then reduce mod 360
/// (preserving sign, matching Java's `%` operator).
fn sanitize(v: f32) -> f32 {
    if v.is_finite() { v % 360.0 } else { 0.0 }
}

impl Rotations {
    /// All-zero rotation.
    pub const ZERO: Rotations = Rotations {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    /// Creates a new `Rotations`, sanitising each component.
    ///
    /// NaN and infinite values become `0.0`; all values are reduced mod 360
    /// (preserving sign).
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: sanitize(x),
            y: sanitize(y),
            z: sanitize(z),
        }
    }

    /// Reads a `Rotations` from a wire buffer (3 × big-endian `f32`).
    ///
    /// # Errors
    ///
    /// Returns [`TypeError`] if fewer than 12 bytes remain.
    pub fn read(buf: &mut Bytes) -> Result<Self, TypeError> {
        let x = types::read_f32(buf)?;
        let y = types::read_f32(buf)?;
        let z = types::read_f32(buf)?;
        Ok(Self::new(x, y, z))
    }

    /// Writes this `Rotations` to a wire buffer (3 × big-endian `f32`).
    pub fn write(&self, buf: &mut BytesMut) {
        types::write_f32(buf, self.x);
        types::write_f32(buf, self.y);
        types::write_f32(buf, self.z);
    }
}

impl std::fmt::Display for Rotations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use bytes::{Bytes, BytesMut};

    use super::*;

    // ── Construction ────────────────────────────────────────────────────

    #[test]
    fn test_rotations_zero_constant() {
        assert_eq!(
            Rotations::ZERO,
            Rotations {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        );
    }

    #[test]
    fn test_rotations_new_normalises_values() {
        let r = Rotations::new(370.0, -10.0, 720.0);
        assert!((r.x - 10.0).abs() < 1e-4);
        // Vanilla Java % preserves sign: -10.0 % 360.0 == -10.0
        assert!((r.y - (-10.0)).abs() < 1e-4);
        assert!(r.z.abs() < 1e-4);
    }

    #[test]
    fn test_rotations_new_nan_becomes_zero() {
        let r = Rotations::new(f32::NAN, 90.0, f32::NAN);
        assert_eq!(r.x, 0.0);
        assert!((r.y - 90.0).abs() < 1e-4);
        assert_eq!(r.z, 0.0);
    }

    #[test]
    fn test_rotations_new_infinity_becomes_zero() {
        let r = Rotations::new(f32::INFINITY, f32::NEG_INFINITY, 45.0);
        assert_eq!(r.x, 0.0);
        assert_eq!(r.y, 0.0);
        assert!((r.z - 45.0).abs() < 1e-4);
    }

    // ── Wire roundtrip ─────────────────────────────────────────────────

    #[test]
    fn test_rotations_wire_roundtrip() {
        let original = Rotations::new(45.0, 90.0, 180.0);
        let mut buf = BytesMut::new();
        original.write(&mut buf);
        assert_eq!(buf.len(), 12);
        let mut read_buf = Bytes::from(buf);
        let decoded = Rotations::read(&mut read_buf).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_rotations_read_truncated_buffer() {
        let mut buf = Bytes::from_static(&[0u8; 8]); // only 8 bytes, need 12
        assert!(Rotations::read(&mut buf).is_err());
    }

    // ── Display ────────────────────────────────────────────────────────

    #[test]
    fn test_rotations_display() {
        let r = Rotations::new(10.0, 20.0, 30.0);
        let s = format!("{r}");
        assert!(s.contains("10"));
        assert!(s.contains("20"));
        assert!(s.contains("30"));
    }

    // ── Property-based tests ────────────────────────────────────────

    mod prop {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn rotations_wire_roundtrip(
                x in -1000.0f32..1000.0,
                y in -1000.0f32..1000.0,
                z in -1000.0f32..1000.0,
            ) {
                let r = Rotations::new(x, y, z);
                let mut buf = BytesMut::new();
                r.write(&mut buf);
                let mut data = Bytes::from(buf);
                let decoded = Rotations::read(&mut data).unwrap();
                prop_assert_eq!(decoded, r);
            }

            #[test]
            fn rotations_new_is_idempotent(
                x in -1000.0f32..1000.0,
                y in -1000.0f32..1000.0,
                z in -1000.0f32..1000.0,
            ) {
                let r = Rotations::new(x, y, z);
                let r2 = Rotations::new(r.x, r.y, r.z);
                prop_assert_eq!(r, r2);
            }

            #[test]
            fn rotations_sanitize_range(
                x in -1000.0f32..1000.0,
                y in -1000.0f32..1000.0,
                z in -1000.0f32..1000.0,
            ) {
                let r = Rotations::new(x, y, z);
                // Java % preserves sign: result is in (-360, 360)
                prop_assert!(r.x > -360.0 && r.x < 360.0);
                prop_assert!(r.y > -360.0 && r.y < 360.0);
                prop_assert!(r.z > -360.0 && r.z < 360.0);
            }
        }
    }

    // ── Snapshot tests ──────────────────────────────────────────────

    mod snapshots {
        use super::*;

        #[test]
        fn snapshot_rotations_display() {
            insta::assert_snapshot!(
                Rotations::new(10.0, 20.0, 30.0).to_string(),
                @"(10, 20, 30)"
            );
        }
    }
}
