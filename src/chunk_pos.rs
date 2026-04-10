//! [`ChunkPos`] extensions for the Minecraft wire protocol.
//!
//! The core [`ChunkPos`] type lives in [`oxidized_types`]. This module
//! adds wire-format read/write and [`BlockPos`] conversion via [`ChunkPosExt`].

use bytes::{Bytes, BytesMut};

use oxidized_codec::types::{self, TypeError};
use oxidized_types::ChunkPos;

use crate::block_pos::BlockPos;

/// Wire-format extension methods for [`ChunkPos`].
pub trait ChunkPosExt {
    /// Returns the chunk containing the given [`BlockPos`].
    fn from_block_pos(pos: &BlockPos) -> ChunkPos;

    /// Returns the block position at the center of this chunk at the given Y.
    ///
    /// Equivalent to vanilla's `getMiddleBlockPosition(int y)`.
    fn get_middle_block_position(&self, y: i32) -> BlockPos;

    /// Returns the block position at the min corner of this chunk at the given Y.
    ///
    /// Equivalent to vanilla's `getWorldPosition()`.
    fn get_world_position(&self) -> BlockPos;

    /// Reads a [`ChunkPos`] from a wire buffer (packed `i64`).
    ///
    /// # Errors
    ///
    /// Returns [`TypeError`] if fewer than 8 bytes remain.
    fn read(buf: &mut Bytes) -> Result<ChunkPos, TypeError>;

    /// Writes this [`ChunkPos`] to a wire buffer (packed `i64`).
    fn write(&self, buf: &mut BytesMut);
}

impl ChunkPosExt for ChunkPos {
    fn from_block_pos(pos: &BlockPos) -> ChunkPos {
        ChunkPos::new(pos.x >> 4, pos.z >> 4)
    }

    fn get_middle_block_position(&self, y: i32) -> BlockPos {
        BlockPos::new(self.middle_block_x(), y, self.middle_block_z())
    }

    fn get_world_position(&self) -> BlockPos {
        BlockPos::new(self.min_block_x(), 0, self.min_block_z())
    }

    fn read(buf: &mut Bytes) -> Result<ChunkPos, TypeError> {
        let packed = types::read_i64(buf)?;
        Ok(ChunkPos::from_long(packed))
    }

    fn write(&self, buf: &mut BytesMut) {
        types::write_i64(buf, self.as_long());
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // ── from_block_pos ──────────────────────────────────────────────

    #[test]
    fn test_chunk_pos_from_block_pos_positive() {
        let block = BlockPos::new(32, 64, 48);
        assert_eq!(ChunkPos::from_block_pos(&block), ChunkPos::new(2, 3));
    }

    #[test]
    fn test_chunk_pos_from_block_pos_negative() {
        let block = BlockPos::new(-1, 0, -1);
        assert_eq!(ChunkPos::from_block_pos(&block), ChunkPos::new(-1, -1));
    }

    #[test]
    fn test_chunk_pos_from_block_pos_boundary_zero() {
        let block = BlockPos::new(0, 0, 0);
        assert_eq!(ChunkPos::from_block_pos(&block), ChunkPos::new(0, 0));
    }

    #[test]
    fn test_chunk_pos_from_block_pos_boundary_15() {
        let block = BlockPos::new(15, 0, 15);
        assert_eq!(ChunkPos::from_block_pos(&block), ChunkPos::new(0, 0));
    }

    #[test]
    fn test_chunk_pos_from_block_pos_boundary_16() {
        let block = BlockPos::new(16, 0, 16);
        assert_eq!(ChunkPos::from_block_pos(&block), ChunkPos::new(1, 1));
    }

    // ── Wire roundtrip ──────────────────────────────────────────────

    #[test]
    fn test_chunk_pos_wire_roundtrip() {
        let pos = ChunkPos::new(100, -200);
        let mut buf = BytesMut::new();
        pos.write(&mut buf);
        assert_eq!(buf.len(), 8);
        let mut data = buf.freeze();
        let decoded = ChunkPos::read(&mut data).unwrap();
        assert_eq!(decoded, pos);
    }

    #[test]
    fn test_chunk_pos_wire_roundtrip_zero() {
        let pos = ChunkPos::ZERO;
        let mut buf = BytesMut::new();
        pos.write(&mut buf);
        let mut data = buf.freeze();
        let decoded = ChunkPos::read(&mut data).unwrap();
        assert_eq!(decoded, pos);
    }

    // ── get_middle_block_position ───────────────────────────────────────

    #[test]
    fn test_chunk_pos_get_middle_block_position() {
        let chunk = ChunkPos::new(0, 0);
        let mid = chunk.get_middle_block_position(64);
        assert_eq!(mid, BlockPos::new(7, 64, 7));
    }

    #[test]
    fn test_chunk_pos_get_middle_block_position_negative() {
        let chunk = ChunkPos::new(-1, -1);
        let mid = chunk.get_middle_block_position(0);
        // middle_block_x for chunk -1 = -16 + 7 = -9
        assert_eq!(mid, BlockPos::new(-9, 0, -9));
    }

    // ── get_world_position ──────────────────────────────────────────────

    #[test]
    fn test_chunk_pos_get_world_position() {
        let chunk = ChunkPos::new(2, 3);
        let world = chunk.get_world_position();
        assert_eq!(world, BlockPos::new(32, 0, 48));
    }

    #[test]
    fn test_chunk_pos_get_world_position_zero() {
        let chunk = ChunkPos::ZERO;
        let world = chunk.get_world_position();
        assert_eq!(world, BlockPos::new(0, 0, 0));
    }
}
