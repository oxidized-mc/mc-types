//! [`EntitySpawnReason`] — reasons an entity was spawned into the world.
//!
//! Maps to vanilla's `EntitySpawnReason` enum used by the entity spawning
//! system to track how entities entered the world and to apply spawn-specific
//! behavior rules (e.g., light requirements, spawn caps).

/// The reason an entity was spawned into the world.
///
/// Different spawn reasons trigger different behavior — for example, trial
/// spawner entities ignore light-level requirements, and the natural spawning
/// system only counts entities with [`EntitySpawnReason::Natural`] against
/// per-chunk caps.
///
/// # Wire format
///
/// Encoded as a VarInt (0–18) in ordinal order.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::EntitySpawnReason;
///
/// let reason = EntitySpawnReason::by_id(0).unwrap();
/// assert_eq!(reason, EntitySpawnReason::Natural);
/// assert!(!reason.is_spawner());
///
/// assert!(EntitySpawnReason::Spawner.is_spawner());
/// assert!(EntitySpawnReason::TrialSpawner.is_spawner());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EntitySpawnReason {
    /// Natural mob spawning (random ticks).
    Natural = 0,
    /// Spawned during initial chunk generation.
    ChunkGeneration = 1,
    /// Spawned by a mob spawner block.
    Spawner = 2,
    /// Spawned as part of a structure (e.g., village iron golem).
    Structure = 3,
    /// Spawned from animal breeding.
    Breeding = 4,
    /// Summoned by another mob (e.g., evoker summoning vexes).
    MobSummoned = 5,
    /// Spawned as a jockey (rider on another entity).
    Jockey = 6,
    /// Spawned by a game event (e.g., warden from sculk).
    Event = 7,
    /// Converted from another entity type (e.g., zombie villager curing).
    Conversion = 8,
    /// Spawned as a reinforcement (e.g., zombie reinforcements).
    Reinforcement = 9,
    /// Triggered spawn (e.g., by player proximity or action).
    Triggered = 10,
    /// Spawned from a water bucket (e.g., bucket of fish).
    Bucket = 11,
    /// Spawned by using a spawn item (e.g., spawn egg).
    SpawnItemUse = 12,
    /// Spawned by a command (`/summon`).
    Command = 13,
    /// Spawned by a dispenser (e.g., dispensing a spawn egg).
    Dispenser = 14,
    /// Spawned as part of a patrol (e.g., pillager patrol).
    Patrol = 15,
    /// Spawned by a trial spawner in a trial chamber.
    TrialSpawner = 16,
    /// Loaded from saved data (chunk load, world load).
    Load = 17,
    /// Arrived via dimension travel (e.g., nether portal).
    DimensionTravel = 18,
}

impl_protocol_enum! {
    EntitySpawnReason {
        Natural             = 0  => "natural",
        ChunkGeneration     = 1  => "chunk_generation",
        Spawner             = 2  => "spawner",
        Structure           = 3  => "structure",
        Breeding            = 4  => "breeding",
        MobSummoned         = 5  => "mob_summoned",
        Jockey              = 6  => "jockey",
        Event               = 7  => "event",
        Conversion          = 8  => "conversion",
        Reinforcement       = 9  => "reinforcement",
        Triggered           = 10 => "triggered",
        Bucket              = 11 => "bucket",
        SpawnItemUse        = 12 => "spawn_item_use",
        Command             = 13 => "command",
        Dispenser           = 14 => "dispenser",
        Patrol              = 15 => "patrol",
        TrialSpawner        = 16 => "trial_spawner",
        Load                = 17 => "load",
        DimensionTravel     = 18 => "dimension_travel",
    }
}

impl EntitySpawnReason {
    /// All variants in ordinal order.
    pub const ALL: [EntitySpawnReason; 19] = [
        Self::Natural,
        Self::ChunkGeneration,
        Self::Spawner,
        Self::Structure,
        Self::Breeding,
        Self::MobSummoned,
        Self::Jockey,
        Self::Event,
        Self::Conversion,
        Self::Reinforcement,
        Self::Triggered,
        Self::Bucket,
        Self::SpawnItemUse,
        Self::Command,
        Self::Dispenser,
        Self::Patrol,
        Self::TrialSpawner,
        Self::Load,
        Self::DimensionTravel,
    ];

    /// Returns `true` if this is a spawner-type reason
    /// ([`Spawner`](Self::Spawner) or [`TrialSpawner`](Self::TrialSpawner)).
    pub const fn is_spawner(self) -> bool {
        matches!(self, Self::Spawner | Self::TrialSpawner)
    }

    /// Returns `true` if entities with this spawn reason ignore
    /// light-level requirements for spawning.
    ///
    /// Currently only [`TrialSpawner`](Self::TrialSpawner) ignores
    /// light requirements.
    pub const fn ignores_light_requirements(self) -> bool {
        matches!(self, Self::TrialSpawner)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use bytes::BytesMut;

    use super::*;

    // ── by_id ───────────────────────────────────────────────────────

    #[test]
    fn test_entity_spawn_reason_by_id_all() {
        for id in 0..=18 {
            let reason = EntitySpawnReason::by_id(id).unwrap();
            assert_eq!(reason.id(), id);
        }
    }

    #[test]
    fn test_entity_spawn_reason_by_id_invalid() {
        assert!(EntitySpawnReason::by_id(-1).is_none());
        assert!(EntitySpawnReason::by_id(19).is_none());
        assert!(EntitySpawnReason::by_id(100).is_none());
    }

    // ── Helper methods ──────────────────────────────────────────────

    #[test]
    fn test_entity_spawn_reason_is_spawner() {
        assert!(EntitySpawnReason::Spawner.is_spawner());
        assert!(EntitySpawnReason::TrialSpawner.is_spawner());
        assert!(!EntitySpawnReason::Natural.is_spawner());
        assert!(!EntitySpawnReason::Command.is_spawner());
        assert!(!EntitySpawnReason::Load.is_spawner());
    }

    #[test]
    fn test_entity_spawn_reason_ignores_light_requirements() {
        assert!(EntitySpawnReason::TrialSpawner.ignores_light_requirements());
        assert!(!EntitySpawnReason::Spawner.ignores_light_requirements());
        assert!(!EntitySpawnReason::Natural.ignores_light_requirements());
    }

    // ── by_name ─────────────────────────────────────────────────────

    #[test]
    fn test_entity_spawn_reason_by_name_valid() {
        assert_eq!(
            EntitySpawnReason::by_name("natural"),
            Some(EntitySpawnReason::Natural)
        );
        assert_eq!(
            EntitySpawnReason::by_name("chunk_generation"),
            Some(EntitySpawnReason::ChunkGeneration)
        );
        assert_eq!(
            EntitySpawnReason::by_name("trial_spawner"),
            Some(EntitySpawnReason::TrialSpawner)
        );
        assert_eq!(
            EntitySpawnReason::by_name("dimension_travel"),
            Some(EntitySpawnReason::DimensionTravel)
        );
    }

    #[test]
    fn test_entity_spawn_reason_by_name_invalid() {
        assert!(EntitySpawnReason::by_name("Natural").is_none());
        assert!(EntitySpawnReason::by_name("unknown").is_none());
        assert!(EntitySpawnReason::by_name("").is_none());
    }

    // ── Display ─────────────────────────────────────────────────────

    #[test]
    fn test_entity_spawn_reason_display() {
        assert_eq!(format!("{}", EntitySpawnReason::Natural), "natural");
        assert_eq!(
            format!("{}", EntitySpawnReason::ChunkGeneration),
            "chunk_generation"
        );
        assert_eq!(
            format!("{}", EntitySpawnReason::TrialSpawner),
            "trial_spawner"
        );
    }

    // ── Wire roundtrip ──────────────────────────────────────────────

    #[test]
    fn test_entity_spawn_reason_wire_roundtrip() {
        for reason in EntitySpawnReason::ALL {
            let mut buf = BytesMut::new();
            reason.write(&mut buf);
            let mut data = buf.freeze();
            let decoded = EntitySpawnReason::read(&mut data).unwrap();
            assert_eq!(decoded, reason, "roundtrip failed for {reason}");
        }
    }

    // ── ALL constant ────────────────────────────────────────────────

    #[test]
    fn test_entity_spawn_reason_all_count() {
        assert_eq!(EntitySpawnReason::ALL.len(), 19);
    }

    #[test]
    fn test_entity_spawn_reason_all_ordinals_sequential() {
        for (i, reason) in EntitySpawnReason::ALL.iter().enumerate() {
            assert_eq!(reason.id(), i as i32);
        }
    }

    // ── Property-based tests ────────────────────────────────────────

    mod prop {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn entity_spawn_reason_id_roundtrip(id in 0i32..19) {
                let reason = EntitySpawnReason::by_id(id).unwrap();
                prop_assert_eq!(reason.id(), id);
            }

            #[test]
            fn entity_spawn_reason_name_roundtrip(id in 0i32..19) {
                let reason = EntitySpawnReason::by_id(id).unwrap();
                let name = reason.name();
                prop_assert_eq!(EntitySpawnReason::by_name(name), Some(reason));
            }
        }
    }

    // ── Snapshot tests ──────────────────────────────────────────────

    mod snapshots {
        use super::*;

        #[test]
        fn snapshot_entity_spawn_reason_display() {
            insta::assert_snapshot!(
                EntitySpawnReason::Natural.to_string(),
                @"natural"
            );
            insta::assert_snapshot!(
                EntitySpawnReason::ChunkGeneration.to_string(),
                @"chunk_generation"
            );
            insta::assert_snapshot!(
                EntitySpawnReason::Spawner.to_string(),
                @"spawner"
            );
            insta::assert_snapshot!(
                EntitySpawnReason::TrialSpawner.to_string(),
                @"trial_spawner"
            );
            insta::assert_snapshot!(
                EntitySpawnReason::DimensionTravel.to_string(),
                @"dimension_travel"
            );
        }
    }
}
