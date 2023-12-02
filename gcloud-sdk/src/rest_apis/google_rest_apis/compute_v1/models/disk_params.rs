use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// DiskParams : Additional disk params.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DiskParams {
    /// Resource manager tags to be bound to the disk. Tag keys and values have the same definition as resource manager tags. Keys must be in the format `tagKeys/{tag_key_id}`, and values are in the format `tagValues/456`. The field is ignored (both PUT & PATCH) when empty.
    #[serde(
        rename = "resourceManagerTags",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_manager_tags: Option<::std::collections::HashMap<String, String>>,
}

impl DiskParams {
    /// Additional disk params.
    pub fn new() -> DiskParams {
        DiskParams {
            resource_manager_tags: None,
        }
    }
}