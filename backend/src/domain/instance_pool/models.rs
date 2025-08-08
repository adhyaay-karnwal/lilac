use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::identifier;

identifier!(InstancePoolId);

#[derive(Clone, Debug, Serialize, Deserialize, strum::EnumString, strum::Display)]
pub enum CloudProvider {
    #[serde(rename = "aws")]
    #[strum(serialize = "aws")]
    Aws,
    #[serde(rename = "gcp")]
    #[strum(serialize = "gcp")]
    Gcp,
    #[serde(rename = "azure")]
    #[strum(serialize = "azure")]
    Azure,
}

#[derive(Clone, Debug)]
pub struct InstancePool {
    pub id: InstancePoolId,
    pub name: String,
    pub description: Option<String>,
    pub provider: CloudProvider,
    pub region: String,
    pub instance_type: String,
    pub min_instances: i32,
    pub max_instances: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
