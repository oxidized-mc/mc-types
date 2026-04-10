//! [`EquipmentSlot`] and [`EquipmentSlotType`] — equipment slot positions on an entity.
//!
//! Wire IDs are non-sequential and match vanilla 26.1's assignment:
//! MainHand=0, Feet=1, Legs=2, Chest=3, Head=4, OffHand=5, Body=6, Saddle=7.

use bytes::{Bytes, BytesMut};

use oxidized_codec::types::TypeError;
use oxidized_codec::varint;

/// Equipment slot type category.
///
/// Groups equipment slots by their function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentSlotType {
    /// Main hand or off hand.
    Hand,
    /// Humanoid armor (feet, legs, chest, head).
    HumanoidArmor,
    /// Animal body armor.
    AnimalArmor,
    /// Saddle slot.
    Saddle,
}

impl std::fmt::Display for EquipmentSlotType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EquipmentSlotType::Hand => f.write_str("hand"),
            EquipmentSlotType::HumanoidArmor => f.write_str("humanoid_armor"),
            EquipmentSlotType::AnimalArmor => f.write_str("animal_armor"),
            EquipmentSlotType::Saddle => f.write_str("saddle"),
        }
    }
}

/// Equipment slot on an entity.
///
/// Wire IDs match vanilla 26.1's non-sequential assignment:
/// MainHand=0, Feet=1, Legs=2, Chest=3, Head=4, OffHand=5, Body=6, Saddle=7.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::EquipmentSlot;
///
/// let slot = EquipmentSlot::by_id(0).unwrap();
/// assert_eq!(slot, EquipmentSlot::MainHand);
/// assert_eq!(slot.to_string(), "mainhand");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentSlot {
    /// Main hand (wire ID 0).
    MainHand,
    /// Off hand (wire ID 5).
    OffHand,
    /// Feet armor (wire ID 1).
    Feet,
    /// Legs armor (wire ID 2).
    Legs,
    /// Chest armor (wire ID 3).
    Chest,
    /// Head armor (wire ID 4).
    Head,
    /// Animal body armor (wire ID 6).
    Body,
    /// Saddle (wire ID 7).
    Saddle,
}

impl EquipmentSlot {
    /// The wire-format ID for this slot (non-sequential — matches vanilla).
    pub const fn id(self) -> i32 {
        match self {
            Self::MainHand => 0,
            Self::Feet => 1,
            Self::Legs => 2,
            Self::Chest => 3,
            Self::Head => 4,
            Self::OffHand => 5,
            Self::Body => 6,
            Self::Saddle => 7,
        }
    }

    /// Looks up a slot by its wire ID.
    pub const fn by_id(id: i32) -> Option<Self> {
        match id {
            0 => Some(Self::MainHand),
            1 => Some(Self::Feet),
            2 => Some(Self::Legs),
            3 => Some(Self::Chest),
            4 => Some(Self::Head),
            5 => Some(Self::OffHand),
            6 => Some(Self::Body),
            7 => Some(Self::Saddle),
            _ => None,
        }
    }

    /// The slot type category.
    pub const fn slot_type(self) -> EquipmentSlotType {
        match self {
            Self::MainHand | Self::OffHand => EquipmentSlotType::Hand,
            Self::Feet | Self::Legs | Self::Chest | Self::Head => EquipmentSlotType::HumanoidArmor,
            Self::Body => EquipmentSlotType::AnimalArmor,
            Self::Saddle => EquipmentSlotType::Saddle,
        }
    }

    /// The slot index within its type group.
    ///
    /// Hands: MainHand=0, OffHand=1. Armor: Feet=0, Legs=1, Chest=2, Head=3.
    /// Body=0. Saddle=0.
    pub const fn index(self) -> usize {
        match self {
            Self::MainHand => 0,
            Self::OffHand => 1,
            Self::Feet => 0,
            Self::Legs => 1,
            Self::Chest => 2,
            Self::Head => 3,
            Self::Body => 0,
            Self::Saddle => 0,
        }
    }

    /// Whether this is a hand slot (MainHand or OffHand).
    pub const fn is_hand(self) -> bool {
        matches!(self, Self::MainHand | Self::OffHand)
    }

    /// Whether this is an armor slot (humanoid or animal).
    pub const fn is_armor(self) -> bool {
        matches!(
            self,
            Self::Feet | Self::Legs | Self::Chest | Self::Head | Self::Body
        )
    }

    /// Whether experience can be applied to items in this slot.
    ///
    /// In vanilla, this is true for all slots except saddle.
    pub const fn can_increase_experience(self) -> bool {
        !matches!(self, Self::Saddle)
    }

    /// The lowercase name of this slot.
    pub const fn name(self) -> &'static str {
        match self {
            Self::MainHand => "mainhand",
            Self::OffHand => "offhand",
            Self::Feet => "feet",
            Self::Legs => "legs",
            Self::Chest => "chest",
            Self::Head => "head",
            Self::Body => "body",
            Self::Saddle => "saddle",
        }
    }

    /// Reads an `EquipmentSlot` from a wire buffer as a VarInt.
    ///
    /// # Errors
    ///
    /// Returns [`TypeError`] if the buffer is truncated or the ID is unknown.
    pub fn read(buf: &mut Bytes) -> Result<Self, TypeError> {
        let id = varint::read_varint_buf(buf)?;
        Self::by_id(id).ok_or(TypeError::InvalidValue { value: id })
    }

    /// Writes this `EquipmentSlot` to a wire buffer as a VarInt.
    pub fn write(&self, buf: &mut BytesMut) {
        varint::write_varint_buf(self.id(), buf);
    }
}

impl std::fmt::Display for EquipmentSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use bytes::{Bytes, BytesMut};

    use super::*;

    // ── Wire ID mapping ─────────────────────────────────────────────────

    #[test]
    fn test_equipment_slot_ids_match_vanilla() {
        assert_eq!(EquipmentSlot::MainHand.id(), 0);
        assert_eq!(EquipmentSlot::Feet.id(), 1);
        assert_eq!(EquipmentSlot::Legs.id(), 2);
        assert_eq!(EquipmentSlot::Chest.id(), 3);
        assert_eq!(EquipmentSlot::Head.id(), 4);
        assert_eq!(EquipmentSlot::OffHand.id(), 5);
        assert_eq!(EquipmentSlot::Body.id(), 6);
        assert_eq!(EquipmentSlot::Saddle.id(), 7);
    }

    #[test]
    fn test_equipment_slot_by_id_roundtrip() {
        for id in 0..=7 {
            let slot = EquipmentSlot::by_id(id).unwrap();
            assert_eq!(slot.id(), id);
        }
    }

    #[test]
    fn test_equipment_slot_by_id_invalid() {
        assert!(EquipmentSlot::by_id(-1).is_none());
        assert!(EquipmentSlot::by_id(8).is_none());
        assert!(EquipmentSlot::by_id(100).is_none());
    }

    // ── Slot types ──────────────────────────────────────────────────────

    #[test]
    fn test_equipment_slot_types() {
        assert_eq!(EquipmentSlot::MainHand.slot_type(), EquipmentSlotType::Hand);
        assert_eq!(EquipmentSlot::OffHand.slot_type(), EquipmentSlotType::Hand);
        assert_eq!(
            EquipmentSlot::Feet.slot_type(),
            EquipmentSlotType::HumanoidArmor
        );
        assert_eq!(
            EquipmentSlot::Legs.slot_type(),
            EquipmentSlotType::HumanoidArmor
        );
        assert_eq!(
            EquipmentSlot::Chest.slot_type(),
            EquipmentSlotType::HumanoidArmor
        );
        assert_eq!(
            EquipmentSlot::Head.slot_type(),
            EquipmentSlotType::HumanoidArmor
        );
        assert_eq!(
            EquipmentSlot::Body.slot_type(),
            EquipmentSlotType::AnimalArmor
        );
        assert_eq!(EquipmentSlot::Saddle.slot_type(), EquipmentSlotType::Saddle);
    }

    // ── Indices ─────────────────────────────────────────────────────────

    #[test]
    fn test_equipment_slot_indices() {
        assert_eq!(EquipmentSlot::MainHand.index(), 0);
        assert_eq!(EquipmentSlot::OffHand.index(), 1);
        assert_eq!(EquipmentSlot::Feet.index(), 0);
        assert_eq!(EquipmentSlot::Legs.index(), 1);
        assert_eq!(EquipmentSlot::Chest.index(), 2);
        assert_eq!(EquipmentSlot::Head.index(), 3);
        assert_eq!(EquipmentSlot::Body.index(), 0);
        assert_eq!(EquipmentSlot::Saddle.index(), 0);
    }

    // ── Boolean queries ─────────────────────────────────────────────────

    #[test]
    fn test_equipment_slot_is_hand() {
        assert!(EquipmentSlot::MainHand.is_hand());
        assert!(EquipmentSlot::OffHand.is_hand());
        assert!(!EquipmentSlot::Feet.is_hand());
        assert!(!EquipmentSlot::Head.is_hand());
        assert!(!EquipmentSlot::Body.is_hand());
        assert!(!EquipmentSlot::Saddle.is_hand());
    }

    #[test]
    fn test_equipment_slot_is_armor() {
        assert!(!EquipmentSlot::MainHand.is_armor());
        assert!(!EquipmentSlot::OffHand.is_armor());
        assert!(EquipmentSlot::Feet.is_armor());
        assert!(EquipmentSlot::Legs.is_armor());
        assert!(EquipmentSlot::Chest.is_armor());
        assert!(EquipmentSlot::Head.is_armor());
        assert!(EquipmentSlot::Body.is_armor());
        assert!(!EquipmentSlot::Saddle.is_armor());
    }

    #[test]
    fn test_equipment_slot_can_increase_experience() {
        assert!(EquipmentSlot::MainHand.can_increase_experience());
        assert!(EquipmentSlot::OffHand.can_increase_experience());
        assert!(EquipmentSlot::Head.can_increase_experience());
        assert!(EquipmentSlot::Body.can_increase_experience());
        assert!(!EquipmentSlot::Saddle.can_increase_experience());
    }

    // ── Wire roundtrip ──────────────────────────────────────────────────

    #[test]
    fn test_equipment_slot_wire_roundtrip_all() {
        let slots = [
            EquipmentSlot::MainHand,
            EquipmentSlot::OffHand,
            EquipmentSlot::Feet,
            EquipmentSlot::Legs,
            EquipmentSlot::Chest,
            EquipmentSlot::Head,
            EquipmentSlot::Body,
            EquipmentSlot::Saddle,
        ];
        for slot in slots {
            let mut buf = BytesMut::new();
            slot.write(&mut buf);
            let mut read_buf = Bytes::from(buf);
            let decoded = EquipmentSlot::read(&mut read_buf).unwrap();
            assert_eq!(slot, decoded, "roundtrip failed for {slot}");
        }
    }

    #[test]
    fn test_equipment_slot_read_empty_buffer() {
        let mut buf = Bytes::new();
        assert!(EquipmentSlot::read(&mut buf).is_err());
    }

    // ── Display ─────────────────────────────────────────────────────────

    #[test]
    fn test_equipment_slot_display() {
        assert_eq!(format!("{}", EquipmentSlot::MainHand), "mainhand");
        assert_eq!(format!("{}", EquipmentSlot::OffHand), "offhand");
    }

    // ── Snapshot tests ──────────────────────────────────────────────

    mod snapshots {
        use super::*;

        #[test]
        fn snapshot_equipment_slot_display() {
            insta::assert_snapshot!(EquipmentSlot::MainHand.to_string(), @"mainhand");
            insta::assert_snapshot!(EquipmentSlot::OffHand.to_string(), @"offhand");
            insta::assert_snapshot!(EquipmentSlot::Head.to_string(), @"head");
            insta::assert_snapshot!(EquipmentSlot::Chest.to_string(), @"chest");
            insta::assert_snapshot!(EquipmentSlot::Legs.to_string(), @"legs");
            insta::assert_snapshot!(EquipmentSlot::Feet.to_string(), @"feet");
            insta::assert_snapshot!(EquipmentSlot::Body.to_string(), @"body");
        }
    }
}
