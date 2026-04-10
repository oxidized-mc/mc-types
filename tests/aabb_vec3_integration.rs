//! Integration tests: AABB ↔ Vec3 interactions.
//!
//! Verifies that AABB operations compose correctly with Vec3 inputs.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use oxidized_mc_types::{Aabb, BlockPos, EntityDimensions, Vec3};

const EPSILON: f64 = 1e-10;

#[test]
fn aabb_from_block_pos_contains_block_center() {
    let bp = BlockPos::new(10, 64, -30);
    let bb = Aabb::from_block_pos(&bp);
    let center = bp.get_center();
    assert!(
        bb.contains_vec(center),
        "AABB from block pos should contain block center"
    );
}

#[test]
fn aabb_from_vec3_pair() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);
    let bb = Aabb::from_vec3(a, b);
    assert!(bb.contains_vec(a));
    // b is at max corner (exclusive for contains), so check it's on the boundary
    assert!((bb.max_x - b.x).abs() < EPSILON);
}

#[test]
fn aabb_move_by_vec3() {
    let bb = Aabb::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    let delta = Vec3::new(10.0, 20.0, 30.0);
    let moved = bb.move_vec(delta);
    assert!((moved.min_x - 10.0).abs() < EPSILON);
    assert!((moved.min_y - 20.0).abs() < EPSILON);
    assert!((moved.min_z - 30.0).abs() < EPSILON);
    assert!((moved.max_x - 11.0).abs() < EPSILON);
    assert!((moved.max_y - 21.0).abs() < EPSILON);
    assert!((moved.max_z - 31.0).abs() < EPSILON);
}

#[test]
fn aabb_center_and_bottom_center() {
    let bb = Aabb::new(0.0, 0.0, 0.0, 2.0, 4.0, 6.0);
    let center = bb.get_center();
    assert!((center.x - 1.0).abs() < EPSILON);
    assert!((center.y - 2.0).abs() < EPSILON);
    assert!((center.z - 3.0).abs() < EPSILON);

    let bottom = bb.get_bottom_center();
    assert!((bottom.x - 1.0).abs() < EPSILON);
    assert!((bottom.y - 0.0).abs() < EPSILON);
    assert!((bottom.z - 3.0).abs() < EPSILON);
}

#[test]
fn aabb_inflate_increases_size() {
    let bb = Aabb::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    let inflated = bb.inflate(0.5);
    assert!((inflated.x_size() - 2.0).abs() < EPSILON);
    assert!((inflated.y_size() - 2.0).abs() < EPSILON);
    assert!((inflated.z_size() - 2.0).abs() < EPSILON);
}

#[test]
fn aabb_intersect_overlap() {
    let a = Aabb::new(0.0, 0.0, 0.0, 2.0, 2.0, 2.0);
    let b = Aabb::new(1.0, 1.0, 1.0, 3.0, 3.0, 3.0);
    assert!(a.intersects(&b));
    let inter = a.intersect(&b);
    assert!((inter.min_x - 1.0).abs() < EPSILON);
    assert!((inter.max_x - 2.0).abs() < EPSILON);
}

#[test]
fn aabb_distance_to_sqr_inside_is_zero() {
    let bb = Aabb::new(0.0, 0.0, 0.0, 10.0, 10.0, 10.0);
    let inside = Vec3::new(5.0, 5.0, 5.0);
    assert!(bb.distance_to_sqr(inside) < EPSILON);
}

#[test]
fn aabb_distance_to_sqr_outside() {
    let bb = Aabb::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    let outside = Vec3::new(2.0, 0.5, 0.5);
    // Distance is 1.0 along X axis
    assert!((bb.distance_to_sqr(outside) - 1.0).abs() < EPSILON);
}

#[test]
fn entity_dimensions_bounding_box_around_position() {
    let dims = EntityDimensions::new(0.6, 1.8);
    let pos = Vec3::new(10.0, 64.0, -30.0);
    let bb = dims.make_bounding_box(pos);

    let half_w = f64::from(0.6_f32) / 2.0;
    let h = f64::from(1.8_f32);
    assert!((bb.min_x - (10.0 - half_w)).abs() < EPSILON);
    assert!((bb.max_x - (10.0 + half_w)).abs() < EPSILON);
    assert!((bb.min_y - 64.0).abs() < EPSILON);
    assert!((bb.max_y - (64.0 + h)).abs() < EPSILON);
    assert!((bb.min_z - (-30.0 - half_w)).abs() < EPSILON);
    assert!((bb.max_z - (-30.0 + half_w)).abs() < EPSILON);
}

#[test]
fn aabb_minmax_union() {
    let a = Aabb::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    let b = Aabb::new(2.0, 2.0, 2.0, 3.0, 3.0, 3.0);
    let union = a.minmax(&b);
    assert!((union.min_x - 0.0).abs() < EPSILON);
    assert!((union.max_x - 3.0).abs() < EPSILON);
    assert!(union.contains_vec(a.get_center()));
    assert!(union.contains_vec(b.get_center()));
}

#[test]
fn aabb_clip_ray() {
    let bb = Aabb::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    let from = Vec3::new(-1.0, 0.5, 0.5);
    let to = Vec3::new(2.0, 0.5, 0.5);
    let hit = bb.clip(from, to).expect("Ray should hit AABB");
    // Hit point should be on the min_x face
    assert!((hit.x - 0.0).abs() < EPSILON);
}
