use serde::{Deserialize, Serialize}; /*
                                      * Cloud Storage JSON API
                                      *
                                      * Stores and retrieves potentially large, immutable data objects.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// BucketAccessControlProjectTeam : The project team associated with the entity, if any.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BucketAccessControlProjectTeam {
    /// The project number.
    #[serde(rename = "projectNumber", skip_serializing_if = "Option::is_none")]
    pub project_number: Option<String>,
    /// The team.
    #[serde(rename = "team", skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
}

impl BucketAccessControlProjectTeam {
    /// The project team associated with the entity, if any.
    pub fn new() -> BucketAccessControlProjectTeam {
        BucketAccessControlProjectTeam {
            project_number: None,
            team: None,
        }
    }
}