use async_trait::async_trait;

use super::{
    errors::InstancePoolRepositoryError,
    models::{CreateInstancePoolRequest, InstancePool, InstancePoolId},
};

#[async_trait]
pub trait InstancePoolRepository: Send + Sync {
    async fn create_instance_pool(
        &self,
        req: &CreateInstancePoolRequest,
    ) -> Result<InstancePool, InstancePoolRepositoryError>;

    async fn get_instance_pool_by_id(
        &self,
        id: &InstancePoolId,
    ) -> Result<InstancePool, InstancePoolRepositoryError>;
