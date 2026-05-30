//! Security module — session validation and authorization checks.
//!
//! This module provides security-related functionality for validating user sessions
//! and enforcing access control. While not traditional "health checks," the validation
//! functions in this module act as liveness checks for the security layer:
//!
//! - [`validate_session_params`] verifies session creation parameters (user ID, TTL)
//! - [`validate_session`] checks if an existing session is still valid and active
//! - [`SessionRecord`] represents a session that can be checked for expiration/activity
//!
//! These validation functions degrade gracefully on failures, returning specific errors
//! rather than panicking. The security module is designed to be non-fatal to overall
//! system health (degraded mode is acceptable if validation fails).
//!
//! See [`docs/security-health-checks.md`](../../../docs/security-health-checks.md)
//! for integration guidance and behavior under degraded conditions.

pub mod session;

pub use session::*;
