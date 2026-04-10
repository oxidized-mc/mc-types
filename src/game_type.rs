//! [`GameType`] — the game mode for a player.
//!
//! Maps to the vanilla `GameType` enum used in login packets,
//! player info updates, and game event packets.

/// The game mode for a player.
///
/// # Wire format
///
/// Encoded as a VarInt (0–3).
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::GameType;
///
/// let gt = GameType::by_id(0).unwrap();
/// assert_eq!(gt, GameType::Survival);
/// assert_eq!(gt.id(), 0);
/// assert_eq!(gt.to_string(), "survival");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GameType {
    /// Survival mode — the player can take damage and must gather resources.
    Survival = 0,
    /// Creative mode — the player has unlimited resources and cannot take damage.
    Creative = 1,
    /// Adventure mode — the player can take damage but cannot break/place blocks
    /// freely.
    Adventure = 2,
    /// Spectator mode — the player is invisible and can fly through blocks.
    Spectator = 3,
}

impl_protocol_enum! {
    GameType {
        Survival  = 0 => "survival",
        Creative  = 1 => "creative",
        Adventure = 2 => "adventure",
        Spectator = 3 => "spectator",
    }
}

impl GameType {
    /// Returns the vanilla translation key for this game type (e.g.,
    /// `"gameMode.survival"`).
    pub const fn translation_key(self) -> &'static str {
        match self {
            GameType::Survival => "gameMode.survival",
            GameType::Creative => "gameMode.creative",
            GameType::Adventure => "gameMode.adventure",
            GameType::Spectator => "gameMode.spectator",
        }
    }

    /// Returns `true` if this is [`GameType::Creative`].
    pub const fn is_creative(self) -> bool {
        matches!(self, GameType::Creative)
    }

    /// Returns `true` if this is a "survival-like" mode where the player
    /// can take damage and needs food ([`GameType::Survival`] or
    /// [`GameType::Adventure`]).
    pub const fn is_survival(self) -> bool {
        matches!(self, GameType::Survival | GameType::Adventure)
    }

    /// Returns `true` if block placing is restricted in this game mode
    /// ([`GameType::Adventure`] or [`GameType::Spectator`]).
    pub const fn is_block_placing_restricted(self) -> bool {
        matches!(self, GameType::Adventure | GameType::Spectator)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use bytes::BytesMut;

    use super::*;

    // ── by_id ───────────────────────────────────────────────────────

    #[test]
    fn test_game_type_by_id_valid() {
        assert_eq!(GameType::by_id(0), Some(GameType::Survival));
        assert_eq!(GameType::by_id(1), Some(GameType::Creative));
        assert_eq!(GameType::by_id(2), Some(GameType::Adventure));
        assert_eq!(GameType::by_id(3), Some(GameType::Spectator));
    }

    #[test]
    fn test_game_type_by_id_invalid() {
        assert_eq!(GameType::by_id(-1), None);
        assert_eq!(GameType::by_id(4), None);
        assert_eq!(GameType::by_id(100), None);
    }

    // ── by_name ─────────────────────────────────────────────────────

    #[test]
    fn test_game_type_by_name_valid() {
        assert_eq!(GameType::by_name("survival"), Some(GameType::Survival));
        assert_eq!(GameType::by_name("creative"), Some(GameType::Creative));
        assert_eq!(GameType::by_name("adventure"), Some(GameType::Adventure));
        assert_eq!(GameType::by_name("spectator"), Some(GameType::Spectator));
    }

    #[test]
    fn test_game_type_by_name_invalid() {
        assert_eq!(GameType::by_name("Survival"), None);
        assert_eq!(GameType::by_name("unknown"), None);
        assert_eq!(GameType::by_name(""), None);
    }

    // ── Roundtrip id ↔ enum ─────────────────────────────────────────

    #[test]
    fn test_game_type_id_roundtrip() {
        for id in 0..=3 {
            let gt = GameType::by_id(id).unwrap();
            assert_eq!(gt.id(), id);
        }
    }

    // ── Boolean predicates ──────────────────────────────────────────

    #[test]
    fn test_game_type_is_creative() {
        assert!(!GameType::Survival.is_creative());
        assert!(GameType::Creative.is_creative());
        assert!(!GameType::Adventure.is_creative());
        assert!(!GameType::Spectator.is_creative());
    }

    #[test]
    fn test_game_type_is_survival() {
        assert!(GameType::Survival.is_survival());
        assert!(!GameType::Creative.is_survival());
        assert!(GameType::Adventure.is_survival());
        assert!(!GameType::Spectator.is_survival());
    }

    #[test]
    fn test_game_type_is_block_placing_restricted() {
        assert!(!GameType::Survival.is_block_placing_restricted());
        assert!(!GameType::Creative.is_block_placing_restricted());
        assert!(GameType::Adventure.is_block_placing_restricted());
        assert!(GameType::Spectator.is_block_placing_restricted());
    }

    // ── Translation keys ──────────────────────────────────────────

    #[test]
    fn test_game_type_translation_key() {
        assert_eq!(GameType::Survival.translation_key(), "gameMode.survival");
        assert_eq!(GameType::Creative.translation_key(), "gameMode.creative");
        assert_eq!(GameType::Adventure.translation_key(), "gameMode.adventure");
        assert_eq!(GameType::Spectator.translation_key(), "gameMode.spectator");
    }

    // ── Display ─────────────────────────────────────────────────────

    #[test]
    fn test_game_type_display() {
        assert_eq!(format!("{}", GameType::Survival), "survival");
        assert_eq!(format!("{}", GameType::Creative), "creative");
        assert_eq!(format!("{}", GameType::Adventure), "adventure");
        assert_eq!(format!("{}", GameType::Spectator), "spectator");
    }

    // ── Wire roundtrip ──────────────────────────────────────────────

    #[test]
    fn test_game_type_wire_roundtrip() {
        for id in 0..=3 {
            let gt = GameType::by_id(id).unwrap();
            let mut buf = BytesMut::new();
            gt.write(&mut buf);
            let mut data = buf.freeze();
            let decoded = GameType::read(&mut data).unwrap();
            assert_eq!(decoded, gt);
        }
    }

    // ── Property-based tests ────────────────────────────────────────

    mod prop {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn game_type_id_roundtrip(id in 0i32..4) {
                let gt = GameType::by_id(id).unwrap();
                prop_assert_eq!(gt.id(), id);
            }
        }
    }

    // ── Snapshot tests ──────────────────────────────────────────────

    mod snapshots {
        use super::*;

        #[test]
        fn snapshot_game_type_display() {
            insta::assert_snapshot!(GameType::Survival.to_string(), @"survival");
            insta::assert_snapshot!(GameType::Creative.to_string(), @"creative");
            insta::assert_snapshot!(GameType::Adventure.to_string(), @"adventure");
            insta::assert_snapshot!(GameType::Spectator.to_string(), @"spectator");
        }
    }
}
