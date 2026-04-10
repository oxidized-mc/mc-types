//! [`EntityDimensions`] — width and height of an entity's axis-aligned hitbox.
//!
//! Used by the physics engine and entity system to compute bounding boxes.

use crate::aabb::Aabb;
use crate::vec3::Vec3;

/// The default eye-height ratio when none is explicitly provided.
///
/// Vanilla uses `height * 0.85` as the default eye position.
const DEFAULT_EYE_HEIGHT_RATIO: f32 = 0.85;

/// Width and height of an entity's axis-aligned hitbox.
///
/// Matches vanilla `net.minecraft.world.entity.EntityDimensions`.
/// The bounding box is always centered horizontally; the height extends
/// upward from the entity's position.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::{EntityDimensions, Vec3};
///
/// let dims = EntityDimensions::new(0.6, 1.8);
/// let bb = dims.make_bounding_box(Vec3::ZERO);
/// assert!((bb.y_size() - 1.8) < 0.01);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityDimensions {
    /// The width of the entity (diameter of the hitbox in X and Z).
    pub width: f32,
    /// The height of the entity (hitbox extent in Y).
    pub height: f32,
    /// The vertical eye position within the hitbox.
    pub eye_height: f32,
    /// Whether the dimensions are fixed (unaffected by scaling).
    pub fixed: bool,
}

impl EntityDimensions {
    /// Creates new scalable entity dimensions with a default eye height.
    #[inline]
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            eye_height: height * DEFAULT_EYE_HEIGHT_RATIO,
            fixed: false,
        }
    }

    /// Creates scalable entity dimensions with a default eye height.
    #[inline]
    pub fn scalable(width: f32, height: f32) -> Self {
        Self::new(width, height)
    }

    /// Creates fixed entity dimensions that ignore scaling.
    #[inline]
    pub fn fixed(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            eye_height: height * DEFAULT_EYE_HEIGHT_RATIO,
            fixed: true,
        }
    }

    /// Returns a copy with the given eye height.
    #[inline]
    pub fn with_eye_height(self, eye_height: f32) -> Self {
        Self { eye_height, ..self }
    }

    /// Builds an AABB centered at the given position with these dimensions.
    ///
    /// The box extends `width/2` in X and Z from the position, and `height`
    /// upward from the Y position.
    #[inline]
    pub fn make_bounding_box(&self, pos: Vec3) -> Aabb {
        let half_w = f64::from(self.width) / 2.0;
        let h = f64::from(self.height);
        Aabb::new(
            pos.x - half_w,
            pos.y,
            pos.z - half_w,
            pos.x + half_w,
            pos.y + h,
            pos.z + half_w,
        )
    }

    /// Scales both dimensions by a factor. Fixed dimensions are unaffected.
    #[inline]
    pub fn scale(self, factor: f32) -> Self {
        if self.fixed {
            return self;
        }
        Self {
            width: self.width * factor,
            height: self.height * factor,
            eye_height: self.eye_height * factor,
            fixed: false,
        }
    }
}

impl std::fmt::Display for EntityDimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}×{}", self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    // ── Construction ────────────────────────────────────────────────────

    #[test]
    fn test_entity_dimensions_new() {
        let d = EntityDimensions::new(0.6, 1.8);
        assert!((d.width - 0.6).abs() < 1e-6);
        assert!((d.height - 1.8).abs() < 1e-6);
        assert!((d.eye_height - 1.8 * 0.85).abs() < 1e-6);
        assert!(!d.fixed);
    }

    #[test]
    fn test_entity_dimensions_scalable() {
        let d = EntityDimensions::scalable(0.6, 1.8);
        assert_eq!(d, EntityDimensions::new(0.6, 1.8));
    }

    #[test]
    fn test_entity_dimensions_fixed() {
        let d = EntityDimensions::fixed(0.6, 1.8);
        assert!((d.width - 0.6).abs() < 1e-6);
        assert!((d.height - 1.8).abs() < 1e-6);
        assert!(d.fixed);
    }

    #[test]
    fn test_entity_dimensions_with_eye_height() {
        let d = EntityDimensions::new(0.6, 1.8).with_eye_height(1.62);
        assert!((d.eye_height - 1.62).abs() < 1e-6);
        assert!((d.width - 0.6).abs() < 1e-6);
    }

    // ── make_bounding_box ───────────────────────────────────────────────

    #[test]
    fn test_entity_dimensions_make_bounding_box_at_origin() {
        let d = EntityDimensions::new(1.0, 2.0);
        let bb = d.make_bounding_box(Vec3::ZERO);
        assert!((bb.min_x - (-0.5)).abs() < 1e-10);
        assert!((bb.min_y - 0.0).abs() < 1e-10);
        assert!((bb.min_z - (-0.5)).abs() < 1e-10);
        assert!((bb.max_x - 0.5).abs() < 1e-10);
        assert!((bb.max_y - 2.0).abs() < 1e-10);
        assert!((bb.max_z - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_entity_dimensions_make_bounding_box_offset() {
        let d = EntityDimensions::new(0.6, 1.8);
        let pos = Vec3::new(10.0, 64.0, -30.0);
        let bb = d.make_bounding_box(pos);
        let half_w = f64::from(0.6_f32) / 2.0;
        assert!((bb.min_x - (10.0 - half_w)).abs() < 1e-10);
        assert!((bb.max_x - (10.0 + half_w)).abs() < 1e-10);
        assert!((bb.min_y - 64.0).abs() < 1e-10);
        assert!((bb.max_y - (64.0 + f64::from(1.8_f32))).abs() < 1e-6);
        assert!((bb.min_z - (-30.0 - half_w)).abs() < 1e-10);
        assert!((bb.max_z - (-30.0 + half_w)).abs() < 1e-10);
    }

    // ── scale ───────────────────────────────────────────────────────────

    #[test]
    fn test_entity_dimensions_scale() {
        let d = EntityDimensions::new(1.0, 2.0).scale(2.0);
        assert!((d.width - 2.0).abs() < 1e-6);
        assert!((d.height - 4.0).abs() < 1e-6);
        assert!((d.eye_height - 2.0 * 0.85 * 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_entity_dimensions_scale_zero() {
        let d = EntityDimensions::new(1.0, 2.0).scale(0.0);
        assert_eq!(d.width, 0.0);
        assert_eq!(d.height, 0.0);
        assert_eq!(d.eye_height, 0.0);
    }

    #[test]
    fn test_entity_dimensions_fixed_ignores_scale() {
        let d = EntityDimensions::fixed(1.0, 2.0);
        let scaled = d.scale(3.0);
        assert_eq!(scaled, d);
    }

    // ── Display ─────────────────────────────────────────────────────────

    #[test]
    fn test_entity_dimensions_display() {
        let d = EntityDimensions::new(0.6, 1.8);
        let s = format!("{d}");
        assert!(s.contains("0.6"));
        assert!(s.contains("1.8"));
    }

    // ── Snapshot tests ──────────────────────────────────────────────

    mod snapshots {
        use super::*;

        #[test]
        fn snapshot_entity_dimensions_display() {
            insta::assert_snapshot!(
                EntityDimensions::new(0.6, 1.8).to_string(),
                @"0.6×1.8"
            );
        }
    }

    // ── Property-based tests ────────────────────────────────────────

    mod prop {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn entity_dimensions_scale_positive(
                w in 0.1f32..100.0,
                h in 0.1f32..100.0,
                factor in 0.01f32..10.0,
            ) {
                let d = EntityDimensions::new(w, h).scale(factor);
                prop_assert!(d.width > 0.0);
                prop_assert!(d.height > 0.0);
            }

            #[test]
            fn entity_dimensions_bounding_box_contains_origin(
                w in 0.1f32..10.0,
                h in 0.1f32..10.0,
            ) {
                let d = EntityDimensions::new(w, h);
                let bb = d.make_bounding_box(Vec3::ZERO);
                // Center of bottom face is at origin
                prop_assert!(bb.contains(0.0, 0.0, 0.0));
            }
        }
    }
}
