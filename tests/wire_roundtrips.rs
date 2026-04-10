//! Integration tests: wire (encode/decode) roundtrips for all types.
//!
//! Tests that encoding to bytes and decoding back produces identical values
//! for every type that implements read/write.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use bytes::BytesMut;

use oxidized_mc_types::{
    BlockPos, BlockState, ChatVisibility, Difficulty, EntitySpawnReason, EquipmentSlot, GameType,
    HumanoidArm, InteractionHand, MobCategory, ParticleStatus, Pose, ResourceLocation, Rotations,
    SectionPos, SoundSource, Vec2, Vec3, Vec3i,
};

/// Helper: encode a value, decode it, and assert equality.
macro_rules! assert_wire_roundtrip {
    ($ty:ty, $val:expr) => {{
        let original = $val;
        let mut buf = BytesMut::new();
        original.write(&mut buf);
        let mut data = buf.freeze();
        let decoded = <$ty>::read(&mut data).unwrap();
        assert_eq!(
            original,
            decoded,
            "Wire roundtrip failed for {}",
            stringify!($ty)
        );
    }};
}

// ── Coordinate types ───────────────────────────────────────────────

#[test]
fn wire_roundtrip_block_pos() {
    assert_wire_roundtrip!(BlockPos, BlockPos::new(100, -64, 200));
    assert_wire_roundtrip!(BlockPos, BlockPos::ZERO);
    assert_wire_roundtrip!(BlockPos, BlockPos::new(-33_554_432, -2048, -33_554_432));
}

#[test]
fn wire_roundtrip_section_pos() {
    assert_wire_roundtrip!(SectionPos, SectionPos::new(6, -4, 12));
    assert_wire_roundtrip!(SectionPos, SectionPos::new(0, 0, 0));
}

#[test]
fn wire_roundtrip_vec3() {
    assert_wire_roundtrip!(Vec3, Vec3::new(1.5, -2.7, 3.14285));
    assert_wire_roundtrip!(Vec3, Vec3::ZERO);
}

#[test]
fn wire_roundtrip_vec2() {
    assert_wire_roundtrip!(Vec2, Vec2::new(1.5, -2.7));
    assert_wire_roundtrip!(Vec2, Vec2::ZERO);
}

#[test]
fn wire_roundtrip_vec3i() {
    assert_wire_roundtrip!(Vec3i, Vec3i::new(10, -20, 30));
    assert_wire_roundtrip!(Vec3i, Vec3i::ZERO);
}

// ── Resource types ─────────────────────────────────────────────────

#[test]
fn wire_roundtrip_resource_location() {
    assert_wire_roundtrip!(ResourceLocation, ResourceLocation::minecraft("stone"));
    assert_wire_roundtrip!(
        ResourceLocation,
        ResourceLocation::new("mymod", "items/sword").unwrap()
    );
}

#[test]
fn wire_roundtrip_block_state() {
    assert_wire_roundtrip!(BlockState, BlockState::new(0));
    assert_wire_roundtrip!(BlockState, BlockState::new(42));
    assert_wire_roundtrip!(BlockState, BlockState::new(u16::MAX));
}

// ── Protocol enums ─────────────────────────────────────────────────

#[test]
fn wire_roundtrip_game_type_all_variants() {
    for id in 0..=3 {
        let gt = GameType::by_id(id).unwrap();
        assert_wire_roundtrip!(GameType, gt);
    }
}

#[test]
fn wire_roundtrip_difficulty_all_variants() {
    for id in 0..=3 {
        let d = Difficulty::by_id(id).unwrap();
        assert_wire_roundtrip!(Difficulty, d);
    }
}

#[test]
fn wire_roundtrip_chat_visibility_all_variants() {
    for id in 0..=2 {
        let v = ChatVisibility::by_id(id).unwrap();
        assert_wire_roundtrip!(ChatVisibility, v);
    }
}

#[test]
fn wire_roundtrip_humanoid_arm_all_variants() {
    for id in 0..=1 {
        let arm = HumanoidArm::by_id(id).unwrap();
        assert_wire_roundtrip!(HumanoidArm, arm);
    }
}

#[test]
fn wire_roundtrip_particle_status_all_variants() {
    for id in 0..=2 {
        let ps = ParticleStatus::by_id(id).unwrap();
        assert_wire_roundtrip!(ParticleStatus, ps);
    }
}

#[test]
fn wire_roundtrip_interaction_hand_all_variants() {
    for id in 0..=1 {
        let h = InteractionHand::by_id(id).unwrap();
        assert_wire_roundtrip!(InteractionHand, h);
    }
}

#[test]
fn wire_roundtrip_pose_all_variants() {
    for id in 0..=17 {
        let p = Pose::by_id(id).unwrap();
        assert_wire_roundtrip!(Pose, p);
    }
}

#[test]
fn wire_roundtrip_equipment_slot_all_variants() {
    let slots = [
        EquipmentSlot::MainHand,
        EquipmentSlot::OffHand,
        EquipmentSlot::Feet,
        EquipmentSlot::Legs,
        EquipmentSlot::Chest,
        EquipmentSlot::Head,
        EquipmentSlot::Body,
    ];
    for slot in &slots {
        assert_wire_roundtrip!(EquipmentSlot, *slot);
    }
}

// ── Rotations ──────────────────────────────────────────────────────

#[test]
fn wire_roundtrip_rotations() {
    assert_wire_roundtrip!(Rotations, Rotations::new(10.0, 20.0, 30.0));
    assert_wire_roundtrip!(Rotations, Rotations::ZERO);
}

// ── Phase 07 enums ────────────────────────────────────────────────

#[test]
fn wire_roundtrip_mob_category_all_variants() {
    for id in 0..=7 {
        let mc = MobCategory::by_id(id).unwrap();
        assert_wire_roundtrip!(MobCategory, mc);
    }
}

#[test]
fn wire_roundtrip_sound_source_all_variants() {
    for id in 0..=10 {
        let ss = SoundSource::by_id(id).unwrap();
        assert_wire_roundtrip!(SoundSource, ss);
    }
}

#[test]
fn wire_roundtrip_entity_spawn_reason_all_variants() {
    for id in 0..=18 {
        let esr = EntitySpawnReason::by_id(id).unwrap();
        assert_wire_roundtrip!(EntitySpawnReason, esr);
    }
}

// ── Multi-value buffer ─────────────────────────────────────────────

#[test]
fn wire_multiple_values_in_sequence() {
    let bp = BlockPos::new(10, 64, -30);
    let v3 = Vec3::new(1.5, 2.5, 3.5);
    let rl = ResourceLocation::minecraft("stone");

    let mut buf = BytesMut::new();
    bp.write(&mut buf);
    v3.write(&mut buf);
    rl.write(&mut buf);

    let mut data = buf.freeze();
    let decoded_bp = BlockPos::read(&mut data).unwrap();
    let decoded_v3 = Vec3::read(&mut data).unwrap();
    let decoded_rl = ResourceLocation::read(&mut data).unwrap();

    assert_eq!(decoded_bp, bp);
    assert_eq!(decoded_v3, v3);
    assert_eq!(decoded_rl, rl);
    assert!(data.is_empty(), "Buffer should be fully consumed");
}
