use serde::{Deserialize, Serialize};

/// Current quota usage/limit snapshot for a tenant.
///
/// Mirrors the backend's `QuotaStatus` (`src/middleware/quota.rs`).
#[derive(Debug, Clone, Deserialize)]
pub struct QuotaStatus {
    pub limit: u32,
    pub used: u32,
    pub remaining: u32,
    pub reset_in_seconds: u64,
}

/// A tenant's quota configuration and current usage.
///
/// Mirrors the backend's `TenantQuotaView` (`src/handlers/admin/quota.rs`).
/// `quota_status` is `None` if usage tracking is unavailable for this tenant
/// (e.g. the quota backend could not be reached) — not an error on its own.
#[derive(Debug, Clone, Deserialize)]
pub struct TenantQuotaView {
    pub tenant_id: String,
    pub name: String,
    pub rate_limit_per_minute: i32,
    pub quota_status: Option<QuotaStatus>,
}

/// Request body for [`crate::admin::QuotasClient::set`].
#[derive(Debug, Clone, Serialize)]
pub(crate) struct SetQuotaRequest {
    pub custom_limit: Option<u32>,
    pub tier: Option<String>,
}
