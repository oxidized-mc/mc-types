//! [`Difficulty`] — the difficulty level of the game.
//!
//! Maps to the vanilla `Difficulty` enum used in server-properties,
//! login/join-game packets, and difficulty-change packets.

/// The difficulty level of the game.
///
/// # Wire format
///
/// Encoded as a VarInt (0–3).
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::Difficulty;
///
/// let d = Difficulty::by_id(2).unwrap();
/// assert_eq!(d, Difficulty::Normal);
/// assert_eq!(d.to_string(), "normal");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Difficulty {
    /// Peaceful — no hostile mobs, health regenerates.
    Peaceful = 0,
    /// Easy — hostile mobs deal less damage.
    Easy = 1,
    /// Normal — default difficulty.
    Normal = 2,
    /// Hard — hostile mobs deal more damage, hunger can kill.
    Hard = 3,
}

impl_protocol_enum! {
    Difficulty {
        Peaceful = 0 => "peaceful",
        Easy     = 1 => "easy",
        Normal   = 2 => "normal",
        Hard     = 3 => "hard",
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use bytes::BytesMut;

    use super::*;

    // ── by_id ───────────────────────────────────────────────────────

    #[test]
    fn test_difficulty_by_id_valid() {
        assert_eq!(Difficulty::by_id(0), Some(Difficulty::Peaceful));
        assert_eq!(Difficulty::by_id(1), Some(Difficulty::Easy));
        assert_eq!(Difficulty::by_id(2), Some(Difficulty::Normal));
        assert_eq!(Difficulty::by_id(3), Some(Difficulty::Hard));
    }

    #[test]
    fn test_difficulty_by_id_invalid() {
        assert_eq!(Difficulty::by_id(-1), None);
        assert_eq!(Difficulty::by_id(4), None);
        assert_eq!(Difficulty::by_id(100), None);
    }

    // ── by_name ─────────────────────────────────────────────────────

    #[test]
    fn test_difficulty_by_name_valid() {
        assert_eq!(Difficulty::by_name("peaceful"), Some(Difficulty::Peaceful));
        assert_eq!(Difficulty::by_name("easy"), Some(Difficulty::Easy));
        assert_eq!(Difficulty::by_name("normal"), Some(Difficulty::Normal));
        assert_eq!(Difficulty::by_name("hard"), Some(Difficulty::Hard));
    }

    #[test]
    fn test_difficulty_by_name_invalid() {
        assert_eq!(Difficulty::by_name("Peaceful"), None);
        assert_eq!(Difficulty::by_name("unknown"), None);
        assert_eq!(Difficulty::by_name(""), None);
    }

    // ── Display ─────────────────────────────────────────────────────

    #[test]
    fn test_difficulty_display() {
        assert_eq!(format!("{}", Difficulty::Peaceful), "peaceful");
        assert_eq!(format!("{}", Difficulty::Easy), "easy");
        assert_eq!(format!("{}", Difficulty::Normal), "normal");
        assert_eq!(format!("{}", Difficulty::Hard), "hard");
    }

    // ── Roundtrip ───────────────────────────────────────────────────

    #[test]
    fn test_difficulty_id_roundtrip() {
        for id in 0..=3 {
            let d = Difficulty::by_id(id).unwrap();
            assert_eq!(d.id(), id);
        }
    }

    // ── Wire roundtrip ──────────────────────────────────────────────

    #[test]
    fn test_difficulty_wire_roundtrip() {
        for id in 0..=3 {
            let d = Difficulty::by_id(id).unwrap();
            let mut buf = BytesMut::new();
            d.write(&mut buf);
            let mut data = buf.freeze();
            let decoded = Difficulty::read(&mut data).unwrap();
            assert_eq!(decoded, d);
        }
    }

    // ── Property-based tests ────────────────────────────────────────

    mod prop {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn difficulty_id_roundtrip(id in 0i32..4) {
                let d = Difficulty::by_id(id).unwrap();
                prop_assert_eq!(d.id(), id);
            }
        }
    }

    // ── Snapshot tests ──────────────────────────────────────────────

    mod snapshots {
        use super::*;

        #[test]
        fn snapshot_difficulty_display() {
            insta::assert_snapshot!(Difficulty::Peaceful.to_string(), @"peaceful");
            insta::assert_snapshot!(Difficulty::Easy.to_string(), @"easy");
            insta::assert_snapshot!(Difficulty::Normal.to_string(), @"normal");
            insta::assert_snapshot!(Difficulty::Hard.to_string(), @"hard");
        }
    }
}
