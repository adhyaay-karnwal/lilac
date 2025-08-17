use std::sync::Arc;

use async_trait::async_trait;

use super::{
    errors::InstancePoolRepositoryError,
    models::{CreateInstancePoolRequest, InstancePool, InstancePoolId},
    ports::InstancePoolRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum InstancePoolServiceError {
    #[error("instance pool with {field} {value} already exists")]
    InstancePoolExists { field: String, value: String },
    #[error("instance pool {0} not found")]
    InstancePoolNotFound(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<InstancePoolRepositoryError> for InstancePoolServiceError {
    fn from(error: InstancePoolRepositoryError) -> Self {
        match error {
            InstancePoolRepositoryError::Duplicate { field, value } => {
                Self::InstancePoolExists { field, value }
            }
            InstancePoolRepositoryError::NotFound(id) => Self::InstancePoolNotFound(id),
            InstancePoolRepositoryError::Unknown(error) => Self::Unknown(error),
        }
    }
}

#[async_trait]
pub trait InstancePoolService: Send + Sync {
    async fn create_instance_pool(
        &self,
        req: &CreateInstancePoolRequest,
    ) -> Result<InstancePool, InstancePoolServiceError>;

    async fn get_instance_pool_by_id(
        &self,
        id: &InstancePoolId,
    ) -> Result<InstancePool, InstancePoolServiceError>;

    async fn list_instance_pools(&self) -> Result<Vec<InstancePool>, InstancePoolServiceError>;
}

#[derive(Clone)]
pub struct InstancePoolServiceImpl {
    repository: Arc<dyn InstancePoolRepository>,
}

impl InstancePoolServiceImpl {
    pub fn new(repository: Arc<dyn InstancePoolRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl InstancePoolService for InstancePoolServiceImpl {
    async fn create_instance_pool(
        &self,
        req: &CreateInstancePoolRequest,
    ) -> Result<InstancePool, InstancePoolServiceError> {
        Ok(self.repository.create_instance_pool(req).await?)
    }

    async fn get_instance_pool_by_id(
        &self,
        id: &InstancePoolId,
    ) -> Result<InstancePool, InstancePoolServiceError> {
        Ok(self.repository.get_instance_pool_by_id(id).await?)
    }

