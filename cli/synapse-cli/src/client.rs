use anyhow::{bail, Result};
use serde::de::DeserializeOwned;

/// Minimal HTTP client for the Synapse API.
pub struct ApiClient {
    http: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl ApiClient {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
        }
    }

    /// GET `path` and deserialise the JSON body into `T`.
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("HTTP {}: {}", status, body);
        }
        Ok(resp.json::<T>().await?)
    }

    /// GET `path` with an optional query string and deserialise the JSON body into `T`.
    pub async fn get_with_query<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .query(query)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("HTTP {}: {}", status, body);
        }
        Ok(resp.json::<T>().await?)
    }
}
