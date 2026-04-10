# oxidized-mc-types

[![CI](https://github.com/oxidized-mc/mc-types/actions/workflows/ci.yml/badge.svg)](https://github.com/oxidized-mc/mc-types/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Foundational Minecraft game types for the [Oxidized MC](https://github.com/oxidized-mc)
ecosystem — a Minecraft Java Edition implementation in Rust.

This crate provides type-safe coordinate systems, geometry primitives, game enums,
and resource identifiers used throughout the Minecraft protocol and game logic.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
oxidized-mc-types = "0.1.0"
```

```rust
use oxidized_mc_types::{BlockPos, SectionPos, Vec3, Direction, ResourceLocation};

// Block positions with type-safe coordinate conversions
let block = BlockPos::new(100, 64, -200);
let section = SectionPos::from_block_pos(&block);
let chunk = block.chunk_pos();

// Entity positions with floating-point precision
let pos = Vec3::new(100.5, 64.0, -200.5);
let moved = pos.add_vec(&Vec3::new(0.0, 1.0, 0.0));

// Namespaced identifiers
let loc = ResourceLocation::new("minecraft", "stone").unwrap();
assert_eq!(loc.to_string(), "minecraft:stone");

// Directions for block adjacency
let facing = Direction::North;
let opposite = facing.opposite();
```

## Type Reference

### Coordinates

| Type | Description | Components |
|------|-------------|------------|
| `BlockPos` | Block position in world space | `x, y, z: i32` |
| `SectionPos` | Chunk section position | `x, y, z: i32` |
| `GlobalPos` | Block position + dimension key | `BlockPos` + `ResourceKey` |
| `ChunkPosExt` | Extension trait for `ChunkPos` | `x, z: i32` |
| `Vec3` | Entity position / velocity | `x, y, z: f64` |
| `Vec3i` | Integer 3D vector | `x, y, z: i32` |
| `Vec2` | 2D float vector | `x, y: f32` |

### Geometry

| Type | Description |
|------|-------------|
| `Aabb` | Axis-aligned bounding box for collision detection and spatial queries |
| `EntityDimensions` | Width/height dimensions for entity hitboxes |
| `Rotations` | 3-float rotation for entity parts (head, arms, legs) |

### Game Enums

| Type | Description | Wire Format |
|------|-------------|-------------|
| `GameType` | Survival / Creative / Adventure / Spectator | VarInt (0–3) |
| `Difficulty` | Peaceful / Easy / Normal / Hard | VarInt (0–3) |
| `Direction` | Down / Up / North / South / West / East | VarInt (0–5) |
| `Pose` | Entity pose (standing, sneaking, swimming, etc.) | VarInt (0–17) |
| `ChatVisibility` | Full / System / Hidden | VarInt (0–2) |
| `HumanoidArm` | Left / Right | VarInt (0–1) |
| `ParticleStatus` | All / Decreased / Minimal | VarInt (0–2) |
| `EquipmentSlot` | Main hand, off hand, armor slots, body | VarInt (0–7) |
| `InteractionHand` | Main hand / Off hand | VarInt (0–1) |

### Identifiers

| Type | Description |
|------|-------------|
| `ResourceLocation` | Namespaced identifier (`minecraft:stone`) |
| `ResourceKey<T>` | Typed registry key binding a `ResourceLocation` to a specific registry |

### Interaction & Raycasting

| Type | Description |
|------|-------------|
| `BlockHitResult` | Result of a block raycast (location, face, block position) |
| `HitResultType` | Miss / Block / Entity discriminant |
| `InteractionResult` | Outcome of a block or entity interaction |
| `BlockState` | Opaque block state identifier (data resolution is the registry's job) |

## Coordinate Conversions

```text
   Vec3 (f64)
     │
     ▼ floor components
   BlockPos (i32)
     │
     ├──► SectionPos    (>> 4 on each axis)
     │
     └──► ChunkPos      (>> 4 on x/z, drops y)
           │
           └──► SectionPos  (with explicit y section)
```

All conversions are provided as methods — use `block_pos.chunk_pos()`,
`SectionPos::from_block_pos()`, etc. Never perform manual bit-shifts.

## Wire Format

All game enums implement `read(&mut Bytes)` and `write(&self, &mut BytesMut)`
for protocol wire encoding (VarInt). Coordinate types (`BlockPos`, `SectionPos`)
support packed `i64` encoding matching the vanilla protocol format.

## For Downstream Crate Authors

- **Import types, not modules**: `use oxidized_mc_types::BlockPos;`
- **Use conversion methods**: `block_pos.chunk_pos()` instead of manual `>> 4` shifts
- **Don't match on struct fields**: Field layout may change in future versions
- **Check `McTypesError`**: Construction/validation errors are returned via this enum

## Contributing

Contributions are welcome! Please see the [Oxidized MC](https://github.com/oxidized-mc)
organization for project-wide guidelines.

## License

Licensed under the [MIT License](LICENSE).
