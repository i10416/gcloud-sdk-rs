use serde::{Deserialize, Serialize}; /*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// RestoreBackupContext : Database instance restore from backup context. Backup context contains source instance id and project id.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RestoreBackupContext {
    /// The ID of the backup run to restore from.
    #[serde(rename = "backupRunId", skip_serializing_if = "Option::is_none")]
    pub backup_run_id: Option<String>,
    /// The ID of the instance that the backup was taken from.
    #[serde(rename = "instanceId", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// This is always `sql#restoreBackupContext`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The full project ID of the source instance.
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl RestoreBackupContext {
    /// Database instance restore from backup context. Backup context contains source instance id and project id.
    pub fn new() -> RestoreBackupContext {
        RestoreBackupContext {
            backup_run_id: None,
            instance_id: None,
            kind: None,
            project: None,
        }
    }
}