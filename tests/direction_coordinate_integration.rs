//! Integration tests: Direction ↔ coordinate type interactions.
//!
//! Verifies that Direction offsets work correctly with BlockPos, Vec3, and Vec3i.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use oxidized_mc_types::direction::{self, Axis};
use oxidized_mc_types::{BlockPos, Direction, Vec3i};

#[test]
fn direction_offset_on_block_pos() {
    let origin = BlockPos::new(0, 64, 0);
    assert_eq!(origin.above(), BlockPos::new(0, 65, 0));
    assert_eq!(origin.below(), BlockPos::new(0, 63, 0));
    assert_eq!(origin.north(), BlockPos::new(0, 64, -1));
    assert_eq!(origin.south(), BlockPos::new(0, 64, 1));
    assert_eq!(origin.east(), BlockPos::new(1, 64, 0));
    assert_eq!(origin.west(), BlockPos::new(-1, 64, 0));
}

#[test]
fn direction_offset_on_vec3i() {
    let origin = Vec3i::new(0, 64, 0);
    assert_eq!(origin.above(), Vec3i::new(0, 65, 0));
    assert_eq!(origin.below(), Vec3i::new(0, 63, 0));
    assert_eq!(origin.north(), Vec3i::new(0, 64, -1));
    assert_eq!(origin.south(), Vec3i::new(0, 64, 1));
}

#[test]
fn all_directions_are_unit_length() {
    for dir in &direction::ALL {
        let step_sum = dir.step_x().abs() + dir.step_y().abs() + dir.step_z().abs();
        assert_eq!(
            step_sum, 1,
            "Direction {dir:?} should have unit length step"
        );
    }
}

#[test]
fn opposite_directions_cancel_out() {
    let bp = BlockPos::new(10, 64, -30);
    for dir in &direction::ALL {
        let moved = bp.relative_steps(*dir, 5);
        let back = moved.relative_steps(dir.opposite(), 5);
        assert_eq!(
            back, bp,
            "Moving {dir:?} then opposite should return to start"
        );
    }
}

#[test]
fn horizontal_directions_preserve_y() {
    let bp = BlockPos::new(10, 64, -30);
    for dir in &direction::HORIZONTALS {
        let moved = bp.relative_steps(*dir, 3);
        assert_eq!(
            moved.y, bp.y,
            "Horizontal direction {dir:?} should preserve Y"
        );
    }
}

#[test]
fn direction_step_matches_axis() {
    for dir in &direction::ALL {
        match dir.axis() {
            Axis::X => {
                assert_ne!(dir.step_x(), 0);
                assert_eq!(dir.step_y(), 0);
                assert_eq!(dir.step_z(), 0);
            },
            Axis::Y => {
                assert_eq!(dir.step_x(), 0);
                assert_ne!(dir.step_y(), 0);
                assert_eq!(dir.step_z(), 0);
            },
            Axis::Z => {
                assert_eq!(dir.step_x(), 0);
                assert_eq!(dir.step_y(), 0);
                assert_ne!(dir.step_z(), 0);
            },
        }
    }
}

#[test]
fn block_pos_and_vec3i_direction_consistency() {
    let bp = BlockPos::new(5, 64, -10);
    let vi: Vec3i = bp.into();

    for dir in &direction::ALL {
        let bp_moved = bp.relative(*dir);
        let vi_moved = vi.relative(*dir);
        assert_eq!(bp_moved.x, vi_moved.x);
        assert_eq!(bp_moved.y, vi_moved.y);
        assert_eq!(bp_moved.z, vi_moved.z);
    }
}

#[test]
fn direction_from_y_rot_compass_points() {
    // Vanilla mapping: South=0°, West=90°, North=180°, East=270°
    assert_eq!(Direction::from_y_rot(0.0), Direction::South);
    assert_eq!(Direction::from_y_rot(90.0), Direction::West);
    assert_eq!(Direction::from_y_rot(180.0), Direction::North);
    assert_eq!(Direction::from_y_rot(270.0), Direction::East);
}

#[test]
fn direction_from_y_rot_wraps_at_360() {
    assert_eq!(Direction::from_y_rot(360.0), Direction::South);
    assert_eq!(Direction::from_y_rot(450.0), Direction::West);
    assert_eq!(Direction::from_y_rot(-90.0), Direction::East);
}

#[test]
fn direction_step_consistency() {
    for dir in &direction::ALL {
        let origin = BlockPos::new(0, 0, 0);
        let moved = origin.relative(*dir);
        // relative() should move by exactly step_x/y/z
        assert_eq!(moved.x, dir.step_x());
        assert_eq!(moved.y, dir.step_y());
        assert_eq!(moved.z, dir.step_z());
    }
}
