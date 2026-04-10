//! [`GlobalPos`] — a block position qualified by a dimension.
//!
//! Used for respawn points and lodestone compass targets.

use crate::block_pos::BlockPos;
use crate::resource_location::ResourceLocation;

/// A block position qualified by a dimension (e.g. `minecraft:overworld` + BlockPos).
///
/// Matches vanilla `net.minecraft.core.GlobalPos`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GlobalPos {
    /// The dimension this position belongs to (e.g. `minecraft:overworld`).
    pub dimension: ResourceLocation,
    /// The block coordinates within the dimension.
    pub pos: BlockPos,
}

impl GlobalPos {
    /// Creates a new `GlobalPos`.
    pub fn new(dimension: ResourceLocation, pos: BlockPos) -> Self {
        Self { dimension, pos }
    }
}

impl std::fmt::Display for GlobalPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.dimension, self.pos)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    #[test]
    fn test_global_pos_construction() {
        let dim = ResourceLocation::minecraft("overworld");
        let pos = BlockPos::new(100, 64, -200);
        let gp = GlobalPos::new(dim.clone(), pos);
        assert_eq!(gp.dimension, dim);
        assert_eq!(gp.pos, pos);
    }

    #[test]
    fn test_global_pos_equality() {
        let a = GlobalPos::new(
            ResourceLocation::minecraft("overworld"),
            BlockPos::new(1, 2, 3),
        );
        let b = GlobalPos::new(
            ResourceLocation::minecraft("overworld"),
            BlockPos::new(1, 2, 3),
        );
        assert_eq!(a, b);
    }

    #[test]
    fn test_global_pos_inequality_dimension() {
        let a = GlobalPos::new(
            ResourceLocation::minecraft("overworld"),
            BlockPos::new(1, 2, 3),
        );
        let b = GlobalPos::new(
            ResourceLocation::minecraft("the_nether"),
            BlockPos::new(1, 2, 3),
        );
        assert_ne!(a, b);
    }

    #[test]
    fn test_global_pos_inequality_pos() {
        let a = GlobalPos::new(
            ResourceLocation::minecraft("overworld"),
            BlockPos::new(1, 2, 3),
        );
        let b = GlobalPos::new(
            ResourceLocation::minecraft("overworld"),
            BlockPos::new(4, 5, 6),
        );
        assert_ne!(a, b);
    }

    #[test]
    fn test_global_pos_display() {
        let gp = GlobalPos::new(
            ResourceLocation::minecraft("overworld"),
            BlockPos::new(10, 64, -30),
        );
        let s = format!("{gp}");
        assert!(s.contains("minecraft:overworld"));
    }

    #[test]
    fn test_global_pos_hash_consistent() {
        use std::collections::HashSet;
        let gp1 = GlobalPos::new(
            ResourceLocation::minecraft("overworld"),
            BlockPos::new(1, 2, 3),
        );
        let gp2 = GlobalPos::new(
            ResourceLocation::minecraft("overworld"),
            BlockPos::new(1, 2, 3),
        );
        let mut set = HashSet::new();
        set.insert(gp1);
        assert!(set.contains(&gp2));
    }
}
