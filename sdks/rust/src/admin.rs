use crate::admin_models::{SetQuotaRequest, TenantQuotaView};
use crate::client::SynapseClient;
use crate::error::SynapseError;

/// Entry point for admin-only Synapse API operations.
///
/// Obtain via [`SynapseClient::admin`]. Requires an API key with admin scope.
pub struct AdminClient<'a> {
    pub(crate) client: &'a SynapseClient,
}

impl<'a> AdminClient<'a> {
    /// Per-tenant quota management.
    pub fn quotas(&self) -> QuotasClient<'a> {
        QuotasClient {
            client: self.client,
        }
    }
}

/// Per-tenant quota management.
///
/// Obtain via [`AdminClient::quotas`].
pub struct QuotasClient<'a> {
    client: &'a SynapseClient,
}

impl<'a> QuotasClient<'a> {
    /// List quota usage for every active tenant.
    ///
    /// Calls `GET /admin/quotas`.
    pub async fn list(&self) -> Result<Vec<TenantQuotaView>, SynapseError> {
        self.client.get("/admin/quotas").await
    }

    /// Get quota usage for a single tenant.
    ///
    /// Calls `GET /admin/quotas/:tenant_id`.
    ///
    /// A 404 for an unknown `tenant_id` is reported as
    /// `SynapseError::Http { status: 404, .. }`, distinguishable from a
    /// `SynapseError::Network` transport failure.
    pub async fn get(&self, tenant_id: &str) -> Result<TenantQuotaView, SynapseError> {
        let path = format!("/admin/quotas/{}", tenant_id);
        self.client.get(&path).await
    }

    /// Override a tenant's custom rate limit.
    ///
    /// Calls `PUT /admin/quotas/:tenant_id`. Leaves the tenant's tier
    /// unchanged — only `custom_limit` is set by this call.
    pub async fn set(
        &self,
        tenant_id: &str,
        limit: u32,
    ) -> Result<serde_json::Value, SynapseError> {
        let path = format!("/admin/quotas/{}", tenant_id);
        let body = SetQuotaRequest {
            custom_limit: Some(limit),
            tier: None,
        };
        self.client.put(&path, &body).await
    }

    /// Reset a tenant's current usage counter back to zero.
    ///
    /// Calls `DELETE /admin/quotas/:tenant_id/reset`. Does not change the
    /// tenant's configured limit, only the counter.
    pub async fn reset(&self, tenant_id: &str) -> Result<serde_json::Value, SynapseError> {
        let path = format!("/admin/quotas/{}/reset", tenant_id);
        self.client.delete(&path).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::task::JoinHandle;

    /// Minimal one-shot mock HTTP server (no mocking crate available in this
    /// workspace). Accepts exactly one connection, replies with a canned
    /// `status`/`body`, and hands back the request line and `X-API-Key`
    /// header it received so tests can assert on both the call shape and
    /// the credential actually sent — no live server, no database.
    async fn mock_server(status: u16, body: &str) -> (String, JoinHandle<(String, Option<String>)>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let body = body.to_string();
        let handle = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buf = vec![0u8; 8192];
            let n = socket.read(&mut buf).await.unwrap();
            let request = String::from_utf8_lossy(&buf[..n]).to_string();
            let request_line = request.lines().next().unwrap_or_default().to_string();
            let api_key = request
                .lines()
                .find(|l| l.to_lowercase().starts_with("x-api-key:"))
                .and_then(|l| l.split_once(':'))
                .map(|(_, v)| v.trim().to_string());

            let reason = if status == 200 { "OK" } else { "Error" };
            let response = format!(
                "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            socket.write_all(response.as_bytes()).await.unwrap();
            socket.flush().await.unwrap();
            (request_line, api_key)
        });
        (format!("http://{addr}"), handle)
    }

    #[tokio::test]
    async fn list_returns_quota_views_on_200() {
        let body = serde_json::json!([{
            "tenant_id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "Acme Co",
            "rate_limit_per_minute": 100,
            "quota_status": {"limit": 100, "used": 42, "remaining": 58, "reset_in_seconds": 30}
        }])
        .to_string();
        let (base_url, handle) = mock_server(200, &body).await;
        let client = SynapseClient::builder(base_url, "admin-secret")
            .disable_retries()
            .build();

        let result = client.admin().quotas().list().await;
        let (request_line, api_key) = handle.await.unwrap();

        assert!(result.is_ok(), "expected Ok, got: {:?}", result);
        let views = result.unwrap();
        assert_eq!(views.len(), 1);
        assert_eq!(views[0].name, "Acme Co");
        assert_eq!(views[0].quota_status.as_ref().unwrap().remaining, 58);
        assert!(request_line.starts_with("GET /admin/quotas "));
        // The configured key must be sent on admin requests (this SDK has a
        // single configured key today — there is no separate admin-vs-public
        // key concept yet to assert "admin, not public" against).
        assert_eq!(api_key.as_deref(), Some("admin-secret"));
    }

    #[tokio::test]
    async fn get_returns_http_404_for_unknown_tenant() {
        let (base_url, handle) =
            mock_server(404, r#"{"error":"tenant not found"}"#).await;
        let client = SynapseClient::builder(base_url, "admin-secret")
            .disable_retries()
            .build();

        let result = client.admin().quotas().get("does-not-exist").await;
        let (request_line, _) = handle.await.unwrap();

        match result {
            Err(SynapseError::Http { status, .. }) => assert_eq!(status, 404),
            other => panic!("expected SynapseError::Http {{ status: 404 }}, got: {:?}", other),
        }
        assert!(request_line.starts_with("GET /admin/quotas/does-not-exist "));
    }

    #[tokio::test]
    async fn set_sends_put_with_custom_limit_and_admin_key() {
        let (base_url, handle) =
            mock_server(200, r#"{"message":"quota updated","tenant_id":"t1"}"#).await;
        let client = SynapseClient::builder(base_url, "admin-secret")
            .disable_retries()
            .build();

        let result = client.admin().quotas().set("t1", 250).await;
        let (request_line, api_key) = handle.await.unwrap();

        assert!(result.is_ok(), "expected Ok, got: {:?}", result);
        assert!(request_line.starts_with("PUT /admin/quotas/t1 "));
        assert_eq!(api_key.as_deref(), Some("admin-secret"));
    }

    #[tokio::test]
    async fn reset_sends_delete() {
        let (base_url, handle) =
            mock_server(200, r#"{"message":"quota reset","tenant_id":"t1"}"#).await;
        let client = SynapseClient::builder(base_url, "admin-secret")
            .disable_retries()
            .build();

        let result = client.admin().quotas().reset("t1").await;
        let (request_line, _) = handle.await.unwrap();

        assert!(result.is_ok(), "expected Ok, got: {:?}", result);
        assert!(request_line.starts_with("DELETE /admin/quotas/t1/reset "));
    }
}
