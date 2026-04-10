//! [`SoundSource`] — sound categories for the audio system.
//!
//! Maps to vanilla's `SoundSource` enum used in sound effect packets
//! and the client's volume slider settings.

/// Sound category controlling volume levels and audio routing.
///
/// Each sound in Minecraft belongs to a category that the player can
/// independently adjust via the "Music & Sounds" settings screen.
///
/// # Wire format
///
/// Encoded as a VarInt (0–10) in ordinal order.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::SoundSource;
///
/// let src = SoundSource::by_id(0).unwrap();
/// assert_eq!(src, SoundSource::Master);
/// assert_eq!(src.name(), "master");
/// assert_eq!(src.to_string(), "master");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SoundSource {
    /// Master volume — controls all sound.
    Master = 0,
    /// Background music.
    Music = 1,
    /// Jukebox and note block records.
    Records = 2,
    /// Weather sounds (rain, thunder).
    Weather = 3,
    /// Block interaction sounds (placing, breaking).
    Blocks = 4,
    /// Hostile mob sounds.
    Hostile = 5,
    /// Neutral/passive mob sounds.
    Neutral = 6,
    /// Player sounds (footsteps, damage).
    Players = 7,
    /// Ambient environmental sounds (cave ambience).
    Ambient = 8,
    /// Voice chat.
    Voice = 9,
    /// User interface sounds.
    Ui = 10,
}

impl_protocol_enum! {
    SoundSource {
        Master  = 0  => "master",
        Music   = 1  => "music",
        Records = 2  => "record",
        Weather = 3  => "weather",
        Blocks  = 4  => "block",
        Hostile = 5  => "hostile",
        Neutral = 6  => "neutral",
        Players = 7  => "player",
        Ambient = 8  => "ambient",
        Voice   = 9  => "voice",
        Ui      = 10 => "ui",
    }
}

impl SoundSource {
    /// All variants in ordinal order.
    pub const ALL: [SoundSource; 11] = [
        Self::Master,
        Self::Music,
        Self::Records,
        Self::Weather,
        Self::Blocks,
        Self::Hostile,
        Self::Neutral,
        Self::Players,
        Self::Ambient,
        Self::Voice,
        Self::Ui,
    ];
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use bytes::BytesMut;

    use super::*;

    // ── by_id ───────────────────────────────────────────────────────

    #[test]
    fn test_sound_source_by_id_all() {
        for id in 0..=10 {
            let src = SoundSource::by_id(id).unwrap();
            assert_eq!(src.id(), id);
        }
    }

    #[test]
    fn test_sound_source_by_id_invalid() {
        assert!(SoundSource::by_id(-1).is_none());
        assert!(SoundSource::by_id(11).is_none());
        assert!(SoundSource::by_id(100).is_none());
    }

    // ── Names match vanilla ─────────────────────────────────────────

    #[test]
    fn test_sound_source_names_match_vanilla() {
        assert_eq!(SoundSource::Master.name(), "master");
        assert_eq!(SoundSource::Music.name(), "music");
        assert_eq!(SoundSource::Records.name(), "record");
        assert_eq!(SoundSource::Weather.name(), "weather");
        assert_eq!(SoundSource::Blocks.name(), "block");
        assert_eq!(SoundSource::Hostile.name(), "hostile");
        assert_eq!(SoundSource::Neutral.name(), "neutral");
        assert_eq!(SoundSource::Players.name(), "player");
        assert_eq!(SoundSource::Ambient.name(), "ambient");
        assert_eq!(SoundSource::Voice.name(), "voice");
        assert_eq!(SoundSource::Ui.name(), "ui");
    }

    // ── by_name ─────────────────────────────────────────────────────

    #[test]
    fn test_sound_source_by_name_valid() {
        assert_eq!(SoundSource::by_name("master"), Some(SoundSource::Master));
        assert_eq!(SoundSource::by_name("hostile"), Some(SoundSource::Hostile));
        assert_eq!(SoundSource::by_name("ui"), Some(SoundSource::Ui));
    }

    #[test]
    fn test_sound_source_by_name_invalid() {
        assert!(SoundSource::by_name("Master").is_none());
        assert!(SoundSource::by_name("unknown").is_none());
        assert!(SoundSource::by_name("").is_none());
    }

    // ── Display ─────────────────────────────────────────────────────

    #[test]
    fn test_sound_source_display() {
        assert_eq!(format!("{}", SoundSource::Master), "master");
        assert_eq!(format!("{}", SoundSource::Records), "record");
        assert_eq!(format!("{}", SoundSource::Ui), "ui");
    }

    // ── Wire roundtrip ──────────────────────────────────────────────

    #[test]
    fn test_sound_source_wire_roundtrip() {
        for src in SoundSource::ALL {
            let mut buf = BytesMut::new();
            src.write(&mut buf);
            let mut data = buf.freeze();
            let decoded = SoundSource::read(&mut data).unwrap();
            assert_eq!(decoded, src, "roundtrip failed for {src}");
        }
    }

    // ── ALL constant ────────────────────────────────────────────────

    #[test]
    fn test_sound_source_all_count() {
        assert_eq!(SoundSource::ALL.len(), 11);
    }

    #[test]
    fn test_sound_source_all_ordinals_sequential() {
        for (i, src) in SoundSource::ALL.iter().enumerate() {
            assert_eq!(src.id(), i as i32);
        }
    }

    // ── Property-based tests ────────────────────────────────────────

    mod prop {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn sound_source_id_roundtrip(id in 0i32..11) {
                let src = SoundSource::by_id(id).unwrap();
                prop_assert_eq!(src.id(), id);
            }

            #[test]
            fn sound_source_name_roundtrip(id in 0i32..11) {
                let src = SoundSource::by_id(id).unwrap();
                let name = src.name();
                prop_assert_eq!(SoundSource::by_name(name), Some(src));
            }
        }
    }

    // ── Snapshot tests ──────────────────────────────────────────────

    mod snapshots {
        use super::*;

        #[test]
        fn snapshot_sound_source_display() {
            insta::assert_snapshot!(SoundSource::Master.to_string(), @"master");
            insta::assert_snapshot!(SoundSource::Music.to_string(), @"music");
            insta::assert_snapshot!(SoundSource::Records.to_string(), @"record");
            insta::assert_snapshot!(SoundSource::Weather.to_string(), @"weather");
            insta::assert_snapshot!(SoundSource::Blocks.to_string(), @"block");
            insta::assert_snapshot!(SoundSource::Hostile.to_string(), @"hostile");
            insta::assert_snapshot!(SoundSource::Neutral.to_string(), @"neutral");
            insta::assert_snapshot!(SoundSource::Players.to_string(), @"player");
            insta::assert_snapshot!(SoundSource::Ambient.to_string(), @"ambient");
            insta::assert_snapshot!(SoundSource::Voice.to_string(), @"voice");
            insta::assert_snapshot!(SoundSource::Ui.to_string(), @"ui");
        }
    }
}
