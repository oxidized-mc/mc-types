//! # oxidized-mc-types
//!
//! Foundational Minecraft game types for the Oxidized MC ecosystem.
//!
//! This crate provides type-safe coordinate systems, geometry primitives,
//! game enums, and resource identifiers used throughout the Minecraft
//! protocol and game logic.
//!
//! ## Coordinate Types
//!
//! | Type | Description | Dimensionality |
//! |------|-------------|----------------|
//! | [`BlockPos`] | Block position in world space | 3D (i32) |
//! | [`ChunkPosExt`] | Chunk column position (extension trait) | 2D (i32) |
//! | [`SectionPos`] | Chunk section position | 3D (i32) |
//! | [`GlobalPos`] | Block position + dimension identifier | 3D (i32) + key |
//! | [`Vec3`] | Entity position / velocity | 3D (f64) |
//! | [`Vec3i`] | Integer 3D vector | 3D (i32) |
//! | [`Vec2`] | 2D float vector | 2D (f32) |
//!
//! All coordinate types are distinct newtypes preventing accidental
//! mixing of block, chunk, and section coordinates at compile time.
//!
//! ## Geometry
//!
//! [`Aabb`] provides axis-aligned bounding boxes for collision detection
//! and spatial queries, including ray intersection via
//! [`Aabb::clip`](aabb::Aabb::clip).
//!
//! ## Game Enums
//!
//! Protocol-compatible enums with wire format support:
//! [`GameType`], [`Difficulty`], [`Direction`], [`ChatVisibility`],
//! [`HumanoidArm`], [`ParticleStatus`], [`Pose`], [`EquipmentSlot`],
//! [`InteractionHand`], [`MobCategory`], [`SoundSource`],
//! [`EntitySpawnReason`].
//!
//! ## Identifiers
//!
//! [`ResourceLocation`] — namespaced identifiers (`minecraft:stone`)
//! used throughout the Minecraft data model.
//!
//! [`ResourceKey<T>`](ResourceKey) — typed registry key binding a
//! [`ResourceLocation`] to a specific registry.
//!
//! ## Interaction & Hit Results
//!
//! [`InteractionResult`] and [`BlockHitResult`] model server-side
//! interaction outcomes and raycasting results.
//!
//! ## Math Utilities
//!
//! The [`mth`] module provides floor/ceil/clamp/lerp functions matching
//! vanilla's `Mth` class, preserving Java edge-case behaviour.
//!
//! ## Feature Flags
//!
//! This crate has no optional features — all types are always available.

#![warn(missing_docs)]
#![deny(unsafe_code)]

#[macro_use]
mod type_macros;

pub mod aabb;
pub mod block_pos;
pub mod block_state;
pub mod chat_visibility;
pub mod chunk_pos;
pub mod difficulty;
pub mod direction;
pub mod entity_dimensions;
pub mod entity_spawn_reason;
pub mod equipment_slot;
pub mod error;
pub mod game_type;
pub mod global_pos;
pub mod hit_result;
pub mod humanoid_arm;
pub mod interaction_hand;
pub mod interaction_result;
pub mod mob_category;
pub mod mth;
pub mod particle_status;
pub mod pose;
pub mod resource_key;
pub mod resource_location;
pub mod rotations;
pub mod section_pos;
pub mod sound_source;
pub mod vec2;
pub mod vec3;
pub mod vec3i;

pub use aabb::Aabb;
pub use block_pos::BlockPos;
pub use block_state::BlockState;
pub use chat_visibility::ChatVisibility;
pub use chunk_pos::ChunkPosExt;
pub use difficulty::Difficulty;
pub use direction::{Axis, AxisDirection, Direction, Plane};
pub use entity_dimensions::EntityDimensions;
pub use entity_spawn_reason::EntitySpawnReason;
pub use equipment_slot::{EquipmentSlot, EquipmentSlotType};
pub use error::McTypesError;
pub use game_type::GameType;
pub use global_pos::GlobalPos;
pub use hit_result::{BlockHitResult, HitResultType};
pub use humanoid_arm::HumanoidArm;
pub use interaction_hand::InteractionHand;
pub use interaction_result::{InteractionResult, SwingSource};
pub use mob_category::MobCategory;
pub use particle_status::ParticleStatus;
pub use pose::Pose;
pub use resource_key::ResourceKey;
pub use resource_location::ResourceLocation;
pub use rotations::Rotations;
pub use section_pos::SectionPos;
pub use sound_source::SoundSource;
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec3i::Vec3i;
