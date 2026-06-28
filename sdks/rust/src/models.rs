use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ── Transaction models ────────────────────────────────────────────────────────

/// A single transaction returned by the API.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    pub id: String,
    pub stellar_account: String,
    pub amount: String,
    pub asset_code: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub anchor_transaction_id: Option<String>,
    pub callback_type: Option<String>,
    pub callback_status: Option<String>,
    pub settlement_id: Option<String>,
    pub memo: Option<String>,
    pub memo_type: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Pagination metadata included in list responses.
#[derive(Debug, Clone, Deserialize)]
pub struct ListMeta {
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

/// Paginated list of transactions.
#[derive(Debug, Clone, Deserialize)]
pub struct TransactionList {
    pub data: Vec<Transaction>,
    pub meta: ListMeta,
}

/// Filters for [`Transactions::search`].
///
/// All fields are optional; omit a field to leave that dimension unfiltered.
/// A search with no matches returns an empty [`TransactionSearch`] (a page with
/// `total == 0` and no `results`), never an error.
#[derive(Debug, Default)]
pub struct SearchParams {
    /// Exact transaction status (e.g. `"pending"`, `"completed"`).
    pub status: Option<String>,
    /// Exact asset code (e.g. `"USD"`).
    pub asset_code: Option<String>,
    /// Inclusive minimum amount, as a decimal string (e.g. `"10.00"`).
    pub min_amount: Option<String>,
    /// Inclusive maximum amount, as a decimal string (e.g. `"500.00"`).
    pub max_amount: Option<String>,
    /// Inclusive RFC 3339 range start (e.g. `"2024-01-01T00:00:00Z"`).
    pub from: Option<String>,
    /// Exclusive RFC 3339 range end (e.g. `"2024-02-01T00:00:00Z"`).
    pub to: Option<String>,
    /// Exact Stellar account to filter by.
    pub stellar_account: Option<String>,
    /// Opaque pagination cursor from a previous response's `next_cursor`.
    pub cursor: Option<String>,
    /// Maximum records per page (server default: 25, max: 100).
    pub limit: Option<i64>,
}

/// A single page of transactions returned by [`Transactions::search`].
#[derive(Debug, Clone, Deserialize)]
pub struct TransactionSearch {
    /// Total number of records matching the filters across all pages.
    pub total: i64,
    /// Matching transactions for this page (empty when nothing matched).
    #[serde(default)]
    pub results: Vec<Transaction>,
    /// Opaque cursor for the next page, or `None` when this is the last page.
    #[serde(default)]
    pub next_cursor: Option<String>,
}

/// Query parameters for [`Transactions::list`].
///
/// All fields are optional; omit a field to accept the server's default.
/// Never construct a `cursor` manually — always use one from a previous
/// response's `meta.next_cursor`.
#[derive(Debug, Default)]
pub struct ListParams {
    /// Opaque pagination cursor from `meta.next_cursor`.
    pub cursor: Option<String>,
    /// Maximum records per page (server default: 25, max: 100).
    pub limit: Option<i64>,
    /// Inclusive ISO 8601 range start (e.g. `"2024-01-01T00:00:00Z"`).
    pub from_date: Option<String>,
    /// Exclusive ISO 8601 range end (e.g. `"2024-02-01T00:00:00Z"`).
    pub to_date: Option<String>,
}

// ── GraphQL models (issue #634) ───────────────────────────────────────────────

/// Request body for `POST /graphql`.
#[derive(Debug, Clone, Serialize)]
pub struct GraphQLRequest {
    pub query: String,
    pub variables: Option<serde_json::Value>,
}

/// A GraphQL application-level error returned inside an HTTP 200 response.
#[derive(Debug, Clone, Deserialize)]
pub struct GraphQLError {
    pub message: String,
}

/// Response from `POST /graphql`.
#[derive(Debug, Clone, Deserialize)]
pub struct GraphQLResponse {
    pub data: Option<serde_json::Value>,
    #[serde(default)]
    pub errors: Vec<GraphQLError>,
}

// ── Stats models (issue #633) ─────────────────────────────────────────────────

/// Per-status transaction count returned by `GET /stats/status`.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct StatusCount {
    pub status: String,
    pub count: i64,
}

/// Per-day transaction volume returned by `GET /stats/daily`.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct DailyTotal {
    pub date: String,
    pub count: i64,
    pub total_amount: String,
}

/// Per-asset statistics returned by `GET /stats/assets`.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AssetStats {
    pub asset_code: String,
    pub count: i64,
    pub total_amount: String,
}

/// Cache metrics returned by `GET /stats/cache`.
///
/// Empty datasets return a zeroed structure, never `null`/`None`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CacheMetrics {
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
    pub evictions: u64,
    pub size: u64,
    pub capacity: u64,
}

impl Default for CacheMetrics {
    fn default() -> Self {
        Self { hits: 0, misses: 0, hit_rate: 0.0, evictions: 0, size: 0, capacity: 0 }
    }
}

/// Query parameters for `stats.daily()`.
#[derive(Debug, Default)]
pub struct DailyParams {
    /// Number of days to include (1–365; server default: 7).
    pub days: Option<i32>,
}

// ── Events / reconnect models (issue #642) ────────────────────────────────────

/// Response from `POST /reconnect` or `GET /reconnect/status`.
#[derive(Debug, Clone, Deserialize)]
pub struct ReconnectResponse {
    #[serde(rename = "type")]
    pub kind: String,
    pub status: Option<ReconnectStatus>,
    pub backoff_seconds: Option<u64>,
    pub requires_resync: Option<bool>,
    pub message: Option<String>,
}

/// Reconnect status variant embedded in [`ReconnectResponse`].
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ReconnectStatus {
    Ready { session_id: String },
    RetryAfter { wait_seconds: u64 },
    SessionExpired,
    InvalidToken,
}

// ── Admin / bulk-status models (issue #644) ───────────────────────────────────

/// Per-item failure reported inside [`BulkStatusResponse`].
#[derive(Debug, Clone, Deserialize)]
pub struct BulkUpdateError {
    pub id: String,
    pub error: String,
}

/// Response from `POST /admin/transactions/bulk-status`.
#[derive(Debug, Clone, Deserialize)]
pub struct BulkStatusResponse {
    pub updated: usize,
    pub failed: usize,
    #[serde(default)]
    pub errors: Vec<BulkUpdateError>,
}
