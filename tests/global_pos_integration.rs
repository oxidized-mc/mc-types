//! Integration tests for `GlobalPos` cross-module interactions.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;

use oxidized_mc_types::{BlockPos, GlobalPos, ResourceLocation};

#[test]
fn global_pos_as_hashmap_key() {
    let mut map = HashMap::new();
    let overworld = GlobalPos::new(
        ResourceLocation::minecraft("overworld"),
        BlockPos::new(100, 64, -200),
    );
    let nether = GlobalPos::new(
        ResourceLocation::minecraft("the_nether"),
        BlockPos::new(12, 32, -25),
    );

    map.insert(overworld.clone(), "spawn");
    map.insert(nether.clone(), "portal");

    assert_eq!(map[&overworld], "spawn");
    assert_eq!(map[&nether], "portal");
    assert_eq!(map.len(), 2);
}

#[test]
fn global_pos_same_block_different_dimensions() {
    let pos = BlockPos::new(0, 64, 0);
    let overworld = GlobalPos::new(ResourceLocation::minecraft("overworld"), pos);
    let nether = GlobalPos::new(ResourceLocation::minecraft("the_nether"), pos);
    let end = GlobalPos::new(ResourceLocation::minecraft("the_end"), pos);

    assert_ne!(overworld, nether);
    assert_ne!(overworld, end);
    assert_ne!(nether, end);
}

#[test]
fn global_pos_display_includes_dimension_and_coords() {
    let gp = GlobalPos::new(
        ResourceLocation::minecraft("overworld"),
        BlockPos::new(100, 64, -200),
    );
    let display = gp.to_string();
    assert!(display.contains("minecraft:overworld"));
    assert!(display.contains("100"));
    assert!(display.contains("64"));
    assert!(display.contains("-200"));
}

#[test]
fn global_pos_custom_namespace() {
    let dim = ResourceLocation::new("mymod", "custom_dim").unwrap();
    let gp = GlobalPos::new(dim, BlockPos::new(0, 0, 0));
    assert!(gp.to_string().contains("mymod:custom_dim"));
}

#[test]
fn global_pos_block_pos_operations() {
    let gp = GlobalPos::new(
        ResourceLocation::minecraft("overworld"),
        BlockPos::new(10, 64, -30),
    );
    // Access the block pos and perform operations
    let above = gp.pos.above();
    assert_eq!(above.y, 65);

    let as_long = gp.pos.as_long();
    let roundtrip = BlockPos::from_long(as_long);
    assert_eq!(roundtrip, gp.pos);
}
