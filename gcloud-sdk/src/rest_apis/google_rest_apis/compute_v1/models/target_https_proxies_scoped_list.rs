use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TargetHttpsProxiesScopedList {
    /// A list of TargetHttpsProxies contained in this scope.
    #[serde(rename = "targetHttpsProxies", skip_serializing_if = "Option::is_none")]
    pub target_https_proxies:
        Option<Vec<crate::google_rest_apis::compute_v1::models::TargetHttpsProxy>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning:
        Option<Box<crate::google_rest_apis::compute_v1::models::BackendServicesScopedListWarning>>,
}

impl TargetHttpsProxiesScopedList {
    pub fn new() -> TargetHttpsProxiesScopedList {
        TargetHttpsProxiesScopedList {
            target_https_proxies: None,
            warning: None,
        }
    }
}