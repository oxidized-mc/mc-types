//! [`InteractionHand`] — which hand the player used for an interaction.
//!
//! Encoded as a VarInt on the wire (MainHand=0, OffHand=1).

use crate::equipment_slot::EquipmentSlot;

/// Which hand the player is using.
///
/// Encoded as a VarInt in interaction-related packets.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::InteractionHand;
///
/// let hand = InteractionHand::by_id(0).unwrap();
/// assert_eq!(hand, InteractionHand::MainHand);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum InteractionHand {
    /// The player's main hand.
    MainHand = 0,
    /// The player's off hand.
    OffHand = 1,
}

impl_protocol_enum! {
    InteractionHand {
        MainHand = 0 => "main_hand",
        OffHand  = 1 => "off_hand",
    }
}

impl InteractionHand {
    /// Converts this hand to the corresponding equipment slot.
    pub const fn as_equipment_slot(self) -> EquipmentSlot {
        match self {
            Self::MainHand => EquipmentSlot::MainHand,
            Self::OffHand => EquipmentSlot::OffHand,
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use bytes::{Bytes, BytesMut};

    use super::*;

    // ── ID mapping ──────────────────────────────────────────────────────

    #[test]
    fn test_interaction_hand_ids() {
        assert_eq!(InteractionHand::MainHand.id(), 0);
        assert_eq!(InteractionHand::OffHand.id(), 1);
    }

    #[test]
    fn test_interaction_hand_by_id() {
        assert_eq!(InteractionHand::by_id(0), Some(InteractionHand::MainHand));
        assert_eq!(InteractionHand::by_id(1), Some(InteractionHand::OffHand));
        assert_eq!(InteractionHand::by_id(2), None);
    }

    #[test]
    fn test_interaction_hand_by_name() {
        assert_eq!(
            InteractionHand::by_name("main_hand"),
            Some(InteractionHand::MainHand)
        );
        assert_eq!(
            InteractionHand::by_name("off_hand"),
            Some(InteractionHand::OffHand)
        );
        assert_eq!(InteractionHand::by_name("invalid"), None);
    }

    // ── as_equipment_slot ───────────────────────────────────────────────

    #[test]
    fn test_interaction_hand_as_equipment_slot() {
        assert_eq!(
            InteractionHand::MainHand.as_equipment_slot(),
            EquipmentSlot::MainHand
        );
        assert_eq!(
            InteractionHand::OffHand.as_equipment_slot(),
            EquipmentSlot::OffHand
        );
    }

    // ── Wire roundtrip ──────────────────────────────────────────────────

    #[test]
    fn test_interaction_hand_wire_roundtrip() {
        for hand in [InteractionHand::MainHand, InteractionHand::OffHand] {
            let mut buf = BytesMut::new();
            hand.write(&mut buf);
            let mut read_buf = Bytes::from(buf);
            let decoded = InteractionHand::read(&mut read_buf).unwrap();
            assert_eq!(hand, decoded);
        }
    }

    #[test]
    fn test_interaction_hand_read_empty_buffer() {
        let mut buf = Bytes::new();
        assert!(InteractionHand::read(&mut buf).is_err());
    }

    // ── Display ─────────────────────────────────────────────────────────

    #[test]
    fn test_interaction_hand_display() {
        assert_eq!(format!("{}", InteractionHand::MainHand), "main_hand");
        assert_eq!(format!("{}", InteractionHand::OffHand), "off_hand");
    }

    // ── Snapshot tests ──────────────────────────────────────────────

    mod snapshots {
        use super::*;

        #[test]
        fn snapshot_interaction_hand_display() {
            insta::assert_snapshot!(InteractionHand::MainHand.to_string(), @"main_hand");
            insta::assert_snapshot!(InteractionHand::OffHand.to_string(), @"off_hand");
        }
    }
}
