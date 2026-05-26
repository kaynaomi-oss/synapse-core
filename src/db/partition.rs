use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use tracing::{error, info};

/// Partition manager that runs maintenance tasks periodically
pub struct PartitionManager {
    pool: PgPool,
    interval: Duration,
    limiter: Option<Arc<crate::services::ResourceLimiter>>,
}

impl PartitionManager {
    pub fn new(pool: PgPool, interval_hours: u64) -> Self {
        Self {
            pool,
            interval: Duration::from_secs(interval_hours * 3600),
            limiter: None,
        }
    }

    pub fn with_limiter(
        pool: PgPool,
        interval_hours: u64,
        limiter: Arc<crate::services::ResourceLimiter>,
    ) -> Self {
        Self {
            pool,
            interval: Duration::from_secs(interval_hours * 3600),
            limiter: Some(limiter),
        }
    }

    /// Start the partition maintenance background task
    pub fn start(self) {
        tokio::spawn(async move {
            let mut interval = time::interval(self.interval);
            interval.tick().await; // Skip first immediate tick

            loop {
                interval.tick().await;
                let result = if let Some(ref limiter) = self.limiter {
                    limiter
                        .run(async {
                            self.maintain_partitions().await
                        })
                        .await
                        .map_err(|e| sqlx::Error::Io(std::io::Error::new(
                            std::io::ErrorKind::TimedOut,
                            e.to_string(),
                        )))
                        .and_then(|r| r)
                } else {
                    self.maintain_partitions().await
                };

                if let Err(e) = result {
                    error!("Partition maintenance failed: {}", e);
                } else {
                    info!("Partition maintenance completed successfully");
                }
            }
        });
    }

    /// Run partition maintenance (create new partitions, detach old ones)
    async fn maintain_partitions(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT maintain_partitions()")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Manually trigger partition creation
    pub async fn create_partition(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT create_monthly_partition()")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Manually trigger old partition detachment
    pub async fn detach_old_partitions(&self, retention_months: i32) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT detach_old_partitions($1)")
            .bind(retention_months)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_partition_manager_creation() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://synapse:synapse@localhost:5432/synapse_test".to_string()
        });

        let pool = PgPool::connect(&database_url).await.unwrap();
        let manager = PartitionManager::new(pool, 24);

        assert_eq!(manager.interval, Duration::from_secs(24 * 3600));
    }
}
