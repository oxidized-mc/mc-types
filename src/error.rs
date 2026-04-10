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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    #[test]
    fn test_invalid_resource_location_display() {
        let err = McTypesError::InvalidResourceLocation("bad:bad:bad".into());
        assert_eq!(err.to_string(), "invalid resource location: bad:bad:bad");
    }

    #[test]
    fn test_unknown_enum_variant_display() {
        let err = McTypesError::UnknownEnumVariant {
            enum_name: "GameType",
            id: 99,
        };
        assert_eq!(err.to_string(), "unknown enum variant id 99 for GameType");
    }

    #[test]
    fn test_coordinate_out_of_range_display() {
        let err = McTypesError::CoordinateOutOfRange("x=99999999".into());
        assert_eq!(err.to_string(), "coordinate out of range: x=99999999");
    }

    #[test]
    fn test_wire_error_from_type_error() {
        let te = TypeError::UnexpectedEof { need: 8, have: 3 };
        let err = McTypesError::from(te);
        assert!(
            err.to_string()
                .contains("unexpected end of buffer (need 8, have 3)")
        );
    }
}
