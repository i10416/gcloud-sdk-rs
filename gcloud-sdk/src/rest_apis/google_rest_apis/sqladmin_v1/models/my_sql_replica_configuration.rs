use serde::{Deserialize, Serialize}; /*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// MySqlReplicaConfiguration : Read-replica configuration specific to MySQL databases.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MySqlReplicaConfiguration {
    /// PEM representation of the trusted CA's x509 certificate.
    #[serde(rename = "caCertificate", skip_serializing_if = "Option::is_none")]
    pub ca_certificate: Option<String>,
    /// PEM representation of the replica's x509 certificate.
    #[serde(rename = "clientCertificate", skip_serializing_if = "Option::is_none")]
    pub client_certificate: Option<String>,
    /// PEM representation of the replica's private key. The corresponsing public key is encoded in the client's certificate.
    #[serde(rename = "clientKey", skip_serializing_if = "Option::is_none")]
    pub client_key: Option<String>,
    /// Seconds to wait between connect retries. MySQL's default is 60 seconds.
    #[serde(
        rename = "connectRetryInterval",
        skip_serializing_if = "Option::is_none"
    )]
    pub connect_retry_interval: Option<i32>,
    /// Path to a SQL dump file in Google Cloud Storage from which the replica instance is to be created. The URI is in the form gs://bucketName/fileName. Compressed gzip files (.gz) are also supported. Dumps have the binlog co-ordinates from which replication begins. This can be accomplished by setting --master-data to 1 when using mysqldump.
    #[serde(rename = "dumpFilePath", skip_serializing_if = "Option::is_none")]
    pub dump_file_path: Option<String>,
    /// This is always `sql#mysqlReplicaConfiguration`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Interval in milliseconds between replication heartbeats.
    #[serde(
        rename = "masterHeartbeatPeriod",
        skip_serializing_if = "Option::is_none"
    )]
    pub master_heartbeat_period: Option<String>,
    /// The password for the replication connection.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// A list of permissible ciphers to use for SSL encryption.
    #[serde(rename = "sslCipher", skip_serializing_if = "Option::is_none")]
    pub ssl_cipher: Option<String>,
    /// The username for the replication connection.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Whether or not to check the primary instance's Common Name value in the certificate that it sends during the SSL handshake.
    #[serde(
        rename = "verifyServerCertificate",
        skip_serializing_if = "Option::is_none"
    )]
    pub verify_server_certificate: Option<bool>,
}

impl MySqlReplicaConfiguration {
    /// Read-replica configuration specific to MySQL databases.
    pub fn new() -> MySqlReplicaConfiguration {
        MySqlReplicaConfiguration {
            ca_certificate: None,
            client_certificate: None,
            client_key: None,
            connect_retry_interval: None,
            dump_file_path: None,
            kind: None,
            master_heartbeat_period: None,
            password: None,
            ssl_cipher: None,
            username: None,
            verify_server_certificate: None,
        }
    }
}