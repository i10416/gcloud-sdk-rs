use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// SourceInstanceParams : A specification of the parameters to use when creating the instance template from a source instance.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SourceInstanceParams {
    /// Attached disks configuration. If not provided, defaults are applied: For boot disk and any other R/W disks, the source images for each disk will be used. For read-only disks, they will be attached in read-only mode. Local SSD disks will be created as blank volumes.
    #[serde(rename = "diskConfigs", skip_serializing_if = "Option::is_none")]
    pub disk_configs:
        Option<Vec<crate::google_rest_apis::compute_v1::models::DiskInstantiationConfig>>,
}

impl SourceInstanceParams {
    /// A specification of the parameters to use when creating the instance template from a source instance.
    pub fn new() -> SourceInstanceParams {
        SourceInstanceParams { disk_configs: None }
    }
}