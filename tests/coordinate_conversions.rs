//! Integration tests: coordinate type conversions.
//!
//! Verifies that `BlockPos`, `SectionPos`, `Vec3`, `Vec3i`, and `ChunkPos`
//! convert consistently when composed together.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use oxidized_mc_types::chunk_pos::ChunkPosExt;
use oxidized_mc_types::{BlockPos, SectionPos, Vec3, Vec3i};
use oxidized_types::ChunkPos;

#[test]
fn block_pos_to_vec3_roundtrip() {
    let bp = BlockPos::new(10, 64, -30);
    let v = bp.get_center();
    // get_center() returns the center of the block
    assert!((v.x - 10.5).abs() < 1e-10);
    assert!((v.y - 64.5).abs() < 1e-10);
    assert!((v.z - (-29.5)).abs() < 1e-10);
}

#[test]
fn vec3_to_block_pos_containing() {
    let v = Vec3::new(10.7, 64.3, -29.1);
    let bp = BlockPos::containing(v.x, v.y, v.z);
    assert_eq!(bp, BlockPos::new(10, 64, -30));
}

#[test]
fn block_pos_to_section_pos_containment() {
    let bp = BlockPos::new(100, 64, -200);
    let sp = SectionPos::of_block_pos(&bp);
    assert!(bp.x >= sp.min_block_x() && bp.x <= sp.max_block_x());
    assert!(bp.y >= sp.min_block_y() && bp.y <= sp.max_block_y());
    assert!(bp.z >= sp.min_block_z() && bp.z <= sp.max_block_z());
}

#[test]
fn block_pos_to_chunk_pos_containment() {
    let bp = BlockPos::new(33, 64, -17);
    let cp = ChunkPos::from_block_pos(&bp);
    let world = cp.get_world_position();
    assert!(bp.x >= world.x && bp.x < world.x + 16);
    assert!(bp.z >= world.z && bp.z < world.z + 16);
}

#[test]
fn chunk_pos_to_section_pos_agreement() {
    let bp = BlockPos::new(100, 64, -200);
    let cp = ChunkPos::from_block_pos(&bp);
    let sp = SectionPos::of_block_pos(&bp);
    // Chunk X/Z and section X/Z should agree
    assert_eq!(cp.x, sp.x);
    assert_eq!(cp.z, sp.z);
}

#[test]
fn vec3i_from_block_pos_preserves_coords() {
    let bp = BlockPos::new(-50, 128, 300);
    let vi: Vec3i = bp.into();
    assert_eq!(vi.x, bp.x);
    assert_eq!(vi.y, bp.y);
    assert_eq!(vi.z, bp.z);
}

#[test]
fn section_pos_origin_covers_sixteen_blocks() {
    let sp = SectionPos::new(0, 0, 0);
    assert_eq!(sp.min_block_x(), 0);
    assert_eq!(sp.max_block_x(), 15);
    assert_eq!(sp.min_block_y(), 0);
    assert_eq!(sp.max_block_y(), 15);
    assert_eq!(sp.min_block_z(), 0);
    assert_eq!(sp.max_block_z(), 15);
}

#[test]
fn negative_block_pos_section_containment() {
    let bp = BlockPos::new(-1, -1, -1);
    let sp = SectionPos::of_block_pos(&bp);
    // Negative coords round toward negative infinity
    assert_eq!(sp.x, -1);
    assert_eq!(sp.y, -1);
    assert_eq!(sp.z, -1);
    assert!(bp.x >= sp.min_block_x() && bp.x <= sp.max_block_x());
}

#[test]
fn block_pos_packed_through_vec3i() {
    let original = BlockPos::new(1000, -500, 2000);
    let packed = original.as_long();
    let unpacked = BlockPos::from_long(packed);
    let vi: Vec3i = unpacked.into();
    assert_eq!(vi.x, original.x);
    assert_eq!(vi.y, original.y);
    assert_eq!(vi.z, original.z);
}
