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
pub struct AcceleratorTypesScopedList {
    /// [Output Only] A list of accelerator types contained in this scope.
    #[serde(rename = "acceleratorTypes", skip_serializing_if = "Option::is_none")]
    pub accelerator_types:
        Option<Vec<crate::google_rest_apis::compute_v1::models::AcceleratorType>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning:
        Option<Box<crate::google_rest_apis::compute_v1::models::AcceleratorTypesScopedListWarning>>,
}

impl AcceleratorTypesScopedList {
    pub fn new() -> AcceleratorTypesScopedList {
        AcceleratorTypesScopedList {
            accelerator_types: None,
            warning: None,
        }
    }
}