//! Telemetry module with input validation, reconnection logic, connection pooling, secure health checks, and metrics optimization.

pub mod connection_pool;
pub mod error_handling;
pub mod health_checks;
pub mod input_validation;
pub mod metrics_optimization;
pub mod reconnection;

pub use connection_pool::{ConnectionPool, PoolConfig, PoolError};
pub use error_handling::{ErrorAction, ErrorHandler, TelemetryError, TelemetryResult};
pub use health_checks::{HealthCheckManager, HealthCheckResult, HealthCheckConfig};
pub use input_validation::InputValidator;
pub use metrics_optimization::{MetricsInstruments, CardinalityLimiter};
pub use reconnection::ReconnectionManager;
