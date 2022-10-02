use serde::{Deserialize, Serialize}; /*
                                      * Cloud Storage JSON API
                                      *
                                      * Stores and retrieves potentially large, immutable data objects.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// BucketIamConfigurationUniformBucketLevelAccess : The bucket's uniform bucket-level access configuration.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BucketIamConfigurationUniformBucketLevelAccess {
    /// If set, access is controlled only by bucket-level or above IAM policies.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The deadline for changing iamConfiguration.uniformBucketLevelAccess.enabled from true to false in RFC 3339  format. iamConfiguration.uniformBucketLevelAccess.enabled may be changed from true to false until the locked time, after which the field is immutable.
    #[serde(rename = "lockedTime", skip_serializing_if = "Option::is_none")]
    pub locked_time: Option<String>,
}

impl BucketIamConfigurationUniformBucketLevelAccess {
    /// The bucket's uniform bucket-level access configuration.
    pub fn new() -> BucketIamConfigurationUniformBucketLevelAccess {
        BucketIamConfigurationUniformBucketLevelAccess {
            enabled: None,
            locked_time: None,
        }
    }
}