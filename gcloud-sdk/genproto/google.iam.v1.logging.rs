// This file is @generated by prost-build.
/// Audit log information specific to Cloud IAM. This message is serialized
/// as an `Any` type in the `ServiceData` message of an
/// `AuditLog` message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditData {
    /// Policy delta between the original policy and the newly set policy.
    #[prost(message, optional, tag = "2")]
    pub policy_delta: ::core::option::Option<super::PolicyDelta>,
}
