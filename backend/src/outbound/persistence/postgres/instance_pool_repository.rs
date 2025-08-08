use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    domain::instance_pool::{
        errors::InstancePoolRepositoryError,
        models::{CreateInstancePoolRequest, InstancePool, InstancePoolId},
        ports::InstancePoolRepository,
    },
    outbound::persistence::postgres::records::InstancePoolRecord,
};

#[derive(Clone)]
pub struct PostgresInstancePoolRepository {
    pool: PgPool,
}

impl PostgresInstancePoolRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InstancePoolRepository for PostgresInstancePoolRepository {
    async fn create_instance_pool(
        &self,
        req: &CreateInstancePoolRequest,
    ) -> Result<InstancePool, InstancePoolRepositoryError> {
        let record = sqlx::query_as!(
            InstancePoolRecord,
            r#"INSERT INTO instance_pools (name, description, provider, region, instance_type, min_instances, max_instances)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, name, description, provider AS "provider: _", region, instance_type, min_instances, max_instances, created_at, updated_at"#,
            req.name,
            req.description,
            req.provider.to_string(),
            req.region,
            req.instance_type,
            req.min_instances,
            req.max_instances,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| InstancePoolRepositoryError::Unknown(e.into()))?;

        Ok(record.into())
    }

    async fn get_instance_pool_by_id(
        &self,
        id: &InstancePoolId,
    ) -> Result<InstancePool, InstancePoolRepositoryError> {
        let record = sqlx::query_as!(
            InstancePoolRecord,
            r#"
            SELECT id, name, description, provider AS "provider: _", region, instance_type, min_instances, max_instances, created_at, updated_at
            FROM instance_pools
            WHERE id = $1
            "#,
            id.inner(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => InstancePoolRepositoryError::NotFound(id.inner().to_string()),
            _ => InstancePoolRepositoryError::Unknown(e.into()),
        })?;

        Ok(record.into())
    }

    async fn list_instance_pools(&self) -> Result<Vec<InstancePool>, InstancePoolRepositoryError> {
        let records = sqlx::query_as!(
            InstancePoolRecord,
            r#"
            SELECT id, name, description, provider AS "provider: _", region, instance_type, min_instances, max_instances, created_at, updated_at
            FROM instance_pools
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| InstancePoolRepositoryError::Unknown(e.into()))?;

        Ok(records.into_iter().map(|r| r.into()).collect())
    }
}

