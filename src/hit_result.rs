//! [`HitResultType`] and [`BlockHitResult`] — raycasting result types.
//!
//! Used by both server and client for block/entity targeting.

use crate::block_pos::BlockPos;
use crate::direction::Direction;
use crate::vec3::Vec3;

/// Discriminant for hit result types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HitResultType {
    /// The ray missed all targets.
    Miss,
    /// The ray hit a block.
    Block,
    /// The ray hit an entity.
    Entity,
}

/// Result of a block raycast.
///
/// Contains the precise hit location, the face that was hit, the block
/// position, and whether the hit originated from inside the block.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockHitResult {
    /// The exact world-space position where the ray intersected.
    pub location: Vec3,
    /// The block face that was hit.
    pub direction: Direction,
    /// The block position that was hit.
    pub block_pos: BlockPos,
    /// Whether this represents a miss.
    pub miss: bool,
    /// Whether the ray started inside the block.
    pub inside: bool,
}

impl BlockHitResult {
    /// Creates a miss result at the given location.
    pub fn miss(location: Vec3, direction: Direction, block_pos: BlockPos) -> Self {
        Self {
            location,
            direction,
            block_pos,
            miss: true,
            inside: false,
        }
    }

    /// Creates a hit result at the given location.
    pub fn hit(location: Vec3, direction: Direction, block_pos: BlockPos) -> Self {
        Self {
            location,
            direction,
            block_pos,
            miss: false,
            inside: false,
        }
    }

    /// Creates a hit result at the given location, originating from inside the block.
    pub fn hit_inside(location: Vec3, direction: Direction, block_pos: BlockPos) -> Self {
        Self {
            location,
            direction,
            block_pos,
            miss: false,
            inside: true,
        }
    }

    /// Whether this result represents a miss.
    pub fn is_miss(&self) -> bool {
        self.miss
    }

    /// Returns the hit result type discriminant.
    pub fn get_type(&self) -> HitResultType {
        if self.miss {
            HitResultType::Miss
        } else {
            HitResultType::Block
        }
    }

    /// Returns a copy with the direction replaced.
    pub fn with_direction(self, direction: Direction) -> Self {
        Self { direction, ..self }
    }

    /// Returns a copy with the block position replaced.
    pub fn with_position(self, pos: BlockPos) -> Self {
        Self {
            block_pos: pos,
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn sample_location() -> Vec3 {
        Vec3::new(1.5, 2.5, 3.5)
    }

    // ── Construction ────────────────────────────────────────────────────

    #[test]
    fn test_block_hit_result_miss() {
        let r = BlockHitResult::miss(sample_location(), Direction::Up, BlockPos::new(1, 2, 3));
        assert!(r.is_miss());
        assert_eq!(r.get_type(), HitResultType::Miss);
        assert!(!r.inside);
    }

    #[test]
    fn test_block_hit_result_hit() {
        let r = BlockHitResult::hit(sample_location(), Direction::North, BlockPos::new(1, 2, 3));
        assert!(!r.is_miss());
        assert_eq!(r.get_type(), HitResultType::Block);
        assert!(!r.inside);
    }

    #[test]
    fn test_block_hit_result_hit_inside() {
        let r =
            BlockHitResult::hit_inside(sample_location(), Direction::East, BlockPos::new(1, 2, 3));
        assert!(!r.is_miss());
        assert_eq!(r.get_type(), HitResultType::Block);
        assert!(r.inside);
    }

    // ── Modifiers ───────────────────────────────────────────────────────

    #[test]
    fn test_block_hit_result_with_direction() {
        let r = BlockHitResult::hit(sample_location(), Direction::Up, BlockPos::new(1, 2, 3));
        let r2 = r.with_direction(Direction::Down);
        assert_eq!(r2.direction, Direction::Down);
        assert_eq!(r2.location, r.location);
        assert_eq!(r2.block_pos, r.block_pos);
    }

    #[test]
    fn test_block_hit_result_with_position() {
        let r = BlockHitResult::hit(sample_location(), Direction::Up, BlockPos::new(1, 2, 3));
        let new_pos = BlockPos::new(10, 20, 30);
        let r2 = r.with_position(new_pos);
        assert_eq!(r2.block_pos, new_pos);
        assert_eq!(r2.location, r.location);
        assert_eq!(r2.direction, r.direction);
    }

    // ── HitResultType ───────────────────────────────────────────────────

    #[test]
    fn test_hit_result_type_values() {
        assert_ne!(HitResultType::Miss, HitResultType::Block);
        assert_ne!(HitResultType::Block, HitResultType::Entity);
        assert_ne!(HitResultType::Miss, HitResultType::Entity);
    }
}
