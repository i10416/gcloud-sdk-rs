use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// CircuitBreakers : Settings controlling the volume of requests, connections and retries to this backend service.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CircuitBreakers {
    /// The maximum number of connections to the backend service. If not specified, there is no limit. Not supported when the backend service is referenced by a URL map that is bound to target gRPC proxy that has validateForProxyless field set to true.
    #[serde(rename = "maxConnections", skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i32>,
    /// The maximum number of pending requests allowed to the backend service. If not specified, there is no limit. Not supported when the backend service is referenced by a URL map that is bound to target gRPC proxy that has validateForProxyless field set to true.
    #[serde(rename = "maxPendingRequests", skip_serializing_if = "Option::is_none")]
    pub max_pending_requests: Option<i32>,
    /// The maximum number of parallel requests that allowed to the backend service. If not specified, there is no limit.
    #[serde(rename = "maxRequests", skip_serializing_if = "Option::is_none")]
    pub max_requests: Option<i32>,
    /// Maximum requests for a single connection to the backend service. This parameter is respected by both the HTTP/1.1 and HTTP/2 implementations. If not specified, there is no limit. Setting this parameter to 1 will effectively disable keep alive. Not supported when the backend service is referenced by a URL map that is bound to target gRPC proxy that has validateForProxyless field set to true.
    #[serde(
        rename = "maxRequestsPerConnection",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_requests_per_connection: Option<i32>,
    /// The maximum number of parallel retries allowed to the backend cluster. If not specified, the default is 1. Not supported when the backend service is referenced by a URL map that is bound to target gRPC proxy that has validateForProxyless field set to true.
    #[serde(rename = "maxRetries", skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
}

impl CircuitBreakers {
    /// Settings controlling the volume of requests, connections and retries to this backend service.
    pub fn new() -> CircuitBreakers {
        CircuitBreakers {
            max_connections: None,
            max_pending_requests: None,
            max_requests: None,
            max_requests_per_connection: None,
            max_retries: None,
        }
    }
}