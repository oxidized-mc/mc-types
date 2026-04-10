//! [`Pose`] — an entity's physical pose state.
//!
//! Affects hitbox dimensions and animations. Transmitted as a VarInt in
//! entity metadata packets.

/// An entity's physical pose state.
///
/// Affects hitbox dimensions and animations. Transmitted as a VarInt in
/// entity metadata packets.
///
/// All 18 variants match vanilla 26.1's `net.minecraft.world.entity.Pose`.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::Pose;
///
/// let p = Pose::by_id(0).unwrap();
/// assert_eq!(p, Pose::Standing);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Pose {
    /// Default standing pose.
    Standing = 0,
    /// Elytra gliding.
    FallFlying = 1,
    /// Sleeping in a bed.
    Sleeping = 2,
    /// Swimming or crawling.
    Swimming = 3,
    /// Riptide trident attack.
    SpinAttack = 4,
    /// Sneaking / crouching.
    Crouching = 5,
    /// Frog long jump.
    LongJumping = 6,
    /// Death animation.
    Dying = 7,
    /// Frog croaking.
    Croaking = 8,
    /// Frog using tongue.
    UsingTongue = 9,
    /// Camel / sniffer sitting.
    Sitting = 10,
    /// Warden roaring.
    Roaring = 11,
    /// Sniffer sniffing.
    Sniffing = 12,
    /// Warden emerging.
    Emerging = 13,
    /// Warden digging.
    Digging = 14,
    /// Breeze sliding.
    Sliding = 15,
    /// Breeze shooting.
    Shooting = 16,
    /// Breeze inhaling.
    Inhaling = 17,
}

impl_protocol_enum! {
    Pose {
        Standing    = 0  => "standing",
        FallFlying  = 1  => "fall_flying",
        Sleeping    = 2  => "sleeping",
        Swimming    = 3  => "swimming",
        SpinAttack  = 4  => "spin_attack",
        Crouching   = 5  => "crouching",
        LongJumping = 6  => "long_jumping",
        Dying       = 7  => "dying",
        Croaking    = 8  => "croaking",
        UsingTongue = 9  => "using_tongue",
        Sitting     = 10 => "sitting",
        Roaring     = 11 => "roaring",
        Sniffing    = 12 => "sniffing",
        Emerging    = 13 => "emerging",
        Digging     = 14 => "digging",
        Sliding     = 15 => "sliding",
        Shooting    = 16 => "shooting",
        Inhaling    = 17 => "inhaling",
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use bytes::{Bytes, BytesMut};

    use super::*;

    // ── ID mapping ──────────────────────────────────────────────────────

    #[test]
    fn test_pose_all_ids() {
        let expected = [
            (Pose::Standing, 0),
            (Pose::FallFlying, 1),
            (Pose::Sleeping, 2),
            (Pose::Swimming, 3),
            (Pose::SpinAttack, 4),
            (Pose::Crouching, 5),
            (Pose::LongJumping, 6),
            (Pose::Dying, 7),
            (Pose::Croaking, 8),
            (Pose::UsingTongue, 9),
            (Pose::Sitting, 10),
            (Pose::Roaring, 11),
            (Pose::Sniffing, 12),
            (Pose::Emerging, 13),
            (Pose::Digging, 14),
            (Pose::Sliding, 15),
            (Pose::Shooting, 16),
            (Pose::Inhaling, 17),
        ];
        for (pose, id) in expected {
            assert_eq!(pose.id(), id, "id mismatch for {pose:?}");
            assert_eq!(Pose::by_id(id), Some(pose), "by_id mismatch for id {id}");
        }
    }

    #[test]
    fn test_pose_by_id_invalid() {
        assert!(Pose::by_id(-1).is_none());
        assert!(Pose::by_id(18).is_none());
    }

    // ── Wire roundtrip (all 18 variants) ────────────────────────────────

    #[test]
    fn test_pose_wire_roundtrip_all() {
        for id in 0..=17 {
            let pose = Pose::by_id(id).unwrap();
            let mut buf = BytesMut::new();
            pose.write(&mut buf);
            let mut read_buf = Bytes::from(buf);
            let decoded = Pose::read(&mut read_buf).unwrap();
            assert_eq!(pose, decoded, "roundtrip failed for {pose:?}");
        }
    }

    #[test]
    fn test_pose_read_empty_buffer() {
        let mut buf = Bytes::new();
        assert!(Pose::read(&mut buf).is_err());
    }

    // ── Name mapping ────────────────────────────────────────────────────

    #[test]
    fn test_pose_names() {
        assert_eq!(Pose::Standing.name(), "standing");
        assert_eq!(Pose::FallFlying.name(), "fall_flying");
        assert_eq!(Pose::Inhaling.name(), "inhaling");
    }

    #[test]
    fn test_pose_by_name() {
        assert_eq!(Pose::by_name("standing"), Some(Pose::Standing));
        assert_eq!(Pose::by_name("fall_flying"), Some(Pose::FallFlying));
        assert_eq!(Pose::by_name("invalid"), None);
    }

    // ── Display ─────────────────────────────────────────────────────────

    #[test]
    fn test_pose_display() {
        assert_eq!(format!("{}", Pose::Standing), "standing");
        assert_eq!(format!("{}", Pose::SpinAttack), "spin_attack");
    }

    // ── Property-based tests ────────────────────────────────────────

    mod prop {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn pose_id_roundtrip(id in 0i32..18) {
                let pose = Pose::by_id(id).unwrap();
                prop_assert_eq!(pose.id(), id);
            }
        }
    }

    // ── Snapshot tests ──────────────────────────────────────────────

    mod snapshots {
        use super::*;

        #[test]
        fn snapshot_pose_display() {
            insta::assert_snapshot!(Pose::Standing.to_string(), @"standing");
            insta::assert_snapshot!(Pose::Crouching.to_string(), @"crouching");
            insta::assert_snapshot!(Pose::SpinAttack.to_string(), @"spin_attack");
        }
    }
}
