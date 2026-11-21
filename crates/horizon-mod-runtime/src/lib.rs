//! Engine-independent storage and registration machinery shared by the game and mod API.

/// Longest production guest-call budget, in milliseconds (one epoch tick is about 1 ms). The
/// budget counts the calling thread's CPU time where the platform has a thread clock.
pub const MAX_GUEST_CALL_BUDGET_MILLIS: u64 = 30_000;

/// Longest wall time any guest call may run, in milliseconds: the backstop for a call that stays
/// under its CPU budget while it waits (in blocking host calls, or for the system to run it).
/// Payload ingress deadlines must exceed this so a load call cannot exhaust a queued payload.
pub const MAX_GUEST_CALL_WALL_MILLIS: u64 = 45_000;

pub mod attachments;
pub mod byte_channel;
/// A level's capability generations, which kept capabilities compare (arismoko/horizon#3446).
pub mod capability_generations;
pub mod dispatch_failures;
/// Load-owned registry names and numeric slots, without game or API key types.
pub mod registry_keys;

pub mod block_definitions;
pub mod block_entities;
/// Immutable hook subscription bitsets, resolved once at installation.
pub mod subscribers;
/// Session-only watered-area indexes, shared by server gameplay and the mod disable boundary.
pub mod water_tickets;
