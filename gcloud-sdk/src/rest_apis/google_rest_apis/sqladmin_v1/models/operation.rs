use serde::{Deserialize, Serialize}; /*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// Operation : An Operation resource. For successful operations that return an Operation resource, only the fields relevant to the operation are populated in the resource.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Operation {
    #[serde(rename = "backupContext", skip_serializing_if = "Option::is_none")]
    pub backup_context: Option<Box<crate::google_rest_apis::sqladmin_v1::models::BackupContext>>,
    /// The time this operation finished in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<crate::google_rest_apis::sqladmin_v1::models::OperationErrors>>,
    #[serde(rename = "exportContext", skip_serializing_if = "Option::is_none")]
    pub export_context: Option<Box<crate::google_rest_apis::sqladmin_v1::models::ExportContext>>,
    #[serde(rename = "importContext", skip_serializing_if = "Option::is_none")]
    pub import_context: Option<Box<crate::google_rest_apis::sqladmin_v1::models::ImportContext>>,
    /// The time this operation was enqueued in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`.
    #[serde(rename = "insertTime", skip_serializing_if = "Option::is_none")]
    pub insert_time: Option<String>,
    /// This is always `sql#operation`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// An identifier that uniquely identifies the operation. You can use this identifier to retrieve the Operations resource that has information about the operation.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of the operation. Valid values are: * `CREATE` * `DELETE` * `UPDATE` * `RESTART` * `IMPORT` * `EXPORT` * `BACKUP_VOLUME` * `RESTORE_VOLUME` * `CREATE_USER` * `DELETE_USER` * `CREATE_DATABASE` * `DELETE_DATABASE`
    #[serde(rename = "operationType", skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<OperationType>,
    /// The URI of this resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// The time this operation actually started in UTC timezone in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// The status of an operation.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Name of the database instance related to this operation.
    #[serde(rename = "targetId", skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "targetLink", skip_serializing_if = "Option::is_none")]
    pub target_link: Option<String>,
    /// The project ID of the target instance related to this operation.
    #[serde(rename = "targetProject", skip_serializing_if = "Option::is_none")]
    pub target_project: Option<String>,
    /// The email address of the user who initiated this operation.
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Operation {
    /// An Operation resource. For successful operations that return an Operation resource, only the fields relevant to the operation are populated in the resource.
    pub fn new() -> Operation {
        Operation {
            backup_context: None,
            end_time: None,
            error: None,
            export_context: None,
            import_context: None,
            insert_time: None,
            kind: None,
            name: None,
            operation_type: None,
            self_link: None,
            start_time: None,
            status: None,
            target_id: None,
            target_link: None,
            target_project: None,
            user: None,
        }
    }
}

/// The type of the operation. Valid values are: * `CREATE` * `DELETE` * `UPDATE` * `RESTART` * `IMPORT` * `EXPORT` * `BACKUP_VOLUME` * `RESTORE_VOLUME` * `CREATE_USER` * `DELETE_USER` * `CREATE_DATABASE` * `DELETE_DATABASE`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OperationType {
    #[serde(rename = "SQL_OPERATION_TYPE_UNSPECIFIED")]
    SqlOperationTypeUnspecified,
    #[serde(rename = "IMPORT")]
    Import,
    #[serde(rename = "EXPORT")]
    Export,
    #[serde(rename = "CREATE")]
    Create,
    #[serde(rename = "UPDATE")]
    Update,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "RESTART")]
    Restart,
    #[serde(rename = "BACKUP")]
    Backup,
    #[serde(rename = "SNAPSHOT")]
    Snapshot,
    #[serde(rename = "BACKUP_VOLUME")]
    BackupVolume,
    #[serde(rename = "DELETE_VOLUME")]
    DeleteVolume,
    #[serde(rename = "RESTORE_VOLUME")]
    RestoreVolume,
    #[serde(rename = "INJECT_USER")]
    InjectUser,
    #[serde(rename = "CLONE")]
    Clone,
    #[serde(rename = "STOP_REPLICA")]
    StopReplica,
    #[serde(rename = "START_REPLICA")]
    StartReplica,
    #[serde(rename = "PROMOTE_REPLICA")]
    PromoteReplica,
    #[serde(rename = "CREATE_REPLICA")]
    CreateReplica,
    #[serde(rename = "CREATE_USER")]
    CreateUser,
    #[serde(rename = "DELETE_USER")]
    DeleteUser,
    #[serde(rename = "UPDATE_USER")]
    UpdateUser,
    #[serde(rename = "CREATE_DATABASE")]
    CreateDatabase,
    #[serde(rename = "DELETE_DATABASE")]
    DeleteDatabase,
    #[serde(rename = "UPDATE_DATABASE")]
    UpdateDatabase,
    #[serde(rename = "FAILOVER")]
    Failover,
    #[serde(rename = "DELETE_BACKUP")]
    DeleteBackup,
    #[serde(rename = "RECREATE_REPLICA")]
    RecreateReplica,
    #[serde(rename = "TRUNCATE_LOG")]
    TruncateLog,
    #[serde(rename = "DEMOTE_MASTER")]
    DemoteMaster,
    #[serde(rename = "MAINTENANCE")]
    Maintenance,
    #[serde(rename = "ENABLE_PRIVATE_IP")]
    EnablePrivateIp,
    #[serde(rename = "DEFER_MAINTENANCE")]
    DeferMaintenance,
    #[serde(rename = "CREATE_CLONE")]
    CreateClone,
    #[serde(rename = "RESCHEDULE_MAINTENANCE")]
    RescheduleMaintenance,
    #[serde(rename = "START_EXTERNAL_SYNC")]
    StartExternalSync,
    #[serde(rename = "LOG_CLEANUP")]
    LogCleanup,
    #[serde(rename = "AUTO_RESTART")]
    AutoRestart,
}

impl Default for OperationType {
    fn default() -> OperationType {
        Self::SqlOperationTypeUnspecified
    }
}
/// The status of an operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "SQL_OPERATION_STATUS_UNSPECIFIED")]
    SqlOperationStatusUnspecified,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "DONE")]
    Done,
}

impl Default for Status {
    fn default() -> Status {
        Self::SqlOperationStatusUnspecified
    }
}