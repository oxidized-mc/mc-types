//! [`InteractionResult`] and [`SwingSource`] — outcome of a block or entity interaction.
//!
//! In vanilla 26.1, `InteractionResult` is a sealed interface with record
//! variants. We model it as a Rust enum — the wire protocol doesn't transmit
//! this type directly, but it governs server-side interaction logic.

/// Source of the arm-swing animation after a successful interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwingSource {
    /// No swing animation.
    None,
    /// Client-initiated swing.
    Client,
    /// Server-initiated swing.
    Server,
}

impl std::fmt::Display for SwingSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SwingSource::None => f.write_str("none"),
            SwingSource::Client => f.write_str("client"),
            SwingSource::Server => f.write_str("server"),
        }
    }
}

/// Result of an interaction attempt.
///
/// In vanilla 26.1 this is a sealed interface with record variants.
/// The wire protocol doesn't transmit this type directly, but it governs
/// server-side interaction logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InteractionResult {
    /// Interaction succeeded. Contains the swing source for animation.
    Success {
        /// How the arm-swing animation is triggered.
        swing_source: SwingSource,
    },
    /// Interaction explicitly failed.
    Fail,
    /// Interaction did nothing — try next handler.
    Pass,
    /// Try the interaction again with an empty hand.
    TryEmptyHandInteraction,
}

impl InteractionResult {
    /// Standard success — client-initiated swing.
    pub const SUCCESS: Self = Self::Success {
        swing_source: SwingSource::Client,
    };

    /// Server-initiated success.
    pub const SUCCESS_SERVER: Self = Self::Success {
        swing_source: SwingSource::Server,
    };

    /// Consume the action without swinging.
    pub const CONSUME: Self = Self::Success {
        swing_source: SwingSource::None,
    };

    /// Whether this result consumes the action (only `Success` variants do).
    pub fn consumes_action(&self) -> bool {
        matches!(self, Self::Success { .. })
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    // ── Constants ────────────────────────────────────────────────────────

    #[test]
    fn test_interaction_result_success_constant() {
        assert_eq!(
            InteractionResult::SUCCESS,
            InteractionResult::Success {
                swing_source: SwingSource::Client
            }
        );
    }

    #[test]
    fn test_interaction_result_success_server_constant() {
        assert_eq!(
            InteractionResult::SUCCESS_SERVER,
            InteractionResult::Success {
                swing_source: SwingSource::Server
            }
        );
    }

    #[test]
    fn test_interaction_result_consume_constant() {
        assert_eq!(
            InteractionResult::CONSUME,
            InteractionResult::Success {
                swing_source: SwingSource::None
            }
        );
    }

    // ── consumes_action ─────────────────────────────────────────────────

    #[test]
    fn test_consumes_action_success() {
        assert!(InteractionResult::SUCCESS.consumes_action());
        assert!(InteractionResult::SUCCESS_SERVER.consumes_action());
        assert!(InteractionResult::CONSUME.consumes_action());
    }

    #[test]
    fn test_consumes_action_non_success() {
        assert!(!InteractionResult::Fail.consumes_action());
        assert!(!InteractionResult::Pass.consumes_action());
        assert!(!InteractionResult::TryEmptyHandInteraction.consumes_action());
    }

    // ── Equality ────────────────────────────────────────────────────────

    #[test]
    fn test_interaction_result_equality() {
        assert_eq!(InteractionResult::Fail, InteractionResult::Fail);
        assert_eq!(InteractionResult::Pass, InteractionResult::Pass);
        assert_ne!(InteractionResult::Fail, InteractionResult::Pass);
        assert_ne!(
            InteractionResult::SUCCESS,
            InteractionResult::SUCCESS_SERVER
        );
    }
}
