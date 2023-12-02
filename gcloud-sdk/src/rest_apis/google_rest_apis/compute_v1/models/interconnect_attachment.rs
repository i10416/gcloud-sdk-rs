use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// InterconnectAttachment : Represents an Interconnect Attachment (VLAN) resource. You can use Interconnect attachments (VLANS) to connect your Virtual Private Cloud networks to your on-premises networks through an Interconnect. For more information, read Creating VLAN Attachments.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InterconnectAttachment {
    /// Determines whether this Attachment will carry packets. Not present for PARTNER_PROVIDER.
    #[serde(rename = "adminEnabled", skip_serializing_if = "Option::is_none")]
    pub admin_enabled: Option<bool>,
    /// Provisioned bandwidth capacity for the interconnect attachment. For attachments of type DEDICATED, the user can set the bandwidth. For attachments of type PARTNER, the Google Partner that is operating the interconnect must set the bandwidth. Output only for PARTNER type, mutable for PARTNER_PROVIDER and DEDICATED, and can take one of the following values: - BPS_50M: 50 Mbit/s - BPS_100M: 100 Mbit/s - BPS_200M: 200 Mbit/s - BPS_300M: 300 Mbit/s - BPS_400M: 400 Mbit/s - BPS_500M: 500 Mbit/s - BPS_1G: 1 Gbit/s - BPS_2G: 2 Gbit/s - BPS_5G: 5 Gbit/s - BPS_10G: 10 Gbit/s - BPS_20G: 20 Gbit/s - BPS_50G: 50 Gbit/s 
    #[serde(rename = "bandwidth", skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<Bandwidth>,
    /// This field is not available.
    #[serde(rename = "candidateIpv6Subnets", skip_serializing_if = "Option::is_none")]
    pub candidate_ipv6_subnets: Option<Vec<String>>,
    /// Up to 16 candidate prefixes that can be used to restrict the allocation of cloudRouterIpAddress and customerRouterIpAddress for this attachment. All prefixes must be within link-local address space (169.254.0.0/16) and must be /29 or shorter (/28, /27, etc). Google will attempt to select an unused /29 from the supplied candidate prefix(es). The request will fail if all possible /29s are in use on Google's edge. If not supplied, Google will randomly select an unused /29 from all of link-local space.
    #[serde(rename = "candidateSubnets", skip_serializing_if = "Option::is_none")]
    pub candidate_subnets: Option<Vec<String>>,
    /// [Output Only] IPv4 address + prefix length to be configured on Cloud Router Interface for this interconnect attachment.
    #[serde(rename = "cloudRouterIpAddress", skip_serializing_if = "Option::is_none")]
    pub cloud_router_ip_address: Option<String>,
    /// [Output Only] IPv6 address + prefix length to be configured on Cloud Router Interface for this interconnect attachment.
    #[serde(rename = "cloudRouterIpv6Address", skip_serializing_if = "Option::is_none")]
    pub cloud_router_ipv6_address: Option<String>,
    /// This field is not available.
    #[serde(rename = "cloudRouterIpv6InterfaceId", skip_serializing_if = "Option::is_none")]
    pub cloud_router_ipv6_interface_id: Option<String>,
    #[serde(rename = "configurationConstraints", skip_serializing_if = "Option::is_none")]
    pub configuration_constraints: Option<Box<crate::google_rest_apis::compute_v1::models::InterconnectAttachmentConfigurationConstraints>>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// [Output Only] IPv4 address + prefix length to be configured on the customer router subinterface for this interconnect attachment.
    #[serde(rename = "customerRouterIpAddress", skip_serializing_if = "Option::is_none")]
    pub customer_router_ip_address: Option<String>,
    /// [Output Only] IPv6 address + prefix length to be configured on the customer router subinterface for this interconnect attachment.
    #[serde(rename = "customerRouterIpv6Address", skip_serializing_if = "Option::is_none")]
    pub customer_router_ipv6_address: Option<String>,
    /// This field is not available.
    #[serde(rename = "customerRouterIpv6InterfaceId", skip_serializing_if = "Option::is_none")]
    pub customer_router_ipv6_interface_id: Option<String>,
    /// [Output Only] Dataplane version for this InterconnectAttachment. This field is only present for Dataplane version 2 and higher. Absence of this field in the API output indicates that the Dataplane is version 1.
    #[serde(rename = "dataplaneVersion", skip_serializing_if = "Option::is_none")]
    pub dataplane_version: Option<i32>,
    /// An optional description of this resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Desired availability domain for the attachment. Only available for type PARTNER, at creation time, and can take one of the following values: - AVAILABILITY_DOMAIN_ANY - AVAILABILITY_DOMAIN_1 - AVAILABILITY_DOMAIN_2 For improved reliability, customers should configure a pair of attachments, one per availability domain. The selected availability domain will be provided to the Partner via the pairing key, so that the provisioned circuit will lie in the specified domain. If not specified, the value will default to AVAILABILITY_DOMAIN_ANY.
    #[serde(rename = "edgeAvailabilityDomain", skip_serializing_if = "Option::is_none")]
    pub edge_availability_domain: Option<EdgeAvailabilityDomain>,
    /// Indicates the user-supplied encryption option of this VLAN attachment (interconnectAttachment). Can only be specified at attachment creation for PARTNER or DEDICATED attachments. Possible values are: - NONE - This is the default value, which means that the VLAN attachment carries unencrypted traffic. VMs are able to send traffic to, or receive traffic from, such a VLAN attachment. - IPSEC - The VLAN attachment carries only encrypted traffic that is encrypted by an IPsec device, such as an HA VPN gateway or third-party IPsec VPN. VMs cannot directly send traffic to, or receive traffic from, such a VLAN attachment. To use *HA VPN over Cloud Interconnect*, the VLAN attachment must be created with this option. 
    #[serde(rename = "encryption", skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    /// [Output Only] Google reference ID, to be used when raising support tickets with Google or otherwise to debug backend connectivity issues. [Deprecated] This field is not used.
    #[serde(rename = "googleReferenceId", skip_serializing_if = "Option::is_none")]
    pub google_reference_id: Option<String>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// URL of the underlying Interconnect object that this attachment's traffic will traverse through.
    #[serde(rename = "interconnect", skip_serializing_if = "Option::is_none")]
    pub interconnect: Option<String>,
    /// A list of URLs of addresses that have been reserved for the VLAN attachment. Used only for the VLAN attachment that has the encryption option as IPSEC. The addresses must be regional internal IP address ranges. When creating an HA VPN gateway over the VLAN attachment, if the attachment is configured to use a regional internal IP address, then the VPN gateway's IP address is allocated from the IP address range specified here. For example, if the HA VPN gateway's interface 0 is paired to this VLAN attachment, then a regional internal IP address for the VPN gateway interface 0 will be allocated from the IP address specified for this VLAN attachment. If this field is not specified when creating the VLAN attachment, then later on when creating an HA VPN gateway on this VLAN attachment, the HA VPN gateway's IP address is allocated from the regional external IP address pool.
    #[serde(rename = "ipsecInternalAddresses", skip_serializing_if = "Option::is_none")]
    pub ipsec_internal_addresses: Option<Vec<String>>,
    /// [Output Only] Type of the resource. Always compute#interconnectAttachment for interconnect attachments.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// A fingerprint for the labels being applied to this InterconnectAttachment, which is essentially a hash of the labels set used for optimistic locking. The fingerprint is initially generated by Compute Engine and changes after every request to modify or update labels. You must always provide an up-to-date fingerprint hash in order to update or change labels, otherwise the request will fail with error 412 conditionNotMet. To see the latest fingerprint, make a get() request to retrieve an InterconnectAttachment.
    #[serde(rename = "labelFingerprint", skip_serializing_if = "Option::is_none")]
    pub label_fingerprint: Option<String>,
    /// Labels for this resource. These can only be added or modified by the setLabels method. Each label key/value pair must comply with RFC1035. Label values may be empty.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// Maximum Transmission Unit (MTU), in bytes, of packets passing through this interconnect attachment. Only 1440 and 1500 are allowed. If not specified, the value will default to 1440.
    #[serde(rename = "mtu", skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    /// Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [Output Only] The current status of whether or not this interconnect attachment is functional, which can take one of the following values: - OS_ACTIVE: The attachment has been turned up and is ready to use. - OS_UNPROVISIONED: The attachment is not ready to use yet, because turnup is not complete. 
    #[serde(rename = "operationalStatus", skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<OperationalStatus>,
    /// [Output only for type PARTNER. Input only for PARTNER_PROVIDER. Not present for DEDICATED]. The opaque identifier of a PARTNER attachment used to initiate provisioning with a selected partner. Of the form \"XXXXX/region/domain\"
    #[serde(rename = "pairingKey", skip_serializing_if = "Option::is_none")]
    pub pairing_key: Option<String>,
    /// Optional BGP ASN for the router supplied by a Layer 3 Partner if they configured BGP on behalf of the customer. Output only for PARTNER type, input only for PARTNER_PROVIDER, not available for DEDICATED.
    #[serde(rename = "partnerAsn", skip_serializing_if = "Option::is_none")]
    pub partner_asn: Option<String>,
    #[serde(rename = "partnerMetadata", skip_serializing_if = "Option::is_none")]
    pub partner_metadata: Option<Box<crate::google_rest_apis::compute_v1::models::InterconnectAttachmentPartnerMetadata>>,
    #[serde(rename = "privateInterconnectInfo", skip_serializing_if = "Option::is_none")]
    pub private_interconnect_info: Option<Box<crate::google_rest_apis::compute_v1::models::InterconnectAttachmentPrivateInfo>>,
    /// [Output Only] URL of the region where the regional interconnect attachment resides. You must specify this field as part of the HTTP request URL. It is not settable as a field in the request body.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// [Output Only] If the attachment is on a Cross-Cloud Interconnect connection, this field contains the interconnect's remote location service provider. Example values: \"Amazon Web Services\" \"Microsoft Azure\". The field is set only for attachments on Cross-Cloud Interconnect connections. Its value is copied from the InterconnectRemoteLocation remoteService field.
    #[serde(rename = "remoteService", skip_serializing_if = "Option::is_none")]
    pub remote_service: Option<String>,
    /// URL of the Cloud Router to be used for dynamic routing. This router must be in the same region as this InterconnectAttachment. The InterconnectAttachment will automatically connect the Interconnect to the network & region within which the Cloud Router is configured.
    #[serde(rename = "router", skip_serializing_if = "Option::is_none")]
    pub router: Option<String>,
    /// [Output Only] Reserved for future use.
    #[serde(rename = "satisfiesPzs", skip_serializing_if = "Option::is_none")]
    pub satisfies_pzs: Option<bool>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// The stack type for this interconnect attachment to identify whether the IPv6 feature is enabled or not. If not specified, IPV4_ONLY will be used. This field can be both set at interconnect attachments creation and update interconnect attachment operations.
    #[serde(rename = "stackType", skip_serializing_if = "Option::is_none")]
    pub stack_type: Option<StackType>,
    /// [Output Only] The current state of this attachment's functionality. Enum values ACTIVE and UNPROVISIONED are shared by DEDICATED/PRIVATE, PARTNER, and PARTNER_PROVIDER interconnect attachments, while enum values PENDING_PARTNER, PARTNER_REQUEST_RECEIVED, and PENDING_CUSTOMER are used for only PARTNER and PARTNER_PROVIDER interconnect attachments. This state can take one of the following values: - ACTIVE: The attachment has been turned up and is ready to use. - UNPROVISIONED: The attachment is not ready to use yet, because turnup is not complete. - PENDING_PARTNER: A newly-created PARTNER attachment that has not yet been configured on the Partner side. - PARTNER_REQUEST_RECEIVED: A PARTNER attachment is in the process of provisioning after a PARTNER_PROVIDER attachment was created that references it. - PENDING_CUSTOMER: A PARTNER or PARTNER_PROVIDER attachment that is waiting for a customer to activate it. - DEFUNCT: The attachment was deleted externally and is no longer functional. This could be because the associated Interconnect was removed, or because the other side of a Partner attachment was deleted. 
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// Length of the IPv4 subnet mask. Allowed values: - 29 (default) - 30 The default value is 29, except for Cross-Cloud Interconnect connections that use an InterconnectRemoteLocation with a constraints.subnetLengthRange.min equal to 30. For example, connections that use an Azure remote location fall into this category. In these cases, the default value is 30, and requesting 29 returns an error. Where both 29 and 30 are allowed, 29 is preferred, because it gives Google Cloud Support more debugging visibility. 
    #[serde(rename = "subnetLength", skip_serializing_if = "Option::is_none")]
    pub subnet_length: Option<i32>,
    /// The type of interconnect attachment this is, which can take one of the following values: - DEDICATED: an attachment to a Dedicated Interconnect. - PARTNER: an attachment to a Partner Interconnect, created by the customer. - PARTNER_PROVIDER: an attachment to a Partner Interconnect, created by the partner. 
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// The IEEE 802.1Q VLAN tag for this attachment, in the range 2-4093. Only specified at creation time.
    #[serde(rename = "vlanTag8021q", skip_serializing_if = "Option::is_none")]
    pub vlan_tag8021q: Option<i32>,
}

impl InterconnectAttachment {
    /// Represents an Interconnect Attachment (VLAN) resource. You can use Interconnect attachments (VLANS) to connect your Virtual Private Cloud networks to your on-premises networks through an Interconnect. For more information, read Creating VLAN Attachments.
    pub fn new() -> InterconnectAttachment {
        InterconnectAttachment {
            admin_enabled: None,
            bandwidth: None,
            candidate_ipv6_subnets: None,
            candidate_subnets: None,
            cloud_router_ip_address: None,
            cloud_router_ipv6_address: None,
            cloud_router_ipv6_interface_id: None,
            configuration_constraints: None,
            creation_timestamp: None,
            customer_router_ip_address: None,
            customer_router_ipv6_address: None,
            customer_router_ipv6_interface_id: None,
            dataplane_version: None,
            description: None,
            edge_availability_domain: None,
            encryption: None,
            google_reference_id: None,
            id: None,
            interconnect: None,
            ipsec_internal_addresses: None,
            kind: None,
            label_fingerprint: None,
            labels: None,
            mtu: None,
            name: None,
            operational_status: None,
            pairing_key: None,
            partner_asn: None,
            partner_metadata: None,
            private_interconnect_info: None,
            region: None,
            remote_service: None,
            router: None,
            satisfies_pzs: None,
            self_link: None,
            stack_type: None,
            state: None,
            subnet_length: None,
            r#type: None,
            vlan_tag8021q: None,
        }
    }
}

/// Provisioned bandwidth capacity for the interconnect attachment. For attachments of type DEDICATED, the user can set the bandwidth. For attachments of type PARTNER, the Google Partner that is operating the interconnect must set the bandwidth. Output only for PARTNER type, mutable for PARTNER_PROVIDER and DEDICATED, and can take one of the following values: - BPS_50M: 50 Mbit/s - BPS_100M: 100 Mbit/s - BPS_200M: 200 Mbit/s - BPS_300M: 300 Mbit/s - BPS_400M: 400 Mbit/s - BPS_500M: 500 Mbit/s - BPS_1G: 1 Gbit/s - BPS_2G: 2 Gbit/s - BPS_5G: 5 Gbit/s - BPS_10G: 10 Gbit/s - BPS_20G: 20 Gbit/s - BPS_50G: 50 Gbit/s
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Bandwidth {
    #[serde(rename = "BPS_100M")]
    Variant100M,
    #[serde(rename = "BPS_10G")]
    Variant10G,
    #[serde(rename = "BPS_1G")]
    Variant1G,
    #[serde(rename = "BPS_200M")]
    Variant200M,
    #[serde(rename = "BPS_20G")]
    Variant20G,
    #[serde(rename = "BPS_2G")]
    Variant2G,
    #[serde(rename = "BPS_300M")]
    Variant300M,
    #[serde(rename = "BPS_400M")]
    Variant400M,
    #[serde(rename = "BPS_500M")]
    Variant500M,
    #[serde(rename = "BPS_50G")]
    Variant50G,
    #[serde(rename = "BPS_50M")]
    Variant50M,
    #[serde(rename = "BPS_5G")]
    Variant5G,
}

impl Default for Bandwidth {
    fn default() -> Bandwidth {
        Self::Variant100M
    }
}
/// Desired availability domain for the attachment. Only available for type PARTNER, at creation time, and can take one of the following values: - AVAILABILITY_DOMAIN_ANY - AVAILABILITY_DOMAIN_1 - AVAILABILITY_DOMAIN_2 For improved reliability, customers should configure a pair of attachments, one per availability domain. The selected availability domain will be provided to the Partner via the pairing key, so that the provisioned circuit will lie in the specified domain. If not specified, the value will default to AVAILABILITY_DOMAIN_ANY.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EdgeAvailabilityDomain {
    #[serde(rename = "AVAILABILITY_DOMAIN_1")]
    Variant1,
    #[serde(rename = "AVAILABILITY_DOMAIN_2")]
    Variant2,
    #[serde(rename = "AVAILABILITY_DOMAIN_ANY")]
    Any,
}

impl Default for EdgeAvailabilityDomain {
    fn default() -> EdgeAvailabilityDomain {
        Self::Variant1
    }
}
/// Indicates the user-supplied encryption option of this VLAN attachment (interconnectAttachment). Can only be specified at attachment creation for PARTNER or DEDICATED attachments. Possible values are: - NONE - This is the default value, which means that the VLAN attachment carries unencrypted traffic. VMs are able to send traffic to, or receive traffic from, such a VLAN attachment. - IPSEC - The VLAN attachment carries only encrypted traffic that is encrypted by an IPsec device, such as an HA VPN gateway or third-party IPsec VPN. VMs cannot directly send traffic to, or receive traffic from, such a VLAN attachment. To use *HA VPN over Cloud Interconnect*, the VLAN attachment must be created with this option.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Encryption {
    #[serde(rename = "IPSEC")]
    Ipsec,
    #[serde(rename = "NONE")]
    None,
}

impl Default for Encryption {
    fn default() -> Encryption {
        Self::Ipsec
    }
}
/// [Output Only] The current status of whether or not this interconnect attachment is functional, which can take one of the following values: - OS_ACTIVE: The attachment has been turned up and is ready to use. - OS_UNPROVISIONED: The attachment is not ready to use yet, because turnup is not complete.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OperationalStatus {
    #[serde(rename = "OS_ACTIVE")]
    Active,
    #[serde(rename = "OS_UNPROVISIONED")]
    Unprovisioned,
}

impl Default for OperationalStatus {
    fn default() -> OperationalStatus {
        Self::Active
    }
}
/// The stack type for this interconnect attachment to identify whether the IPv6 feature is enabled or not. If not specified, IPV4_ONLY will be used. This field can be both set at interconnect attachments creation and update interconnect attachment operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StackType {
    #[serde(rename = "IPV4_IPV6")]
    Ipv6,
    #[serde(rename = "IPV4_ONLY")]
    Only,
}

impl Default for StackType {
    fn default() -> StackType {
        Self::Ipv6
    }
}
/// [Output Only] The current state of this attachment's functionality. Enum values ACTIVE and UNPROVISIONED are shared by DEDICATED/PRIVATE, PARTNER, and PARTNER_PROVIDER interconnect attachments, while enum values PENDING_PARTNER, PARTNER_REQUEST_RECEIVED, and PENDING_CUSTOMER are used for only PARTNER and PARTNER_PROVIDER interconnect attachments. This state can take one of the following values: - ACTIVE: The attachment has been turned up and is ready to use. - UNPROVISIONED: The attachment is not ready to use yet, because turnup is not complete. - PENDING_PARTNER: A newly-created PARTNER attachment that has not yet been configured on the Partner side. - PARTNER_REQUEST_RECEIVED: A PARTNER attachment is in the process of provisioning after a PARTNER_PROVIDER attachment was created that references it. - PENDING_CUSTOMER: A PARTNER or PARTNER_PROVIDER attachment that is waiting for a customer to activate it. - DEFUNCT: The attachment was deleted externally and is no longer functional. This could be because the associated Interconnect was removed, or because the other side of a Partner attachment was deleted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "DEFUNCT")]
    Defunct,
    #[serde(rename = "PARTNER_REQUEST_RECEIVED")]
    PartnerRequestReceived,
    #[serde(rename = "PENDING_CUSTOMER")]
    PendingCustomer,
    #[serde(rename = "PENDING_PARTNER")]
    PendingPartner,
    #[serde(rename = "STATE_UNSPECIFIED")]
    StateUnspecified,
    #[serde(rename = "UNPROVISIONED")]
    Unprovisioned,
}

impl Default for State {
    fn default() -> State {
        Self::Active
    }
}
/// The type of interconnect attachment this is, which can take one of the following values: - DEDICATED: an attachment to a Dedicated Interconnect. - PARTNER: an attachment to a Partner Interconnect, created by the customer. - PARTNER_PROVIDER: an attachment to a Partner Interconnect, created by the partner.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "DEDICATED")]
    Dedicated,
    #[serde(rename = "PARTNER")]
    Partner,
    #[serde(rename = "PARTNER_PROVIDER")]
    PartnerProvider,
}

impl Default for Type {
    fn default() -> Type {
        Self::Dedicated
    }
}