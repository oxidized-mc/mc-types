//! [`BlockState`] — a lightweight block state identifier.
//!
//! Maps to vanilla's global palette ID. The full property system and data
//! table live in the `registry` crate; mc-types only provides the ID type.

use bytes::{Bytes, BytesMut};

use oxidized_codec::types::TypeError;
use oxidized_codec::varint;

/// A block state identifier mapping to vanilla's global palette ID.
///
/// The actual block data (properties, flags, collision shapes) is resolved
/// through the registry crate's lookup tables.
///
/// # Wire format
///
/// Encoded as a VarInt.
///
/// # Examples
///
/// ```
/// use oxidized_mc_types::BlockState;
///
/// let air = BlockState::AIR;
/// assert_eq!(air.id(), 0);
///
/// let stone = BlockState::new(1);
/// assert_eq!(stone.id(), 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlockState(u16);

impl BlockState {
    /// The default air block state (ID 0).
    pub const AIR: BlockState = BlockState(0);

    /// Creates a new `BlockState` from a raw palette ID.
    pub const fn new(id: u16) -> Self {
        Self(id)
    }

    /// Returns the raw palette ID.
    pub const fn id(self) -> u16 {
        self.0
    }

    /// Reads a `BlockState` from a wire buffer as a VarInt.
    ///
    /// # Errors
    ///
    /// Returns [`TypeError`] if the buffer is truncated or the VarInt value
    /// is outside the valid `u16` range (`0..=65535`).
    pub fn read(buf: &mut Bytes) -> Result<Self, TypeError> {
        let id = varint::read_varint_buf(buf)?;
        let id = u16::try_from(id).map_err(|_| TypeError::InvalidValue { value: id })?;
        Ok(Self(id))
    }

    /// Writes this `BlockState` to a wire buffer as a VarInt.
    pub fn write(&self, buf: &mut BytesMut) {
        varint::write_varint_buf(i32::from(self.0), buf);
    }
}

impl std::fmt::Display for BlockState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BlockState({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use bytes::{Bytes, BytesMut};

    use super::*;

    // ── Construction ────────────────────────────────────────────────────

    #[test]
    fn test_block_state_air_is_zero() {
        assert_eq!(BlockState::AIR.id(), 0);
    }

    #[test]
    fn test_block_state_new_and_id() {
        let state = BlockState::new(42);
        assert_eq!(state.id(), 42);
    }

    #[test]
    fn test_block_state_equality() {
        assert_eq!(BlockState::new(1), BlockState::new(1));
        assert_ne!(BlockState::new(1), BlockState::new(2));
    }

    #[test]
    fn test_block_state_ordering() {
        assert!(BlockState::new(0) < BlockState::new(1));
        assert!(BlockState::new(100) > BlockState::new(50));
    }

    // ── Wire roundtrip ─────────────────────────────────────────────────

    #[test]
    fn test_block_state_wire_roundtrip_zero() {
        let original = BlockState::AIR;
        let mut buf = BytesMut::new();
        original.write(&mut buf);
        let mut read_buf = Bytes::from(buf);
        let decoded = BlockState::read(&mut read_buf).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_block_state_wire_roundtrip_nonzero() {
        let original = BlockState::new(1234);
        let mut buf = BytesMut::new();
        original.write(&mut buf);
        let mut read_buf = Bytes::from(buf);
        let decoded = BlockState::read(&mut read_buf).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_block_state_wire_roundtrip_max() {
        let original = BlockState::new(u16::MAX);
        let mut buf = BytesMut::new();
        original.write(&mut buf);
        let mut read_buf = Bytes::from(buf);
        let decoded = BlockState::read(&mut read_buf).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_block_state_read_empty_buffer() {
        let mut buf = Bytes::new();
        assert!(BlockState::read(&mut buf).is_err());
    }

    // ── Display ────────────────────────────────────────────────────────

    #[test]
    fn test_block_state_display() {
        assert_eq!(format!("{}", BlockState::new(42)), "BlockState(42)");
    }

    // ── Property-based tests ────────────────────────────────────────

    mod prop {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn block_state_wire_roundtrip(id: u16) {
                let bs = BlockState::new(id);
                let mut buf = BytesMut::new();
                bs.write(&mut buf);
                let mut data = Bytes::from(buf);
                let decoded = BlockState::read(&mut data).unwrap();
                prop_assert_eq!(decoded, bs);
            }
        }
    }

    // ── Snapshot tests ──────────────────────────────────────────────

    mod snapshots {
        use super::*;

        #[test]
        fn snapshot_block_state_display() {
            insta::assert_snapshot!(BlockState::new(0).to_string(), @"BlockState(0)");
            insta::assert_snapshot!(BlockState::new(42).to_string(), @"BlockState(42)");
        }
    }
}
