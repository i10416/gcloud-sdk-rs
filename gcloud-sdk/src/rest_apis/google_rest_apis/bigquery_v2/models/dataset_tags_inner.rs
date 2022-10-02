use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DatasetTagsInner {
    /// [Required] The namespaced friendly name of the tag key, e.g. \"12345/environment\" where 12345 is org id.
    #[serde(rename = "tagKey", skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// [Required] Friendly short name of the tag value, e.g. \"production\".
    #[serde(rename = "tagValue", skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl DatasetTagsInner {
    pub fn new() -> DatasetTagsInner {
        DatasetTagsInner {
            tag_key: None,
            tag_value: None,
        }
    }
}