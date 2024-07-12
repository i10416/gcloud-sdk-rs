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
pub struct ProjectsSetCloudArmorTierRequest {
    /// Managed protection tier to be set.
    #[serde(rename = "cloudArmorTier", skip_serializing_if = "Option::is_none")]
    pub cloud_armor_tier: Option<CloudArmorTier>,
}

impl ProjectsSetCloudArmorTierRequest {
    pub fn new() -> ProjectsSetCloudArmorTierRequest {
        ProjectsSetCloudArmorTierRequest {
            cloud_armor_tier: None,
        }
    }
}

/// Managed protection tier to be set.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CloudArmorTier {
    #[serde(rename = "CA_ENTERPRISE_ANNUAL")]
    EnterpriseAnnual,
    #[serde(rename = "CA_ENTERPRISE_PAYGO")]
    EnterprisePaygo,
    #[serde(rename = "CA_STANDARD")]
    Standard,
}

impl Default for CloudArmorTier {
    fn default() -> CloudArmorTier {
        Self::EnterpriseAnnual
    }
}