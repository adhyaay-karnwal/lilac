#[derive(Debug, thiserror::Error)]
pub enum InstancePoolRepositoryError {
    #[error("instance pool with {field} {value} already exists")]
    Duplicate { field: String, value: String },
    #[error("instance pool {0} not found")]
    NotFound(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

