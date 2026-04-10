//! Error types for `oxidized-mc-types`.
//!
//! Separates construction/validation errors ([`McTypesError`]) from
//! wire-format codec errors ([`TypeError`]).

use oxidized_codec::types::TypeError;

/// Errors from constructing or validating Minecraft types.
///
/// Wire-format read/write operations return
/// [`TypeError`] directly.
/// This enum covers construction-time and lookup failures.
#[derive(Debug, thiserror::Error)]
pub enum McTypesError {
    /// A resource location string failed validation.
    #[error("invalid resource location: {0}")]
    InvalidResourceLocation(String),

    /// An integer ID did not map to any known enum variant.
    #[error("unknown enum variant id {id} for {enum_name}")]
    UnknownEnumVariant {
        /// The name of the enum type.
        enum_name: &'static str,
        /// The unrecognized variant ID.
        id: i32,
    },

    /// A coordinate value was outside the representable range.
    #[error("coordinate out of range: {0}")]
    CoordinateOutOfRange(String),

    /// A wire-format codec error.
    #[error(transparent)]
    Wire(#[from] TypeError),
}
