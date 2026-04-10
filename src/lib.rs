//! Minecraft game types — BlockPos, SectionPos, Vec3, Aabb, Direction,
//! ResourceLocation, GameType, and more.

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
pub mod equipment_slot;
pub mod error;
pub mod game_type;
pub mod global_pos;
pub mod hit_result;
pub mod humanoid_arm;
pub mod interaction_hand;
pub mod interaction_result;
pub mod mth;
pub mod particle_status;
pub mod pose;
pub mod resource_key;
pub mod resource_location;
pub mod rotations;
pub mod section_pos;
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
pub use equipment_slot::{EquipmentSlot, EquipmentSlotType};
pub use error::McTypesError;
pub use game_type::GameType;
pub use global_pos::GlobalPos;
pub use hit_result::{BlockHitResult, HitResultType};
pub use humanoid_arm::HumanoidArm;
pub use interaction_hand::InteractionHand;
pub use interaction_result::{InteractionResult, SwingSource};
pub use particle_status::ParticleStatus;
pub use pose::Pose;
pub use resource_key::ResourceKey;
pub use resource_location::ResourceLocation;
pub use rotations::Rotations;
pub use section_pos::SectionPos;
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec3i::Vec3i;
