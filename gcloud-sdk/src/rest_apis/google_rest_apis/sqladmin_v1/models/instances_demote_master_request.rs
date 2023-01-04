use serde::{Deserialize, Serialize}; /*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// InstancesDemoteMasterRequest : Database demote primary instance request.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstancesDemoteMasterRequest {
    #[serde(
        rename = "demoteMasterContext",
        skip_serializing_if = "Option::is_none"
    )]
    pub demote_master_context:
        Option<Box<crate::google_rest_apis::sqladmin_v1::models::DemoteMasterContext>>,
}

impl InstancesDemoteMasterRequest {
    /// Database demote primary instance request.
    pub fn new() -> InstancesDemoteMasterRequest {
        InstancesDemoteMasterRequest {
            demote_master_context: None,
        }
    }
}