//! [`EntityDimensions`] — width and height of an entity's axis-aligned hitbox.
//!
//! Used by the physics engine and entity system to compute bounding boxes.

use crate::aabb::Aabb;
use crate::vec3::Vec3;

/// Width and height of an entity's axis-aligned hitbox.
///
/// Matches vanilla `net.minecraft.world.entity.EntityDimensions`.
/// The bounding box is always centered horizontally; the height extends
/// upward from the entity's position.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityDimensions {
    /// The width of the entity (diameter of the hitbox in X and Z).
    pub width: f32,
    /// The height of the entity (hitbox extent in Y).
    pub height: f32,
}

impl EntityDimensions {
    /// Creates new entity dimensions.
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Creates scalable entity dimensions (same as `new` — the `fixed` flag
    /// from vanilla is handled by the entity system, not here).
    pub const fn scalable(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Builds an AABB centered at the given position with these dimensions.
    ///
    /// The box extends `width/2` in X and Z from the position, and `height`
    /// upward from the Y position.
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

    /// Scales both dimensions by a factor.
    pub fn scale(self, factor: f32) -> Self {
        Self {
            width: self.width * factor,
            height: self.height * factor,
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
    }

    #[test]
    fn test_entity_dimensions_scalable() {
        let d = EntityDimensions::scalable(0.6, 1.8);
        assert_eq!(d, EntityDimensions::new(0.6, 1.8));
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
    }

    #[test]
    fn test_entity_dimensions_scale_zero() {
        let d = EntityDimensions::new(1.0, 2.0).scale(0.0);
        assert_eq!(d.width, 0.0);
        assert_eq!(d.height, 0.0);
    }

    // ── Display ─────────────────────────────────────────────────────────

    #[test]
    fn test_entity_dimensions_display() {
        let d = EntityDimensions::new(0.6, 1.8);
        let s = format!("{d}");
        assert!(s.contains("0.6"));
        assert!(s.contains("1.8"));
    }
}
