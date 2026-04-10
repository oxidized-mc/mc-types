//! [`MobCategory`] — mob spawning categories.
//!
//! Maps to vanilla's `MobCategory` enum used by the natural spawning system
//! to manage per-chunk spawn caps, despawn distances, and behavior flags.

use bytes::{Bytes, BytesMut};

use oxidized_codec::types::TypeError;
use oxidized_codec::varint;

/// Mob spawning category.
///
/// Each category defines spawn caps, despawn behavior, and whether the mobs
/// in it are friendly. The natural spawning system uses these categories to
/// decide how many mobs of each type can exist per chunk.
///
/// # Wire format
///
/// Encoded as a VarInt (0–7) in ordinal order.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::MobCategory;
///
/// let cat = MobCategory::by_id(0).unwrap();
/// assert_eq!(cat, MobCategory::Monster);
/// assert_eq!(cat.name(), "monster");
/// assert_eq!(cat.max_instances_per_chunk(), 70);
/// assert!(!cat.is_friendly());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MobCategory {
    /// Hostile mobs (zombies, skeletons, etc.) — cap 70.
    Monster = 0,
    /// Passive land animals (cows, pigs, etc.) — cap 10, persistent.
    Creature = 1,
    /// Ambient mobs (bats) — cap 15.
    Ambient = 2,
    /// Axolotls — cap 5.
    Axolotls = 3,
    /// Underground water creatures (glow squid) — cap 5.
    UndergroundWaterCreature = 4,
    /// Water creatures (dolphins, squid) — cap 5.
    WaterCreature = 5,
    /// Water ambient mobs (fish) — cap 20, shorter despawn distance.
    WaterAmbient = 6,
    /// Miscellaneous entities (non-mob entities) — no cap, persistent.
    Misc = 7,
}

/// Constant no-despawn distance shared by all categories (32 blocks).
const NO_DESPAWN_DISTANCE: i32 = 32;

impl MobCategory {
    /// All variants in ordinal order.
    pub const ALL: [MobCategory; 8] = [
        Self::Monster,
        Self::Creature,
        Self::Ambient,
        Self::Axolotls,
        Self::UndergroundWaterCreature,
        Self::WaterCreature,
        Self::WaterAmbient,
        Self::Misc,
    ];

    /// Returns the numeric ID (ordinal) of this category.
    pub const fn id(self) -> i32 {
        self as i32
    }

    /// The serialized name of this category (e.g., `"monster"`, `"creature"`).
    pub const fn name(self) -> &'static str {
        match self {
            Self::Monster => "monster",
            Self::Creature => "creature",
            Self::Ambient => "ambient",
            Self::Axolotls => "axolotls",
            Self::UndergroundWaterCreature => "underground_water_creature",
            Self::WaterCreature => "water_creature",
            Self::WaterAmbient => "water_ambient",
            Self::Misc => "misc",
        }
    }

    /// Maximum mob instances per chunk for this category.
    ///
    /// Returns −1 for [`MobCategory::Misc`] (unlimited).
    pub const fn max_instances_per_chunk(self) -> i32 {
        match self {
            Self::Monster => 70,
            Self::Creature => 10,
            Self::Ambient => 15,
            Self::Axolotls => 5,
            Self::UndergroundWaterCreature => 5,
            Self::WaterCreature => 5,
            Self::WaterAmbient => 20,
            Self::Misc => -1,
        }
    }

    /// Whether mobs in this category are friendly (non-hostile).
    pub const fn is_friendly(self) -> bool {
        !matches!(self, Self::Monster)
    }

    /// Whether mobs in this category are persistent (don't despawn naturally).
    pub const fn is_persistent(self) -> bool {
        matches!(self, Self::Creature | Self::Misc)
    }

    /// The distance (in blocks) at which mobs in this category despawn.
    pub const fn despawn_distance(self) -> i32 {
        match self {
            Self::WaterAmbient => 64,
            _ => 128,
        }
    }

    /// The distance (in blocks) within which mobs never despawn.
    ///
    /// Always returns 32 for all categories.
    pub const fn no_despawn_distance(self) -> i32 {
        NO_DESPAWN_DISTANCE
    }

    /// Looks up a category by its numeric ID.
    pub const fn by_id(id: i32) -> Option<Self> {
        match id {
            0 => Some(Self::Monster),
            1 => Some(Self::Creature),
            2 => Some(Self::Ambient),
            3 => Some(Self::Axolotls),
            4 => Some(Self::UndergroundWaterCreature),
            5 => Some(Self::WaterCreature),
            6 => Some(Self::WaterAmbient),
            7 => Some(Self::Misc),
            _ => None,
        }
    }

    /// Looks up a category by its serialized name.
    pub fn by_name(name: &str) -> Option<Self> {
        match name {
            "monster" => Some(Self::Monster),
            "creature" => Some(Self::Creature),
            "ambient" => Some(Self::Ambient),
            "axolotls" => Some(Self::Axolotls),
            "underground_water_creature" => Some(Self::UndergroundWaterCreature),
            "water_creature" => Some(Self::WaterCreature),
            "water_ambient" => Some(Self::WaterAmbient),
            "misc" => Some(Self::Misc),
            _ => None,
        }
    }

    /// Reads a `MobCategory` from a wire buffer as a VarInt.
    ///
    /// # Errors
    ///
    /// Returns [`TypeError`] if the buffer is truncated or the ID is unknown.
    pub fn read(buf: &mut Bytes) -> Result<Self, TypeError> {
        let id = varint::read_varint_buf(buf)?;
        Self::by_id(id).ok_or(TypeError::InvalidValue { value: id })
    }

    /// Writes this `MobCategory` to a wire buffer as a VarInt.
    pub fn write(&self, buf: &mut BytesMut) {
        varint::write_varint_buf(self.id(), buf);
    }
}

impl std::fmt::Display for MobCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use bytes::BytesMut;

    use super::*;

    // ── Metadata accuracy ───────────────────────────────────────────

    #[test]
    fn test_mob_category_monster() {
        let cat = MobCategory::Monster;
        assert_eq!(cat.name(), "monster");
        assert_eq!(cat.max_instances_per_chunk(), 70);
        assert!(!cat.is_friendly());
        assert!(!cat.is_persistent());
        assert_eq!(cat.despawn_distance(), 128);
    }

    #[test]
    fn test_mob_category_creature() {
        let cat = MobCategory::Creature;
        assert_eq!(cat.name(), "creature");
        assert_eq!(cat.max_instances_per_chunk(), 10);
        assert!(cat.is_friendly());
        assert!(cat.is_persistent());
        assert_eq!(cat.despawn_distance(), 128);
    }

    #[test]
    fn test_mob_category_ambient() {
        let cat = MobCategory::Ambient;
        assert_eq!(cat.name(), "ambient");
        assert_eq!(cat.max_instances_per_chunk(), 15);
        assert!(cat.is_friendly());
        assert!(!cat.is_persistent());
    }

    #[test]
    fn test_mob_category_water_ambient_despawn_distance() {
        assert_eq!(MobCategory::WaterAmbient.despawn_distance(), 64);
        assert_eq!(MobCategory::Monster.despawn_distance(), 128);
        assert_eq!(MobCategory::WaterCreature.despawn_distance(), 128);
    }

    #[test]
    fn test_mob_category_misc() {
        let cat = MobCategory::Misc;
        assert_eq!(cat.max_instances_per_chunk(), -1);
        assert!(cat.is_friendly());
        assert!(cat.is_persistent());
    }

    #[test]
    fn test_mob_category_no_despawn_distance_constant() {
        for cat in MobCategory::ALL {
            assert_eq!(cat.no_despawn_distance(), 32);
        }
    }

    // ── by_id ───────────────────────────────────────────────────────

    #[test]
    fn test_mob_category_by_id_all() {
        for id in 0..=7 {
            let cat = MobCategory::by_id(id).unwrap();
            assert_eq!(cat.id(), id);
        }
    }

    #[test]
    fn test_mob_category_by_id_invalid() {
        assert!(MobCategory::by_id(-1).is_none());
        assert!(MobCategory::by_id(8).is_none());
        assert!(MobCategory::by_id(100).is_none());
    }

    // ── by_name ─────────────────────────────────────────────────────

    #[test]
    fn test_mob_category_by_name_valid() {
        assert_eq!(MobCategory::by_name("monster"), Some(MobCategory::Monster));
        assert_eq!(
            MobCategory::by_name("creature"),
            Some(MobCategory::Creature)
        );
        assert_eq!(
            MobCategory::by_name("water_ambient"),
            Some(MobCategory::WaterAmbient)
        );
        assert_eq!(
            MobCategory::by_name("underground_water_creature"),
            Some(MobCategory::UndergroundWaterCreature)
        );
    }

    #[test]
    fn test_mob_category_by_name_invalid() {
        assert!(MobCategory::by_name("Monster").is_none());
        assert!(MobCategory::by_name("unknown").is_none());
        assert!(MobCategory::by_name("").is_none());
    }

    // ── Display ─────────────────────────────────────────────────────

    #[test]
    fn test_mob_category_display() {
        assert_eq!(format!("{}", MobCategory::Monster), "monster");
        assert_eq!(format!("{}", MobCategory::Creature), "creature");
        assert_eq!(format!("{}", MobCategory::WaterAmbient), "water_ambient");
    }

    // ── Wire roundtrip ──────────────────────────────────────────────

    #[test]
    fn test_mob_category_wire_roundtrip() {
        for cat in MobCategory::ALL {
            let mut buf = BytesMut::new();
            cat.write(&mut buf);
            let mut data = buf.freeze();
            let decoded = MobCategory::read(&mut data).unwrap();
            assert_eq!(decoded, cat, "roundtrip failed for {cat}");
        }
    }

    // ── ALL constant ────────────────────────────────────────────────

    #[test]
    fn test_mob_category_all_count() {
        assert_eq!(MobCategory::ALL.len(), 8);
    }

    #[test]
    fn test_mob_category_all_ordinals_sequential() {
        for (i, cat) in MobCategory::ALL.iter().enumerate() {
            assert_eq!(cat.id(), i as i32);
        }
    }

    // ── Property-based tests ────────────────────────────────────────

    mod prop {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn mob_category_id_roundtrip(id in 0i32..8) {
                let cat = MobCategory::by_id(id).unwrap();
                prop_assert_eq!(cat.id(), id);
            }

            #[test]
            fn mob_category_name_roundtrip(id in 0i32..8) {
                let cat = MobCategory::by_id(id).unwrap();
                let name = cat.name();
                prop_assert_eq!(MobCategory::by_name(name), Some(cat));
            }
        }
    }

    // ── Snapshot tests ──────────────────────────────────────────────

    mod snapshots {
        use super::*;

        #[test]
        fn snapshot_mob_category_display() {
            insta::assert_snapshot!(MobCategory::Monster.to_string(), @"monster");
            insta::assert_snapshot!(MobCategory::Creature.to_string(), @"creature");
            insta::assert_snapshot!(MobCategory::Ambient.to_string(), @"ambient");
            insta::assert_snapshot!(MobCategory::Axolotls.to_string(), @"axolotls");
            insta::assert_snapshot!(
                MobCategory::UndergroundWaterCreature.to_string(),
                @"underground_water_creature"
            );
            insta::assert_snapshot!(
                MobCategory::WaterCreature.to_string(),
                @"water_creature"
            );
            insta::assert_snapshot!(
                MobCategory::WaterAmbient.to_string(),
                @"water_ambient"
            );
            insta::assert_snapshot!(MobCategory::Misc.to_string(), @"misc");
        }
    }
}
