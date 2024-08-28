
use arbitrary::Arbitrary;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// All the below STRUCTS were modified so that I could quickly create an environment to showcase error
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrCanceledOptType {
    #[serde(rename = "ei:eiResponse")]
    #[serde(alias = "eiResponse")]
    pub ei_response: String,

    // Identifier for an opt interaction
    #[serde(rename = "ei:optID", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "optID")]
    pub opt_id: Option<String>,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrCancelOptType {
    #[serde(rename = "pyld:requestID")]
    #[serde(alias = "requestID")]
    pub request_id: String,

    // Identifier for an opt interaction
    #[serde(rename = "ei:optID")]
    #[serde(alias = "optID")]
    pub opt_id: String,

    #[serde(rename = "ei:venID")]
    #[serde(alias = "venID")]
    pub ven_id: String,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrCanceledPartyRegistrationType {
    #[serde(rename = "ei:eiResponse")]
    #[serde(alias = "eiResponse")]
    pub ei_response: String,

    // Identifier for Registration transaction. Not included in response to query registration unless already registered
    #[serde(rename = "ei:registrationID", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "registrationID")]
    pub registration_id: Option<String>,

    #[serde(rename = "ei:venID", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "venID")]
    pub ven_id: Option<String>,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrCancelPartyRegistrationType {
    #[serde(rename = "pyld:requestID")]
    #[serde(alias = "requestID")]
    pub request_id: String,

    // Identifier for Registration transaction. Not included in response to query registration unless already registered
    #[serde(rename = "ei:registrationID")]
    #[serde(alias = "registrationID")]
    pub registration_id: String,

    #[serde(rename = "ei:venID", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "venID")]
    pub ven_id: Option<String>,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrCanceledReportType {
    #[serde(rename = "ei:eiResponse")]
    #[serde(alias = "eiResponse")]
    pub ei_response: String,

    // List of periodic reports that have not yet been delivered
    #[serde(rename = "oadr:oadrPendingReports")]
    #[serde(alias = "oadrPendingReports")]
    pub oadr_pending_reports: OadrPendingReportsType,

    #[serde(rename = "ei:venID")]
    #[serde(alias = "venID")]
    pub ven_id: Option<String>,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrCancelReportType {
    #[serde(rename = "pyld:requestID")]
    #[serde(alias = "requestID")]
    pub request_id: String,

    // Identifier for a particular report request
    #[serde(
        default,
        rename = "ei:reportRequestID",
        skip_serializing_if = "Vec::is_empty"
    )]
    #[serde(alias = "reportRequestID")]
    pub report_request_id: Vec<String>,

    #[serde(rename = "pyld:reportToFollow")]
    #[serde(alias = "reportToFollow")]
    pub report_to_follow: bool,

    #[serde(rename = "ei:venID", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "venID")]
    pub ven_id: Option<String>,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrCreatedEventType {
    #[serde(rename = "pyld:eiCreatedEvent")]
    #[serde(alias = "eiCreatedEvent")]
    pub created_event: String,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrCreatedOptType {
    #[serde(rename = "ei:eiResponse")]
    #[serde(alias = "eiResponse")]
    pub ei_response: String,

    // Identifier for an opt interaction
    #[serde(rename = "ei:optID")]
    #[serde(alias = "optID")]
    pub opt_id: String,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrCreatedPartyRegistrationType {
    #[serde(rename = "ei:eiResponse")]
    #[serde(alias = "eiResponse")]
    pub ei_response: String,

    // Identifier for Registration transaction. Not included in response to query registration unless already registered
    #[serde(rename = "ei:registrationID", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "registrationID")]
    pub registration_id: Option<String>,

    // venID not included in query unless already registered
    #[serde(rename = "ei:venID", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "venID")]
    pub ven_id: Option<String>,

    #[serde(rename = "ei:vtnID")]
    #[serde(alias = "vtnID")]
    pub vtn_id: String,

    // VTN response to query registration returns all supported. This element is not required for a registration  response
    #[serde(rename = "oadr:oadrProfiles")]
    #[serde(alias = "oadrProfiles")]
    pub oadr_profiles: OadrProfiles,

    // HTTP Pull Only - The VEN shall send an oadrPoll payload to the VTN at most once for each duration specified by this element
    #[serde(
        rename = "oadr:oadrRequestOadrPollFreq",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(alias = "oadrRequestOadrPollFreq")]
    pub oadr_requested_oadr_poll_freq: Option<String>,

    // Service specific registration information
    #[serde(
        rename = "oadr:oadrServiceSpecificInfo",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(alias = "oadrServiceSpecificInfo")]
    pub oadr_service_specific_info: Option<OadrServiceSpecificInfo>,

    #[serde(
        default,
        rename = "oadr:oadrExtensions",
        skip_serializing_if = "Vec::is_empty"
    )]
    #[serde(alias = "oadrExtensions")]
    pub oadr_extensions: Vec<OadrExtension>,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrCreatePartyRegistrationType {
    #[serde(rename = "pyld:requestID")]
    #[serde(alias = "requestID")]
    pub request_id: String,

    // Identifier for Registration transaction. Not included in response to query registration unless already registered
    #[serde(rename = "ei:registrationID", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "registrationID")]
    pub registration_id: Option<String>,

    #[serde(rename = "ei:venID", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "venID")]
    pub ven_id: Option<String>,

    // OpenADR profile name such as 2.0a or 2.0b.
    #[serde(rename = "oadr:oadrProfileName")]
    #[serde(alias = "oadrProfileName")]
    pub oadr_profile_name: OadrProfileType,

    // OpenADR transport name such as simpleHttp or xmpp
    #[serde(rename = "oadr:oadrTransportName")]
    #[serde(alias = "oadrTransportName")]
    pub oadr_transport_name: OadrTransportType,

    // Address of this VEN. Not required if http pull model
    #[serde(
        rename = "oadr:oadrTransportAddress",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(alias = "oadrTransportAddress")]
    pub oadr_transport_address: Option<String>,

    // ReportOnlyDeviceFlag - True or False
    #[serde(rename = "oadr:oadrReportOnly")]
    #[serde(alias = "oadrReportOnly")]
    pub oadr_report_only: bool,

    // Implementation supports XML signatures - True or False
    #[serde(rename = "oadr:oadrXmlSignature")]
    #[serde(alias = "oadrXmlSignature")]
    pub oadr_xml_signature: bool,

    // Human readable name for VEN
    #[serde(rename = "oadr:oadrVenName", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "oadrVenName")]
    pub oadr_ven_name: Option<String>,

    // If transport is simpleHttp indicate if VEN is operating in pull exchange model - true or false
    #[serde(rename = "oadrHttpPullModel", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "oadrHttpPullModel")]
    pub oadr_http_pull_model: Option<bool>,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrCreatedReportType {
    #[serde(rename = "ei:eiResponse")]
    #[serde(alias = "eiResponse")]
    pub ei_reponse: String,

    // List of periodic reports that have not yet been delivered
    #[serde(rename = "oadr:oadrPendingReports")]
    #[serde(alias = "oadrPendingReports")]
    pub oadr_pending_reports: OadrPendingReportsType,

    #[serde(rename = "ei:venID", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "venID")]
    pub ven_id: Option<String>,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrCreateOptType {
    #[serde(rename = "ei:EiOptType")]
    #[serde(alias = "EiOptType")]
    pub ei_opt_type: String,

    #[serde(rename = "pyld:requestID")]
    #[serde(alias = "requestID")]
    pub request_id: String,

    #[serde(
        rename = "ei:qualifiedEventID",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(alias = "qualifiedEventID")]
    pub qualified_event_id: Option<String>,

    #[serde(rename = "ei:eiTarget")]
    #[serde(alias = "eiTarget")]
    pub ei_target: String,

    // Device Class target - use only endDeviceAsset.
    #[serde(
        rename = "oadr:oadrDeviceClass",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(alias = "oadrDeviceClass")]
    pub oadr_device_class: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrCreateReportType {
    #[serde(rename = "pyld:requestID")]
    #[serde(alias = "requestID")]
    pub request_id: String,

    // Request report
    #[serde(
        default,
        rename = "oadr:oadrReportRequest",
        skip_serializing_if = "Vec::is_empty"
    )]
    #[serde(alias = "oadrReportRequest")]
    pub oadr_report_request: Vec<OadrReportRequestType>,

    #[serde(rename = "ei:venID", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "venID")]
    pub ven_id: Option<String>,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrDistributeEventType {
    #[serde(rename = "ei:eiResponses", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "eiResponses")]
    pub ei_response: Option<String>,

    #[serde(rename = "pyld:requestID")]
    #[serde(alias = "requestID")]
    pub request_id: String,

    #[serde(rename = "ei:vtnID")]
    #[serde(alias = "vtnID")]
    pub vtn_id: String,

    // An object containing a demand response event
    #[serde(
        default,
        rename = "oadr:oadrEvent",
        skip_serializing_if = "Vec::is_empty"
    )]
    #[serde(alias = "oadrEvent")]
    pub oadr_event: Vec<OadrEvent>,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrEvent {
    #[serde(rename = "ei:eiEvent")]
    #[serde(alias = "eiEvent")]
    pub ei_event: String,

    // Controls when optIn/optOut repsonse is required. Can be always or never
    #[serde(rename = "oadr:oadrResponseRequired")]
    #[serde(alias = "oadrResponseRequired")]
    pub oadr_response_required: String,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrExtension {
    #[serde(rename = "oadr:oadrExtensionName")]
    #[serde(alias = "oadrExtensionName")]
    pub oadr_extension_name: String,

    // A key value pair of service specific registration information
    #[serde(
        default,
        rename = "oadr:oadrInfo",
        skip_serializing_if = "Vec::is_empty"
    )]
    #[serde(alias = "oadrInfo")]
    pub oadr_info: Vec<OadrInfo>,
}

// /// SEE OADR_20B
// #[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
// pub struct OadrExtensions {
//     #[serde(Arbitrary, Default, flatten, skip_serializing_if = "Vec::is_empty")]
//     pub oadr_extension: Vec<OadrExtension>,
// }

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrGBItemBase {
    //extends strm:StreamPayloadBaseType which is not implemented beacuse its an abstract
    #[serde(rename = "atom:feed")]
    #[serde(alias = "feed")]
    pub feed: String,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrGBStreamPayloadBase {
    #[serde(flatten)]
    pub item_base: String,

    #[serde(rename = "atom:feed")]
    #[serde(alias = "feed")]
    pub feed: String,
}

/// SEE OADR_20B
/// A key value pair of service specific registration information
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrInfo {
    #[serde(rename = "oadr:oadKey")]
    #[serde(alias = "oadKey")]
    pub oadr_key: String,

    #[serde(rename = "oadr:oadrValue")]
    #[serde(alias = "oadrValue")]
    pub oadr_value: String,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrLoadControlStateType {
    #[serde(rename = "oadr:oadrCapacity", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "oadrCapacity")]
    pub oadr_capacity: Option<OadrLoadControlStateTypeType>,

    #[serde(
        rename = "oadr:oadrLevelOffset",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(alias = "oadrLevelOffset")]
    pub oadr_level_offset: Option<OadrLoadControlStateTypeType>,

    #[serde(
        rename = "oadr:oadrPercentOffset",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(alias = "oadrPercentOffset")]
    pub oadr_percent_offset: Option<OadrLoadControlStateTypeType>,

    #[serde(rename = "oadr:oadrSetPoint", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "oadrSetPoint")]
    pub oadr_set_point: Option<OadrLoadControlStateTypeType>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrLoadControlStateTypeType {
    #[serde(rename = "oadr:oadrMin", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "oadrMin")]
    pub oadr_min: Option<String>,

    #[serde(rename = "oadr:oadrMax", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "oadrMax")]
    pub oadr_max: Option<String>,

    #[serde(rename = "oadr:oadrCurrent")]
    #[serde(alias = "oadrCurrent")]
    pub oadr_current: String,

    #[serde(rename = "oadr:oadrNormal", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "oadrNormal")]
    pub oadr_normal: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrPayload {
    #[serde(rename = "ds:Signature", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "Signature")]
    pub signature: Option<String>,

    #[serde(rename = "oadr:oadrSignedObject")]
    #[serde(alias = "oadrSignedObject")]
    pub oadr_signed_object: OadrSignedObject,
}

/// SEE OADR_20B
/// This is the payload for reports that require a status.
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrPayloadResourceStatusType {
    // If true then resource/asset is online, if false then offline.
    #[serde(rename = "oadr:oadrOnline")]
    #[serde(alias = "oadrOnline")]
    pub oadr_online: bool,

    // If true then the control of the load has been manually overridden
    #[serde(rename = "oadr:oadrManualOverride")]
    #[serde(alias = "oadrManualOverride")]
    pub oadr_manual_override: bool,

    #[serde(
        rename = "oadr:oadrLoadControlState",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(alias = "oadrLoadControlState")]
    pub oadr_load_control_state: Option<OadrLoadControlStateType>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrPendingReportsType {
    // Identifier for a particular report request
    #[serde(
        default,
        rename = "ei:reportRequestID",
        skip_serializing_if = "Vec::is_empty"
    )]
    #[serde(alias = "reportRequestID")]
    pub report_request_id: Vec<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrPollType {
    #[serde(rename = "ei:venID")]
    #[serde(alias = "venID")]
    pub ven_id: String,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrProfiles {
    // OpenADR profile name such as 2.0a or 2.0b.
    #[serde(rename = "oadr:oadrProfileName")]
    #[serde(alias = "oadrProfileName")]
    pub oadr_profile_name: OadrProfileType,

    // OpenADR transports supported by implementation
    #[serde(rename = "oadr:oadrTransports")]
    #[serde(alias = "oadrTransports")]
    pub oadr_transports: OadrTransports,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub enum OadrProfileTypeEnum {
    //token
    TwoPointOA(String),
    TwoPointOB(String),
}

#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrProfileType {
    #[serde(rename = "$value")]
    pub value: OadrProfileTypeEnum,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrQueryRegistrationType {
    #[serde(rename = "pyld:requestID")]
    #[serde(alias = "requestID")]
    pub request_id: String,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrRegisteredReportType {
    #[serde(rename = "ei:eiResponse")]
    #[serde(alias = "eiResponse")]
    pub ei_response: String,

    // Request report
    #[serde(
        default,
        rename = "oadr:oadrReportRequest",
        skip_serializing_if = "Vec::is_empty"
    )]
    #[serde(alias = "oadrReportRequest")]
    pub oadr_report_request: Vec<OadrReportRequestType>,

    #[serde(rename = "ei:venID", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "venID")]
    pub ven_id: Option<String>,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrRegisterReportType {
    #[serde(rename = "pyld:requestID")]
    #[serde(alias = "requestID")]
    pub request_id: String,

    #[serde(
        default,
        rename = "oadr:oadrReport",
        skip_serializing_if = "Vec::is_empty"
    )]
    #[serde(alias = "oadrReport")]
    pub oadr_report: Vec<OadrReportType>,

    #[serde(rename = "ei:venID", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "venID")]
    pub ven_id: Option<String>,

    // Identifier for a particular report request
    #[serde(rename = "ei:reportRequestID", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "reportRequestID")]
    pub report_request_id: Option<String>,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
/// Describes the subject and attributes of a report.
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrReportDescriptionType {
    // ReferenceID for this data point
    #[serde(rename = "ei:rID")]
    #[serde(alias = "rID")]
    pub r_id: String,

    // Device Class target - use only endDeviceAsset.
    #[serde(rename = "ei:reportSubject", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "reportSubject")]
    pub report_subject: Option<String>,

    // Sources for data in this report. Examples include meters or submeters.
    // For example, if a meter is capable of providing two different types of measurements,
    // then each measurement stream would be separately identified.
    #[serde(
        rename = "ei:reportDataSource",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(alias = "reportDataSource")]
    pub report_data_source: Option<String>,

    // An enumerated value that gives the type of report being provided.
    #[serde(rename = "ei:reportType")]
    #[serde(alias = "reportType")]
    pub report_type: String,

    // What is measured or tracked in this report (Units).
    #[serde(rename = "emix:itemBase", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "itemBase")]
    pub item_base: Option<String>,

    // Metadata about the Readings, such as mean or derived
    #[serde(rename = "ei:readingType")]
    #[serde(alias = "readingType")]
    pub reading_type: String,

    #[serde(rename = "emix:marketContext", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "marketContext")]
    pub market_context: Option<String>,

    // Sampling rate for telemetry type data
    #[serde(
        rename = "oadr:oadrSamplingRate",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(alias = "oadrSamplingRate")]
    pub oadr_sampling_rate: Option<OadrSamplingRateType>,
}

/// SEE OADR_20B
// Data point values for reports
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrReportPayloadType {
    // Report payload for use in reports.
    #[serde(flatten)]
    pub report_payload_type: String,

    #[serde(
        rename = "oadr:oadrDataQuality",
        skip_serializing_if = "Option::is_none"
    )]
    // Enumerated value for the quality of this data item
    #[serde(alias = "oadrDataQuality")]
    pub oadr_data_quality: Option<String>,
}

/// SEE OADR_20B
/// This type is used to request an EiReport
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrReportRequestType {
    // Identifier for a particular report request
    #[serde(rename = "ei:reportRequestID")]
    #[serde(alias = "reportRequestID")]
    pub report_request_id: String,

    #[serde(rename = "ei:reportSpecifier")]
    #[serde(alias = "reportSpecifier")]
    pub report_specifier: String,
}

/// SEE OADR_20B
/// eiReport is a Stream of [measurements] recorded over time and delivered to the requestor periodically. The readings may be actual, computed, summed if derived in some other manner.
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrReportType {
    #[serde(rename = "strm:streamBaseType")]
    #[serde(alias = "streamBaseType")]
    pub stream_base_type: String,

    // reference ID to this report.
    #[serde(rename = "ei:reportID", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "reportID")]
    pub report_id: Option<String>,

    // Define data points the implementation is capable of reporting on. Only used in Metadata report
    #[serde(
        default,
        rename = "oadr:oadrReportDescription",
        skip_serializing_if = "Vec::is_empty"
    )]
    #[serde(alias = "oadrReportDescription")]
    pub oadr_report_description: Vec<OadrReportDescriptionType>,

    // Identifier for a particular report request
    #[serde(rename = "ei:reportRequestID")]
    #[serde(alias = "reportRequestID")]
    pub report_request_id: String,

    // Identifier for a particular Metadata report specification
    #[serde(rename = "ei:reportSpecifierID")]
    #[serde(alias = "reportSpecifierID")]
    pub report_specifier_id: String,

    // Optional name for a report.
    #[serde(rename = "ei:reportName", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "reportName")]
    pub report_name: Option<String>,

    #[serde(rename = "ei:createdDateTime")]
    #[serde(alias = "createdDateTime")]
    pub created_date_time: String,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrRequestEventType {
    #[serde(rename = "pyld:eiRequestEvent")]
    #[serde(alias = "eiRequestEvent")]
    pub ei_request_event: String,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
/// Used by VTN to request that the VEN reregister
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrRequestReregistrationType {
    #[serde(rename = "ei:venID")]
    #[serde(alias = "venID")]
    pub ven_id: String,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrResponseType {
    #[serde(rename = "ei:eiResponse")]
    #[serde(alias = "eiResponse")]
    pub ei_response: String,

    #[serde(rename = "ei:venID", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "venID")]
    pub ven_id: Option<String>,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrSamplingRateType {
    // Minimum sampling period
    #[serde(rename = "oadr:oadrMinPeriod")]
    #[serde(alias = "oadrMinPeriod")]
    pub oadr_min_period: String,

    // Maximum sampling period
    #[serde(rename = "oadr:oadrMaxPeriod")]
    #[serde(alias = "oadrMaxPeriod")]
    pub oadr_max_period: String,

    // If true then the data will be recorded when it changes, but at no greater a frequency than that specified  by minPeriod.
    #[serde(rename = "oadr:oadrOnChange")]
    #[serde(alias = "oadrOnChange")]
    pub oadr_on_change: bool,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrService {
    #[serde(rename = "oadr:oadrServiceName")]
    #[serde(alias = "oadrServiceName")]
    pub oadr_service_name: OadrServiceNameType,

    // A key value pair of service specific registration information
    #[serde(
        default,
        rename = "oadr:oadrInfo",
        skip_serializing_if = "Vec::is_empty"
    )]
    #[serde(alias = "oadrInfo")]
    pub oadr_info: Vec<OadrInfo>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub enum OadrServiceNameTypeEnum {
    EiEvent(String),
    EiOpt(String),
    EiReport(String),
    EiRegisterParty(String),
    OadrPoll(String),
}

#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrServiceNameType {
    #[serde(rename = "$value")]
    pub value: OadrServiceNameTypeEnum,
}

/// SEE OADR_20B
/// Service specific registration information
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrServiceSpecificInfo {
    #[serde(
        default,
        rename = "oadr:oadrServiceSpecificInfo",
        skip_serializing_if = "Vec::is_empty"
    )]
    #[serde(alias = "oadrServiceSpecificInfo")]
    pub oadr_service: Vec<OadrService>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrSignedObject {
    #[serde(rename = "@Id", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@Id")]
    pub id: Option<String>,

    //Send DR Events to a VEN
    #[serde(rename = "oadr:oadrDistributeEvent")]
    #[serde(alias = "oadrDistributeEvent")]
    pub oadr_distribute_event: OadrDistributeEventType,

    #[serde(rename = "oadr:oadrCreatedEvent")]
    #[serde(alias = "oadrCreatedEvent")]
    pub oadr_create_event: OadrCreatedEventType,

    #[serde(rename = "oadr:oadrRequestEvent")]
    #[serde(alias = "oadrRequestEvent")]
    pub oadr_request_event: OadrRequestEventType,

    #[serde(rename = "oadr:oadrResponse")]
    #[serde(alias = "oadrResponse")]
    pub oadr_response: OadrResponseType,

    // Acknowledge cancelation of an opt schedule
    #[serde(rename = "oadr:oadrCancelOpt")]
    #[serde(alias = "oadrCancelOpt")]
    pub oadr_cancel_opt: OadrCancelOptType,

    #[serde(rename = "oadr:oadrCanceledOpt")]
    #[serde(alias = "oadrCanceledOpt")]
    pub oadr_canceled_opt: OadrCanceledOptType,

    // Create an optIn or optOut schedule
    #[serde(rename = "oadr:oadrCreateOpt")]
    #[serde(alias = "oadrCreateOpt")]
    pub oadr_create_opt: OadrCreateOptType,

    // Acknowledge receipt of an opt schedule
    #[serde(rename = "oadr:oadrCreadtedOpt")]
    #[serde(alias = "oadrCreadtedOpt")]
    pub oadr_created_opt: OadrCreatedOptType,

    // Cancel a report
    #[serde(rename = "oadr:oadrCancelReport")]
    #[serde(alias = "oadrCancelReport")]
    pub oadr_cancel_report: OadrCancelReportType,

    #[serde(rename = "oadr:oadrCanceledReport")]
    #[serde(alias = "oadrCanceledReport")]
    pub oadr_canceled_report: OadrCanceledReportType,

    //Request report from other party
    #[serde(rename = "oadr:oadrCreateReport")]
    #[serde(alias = "oadrCreateReport")]
    pub oadr_create_report: OadrCreateReportType,

    // Acknowledge the request for report was received
    #[serde(rename = "oadr:oadrCreatedReport")]
    #[serde(alias = "oadrCreatedReport")]
    pub oadr_created_report: OadrCreatedReportType,

    //Register Metadata report settings with other party
    #[serde(rename = "oadr:oadrRegisterReport")]
    #[serde(alias = "oadrRegisterReport")]
    pub oadr_register_report: OadrRegisterReportType,

    // Acknowledge registration of Metadata report by other party
    #[serde(rename = "oadr:oadrRegisteredReport")]
    #[serde(alias = "oadrRegisteredReport")]
    pub oadr_registered_report: OadrRegisteredReportType,

    // Send a previously requested report
    #[serde(rename = "oadr:oadrUpdateReport")]
    #[serde(alias = "oadrUpdateReport")]
    pub oadr_update_report: OadrUpdateReportType,

    // Acknowledge receipt of a report
    #[serde(rename = "oadr:oadrUpdatedReport")]
    #[serde(alias = "oadrUpdatedReport")]
    pub oadr_updated_report: OadrUpdatedReportType,

    // Cancel a registration
    #[serde(rename = "oadr:oadrCancelPartyRegistration")]
    #[serde(alias = "oadrCancelPartyRegistration")]
    pub oadr_cancel_party_registration: OadrCancelPartyRegistrationType,

    // Acknowledge cancelation of registration
    #[serde(rename = "oadr:oadrCanceledPartyRegistration")]
    #[serde(alias = "oadrCanceledPartyRegistration")]
    pub oadr_canceled_party_registration: OadrCanceledPartyRegistrationType,

    // Used by VEN to initiate registration with VTN
    #[serde(rename = "oadr:oadrCreatePartyRegistration")]
    #[serde(alias = "oadrCreatePartyRegistration")]
    pub oadr_create_party_registration: OadrCreatePartyRegistrationType,

    // Acknowledge receipt of VEN registration, provide VTN registration info
    #[serde(rename = "oadr:oadrCreatedPartyRegistration")]
    #[serde(alias = "oadrCreatedPartyRegistration")]
    pub oadr_created_party_registration: OadrCreatedPartyRegistrationType,

    #[serde(rename = "oadr:oadrRequestRegistration")]
    #[serde(alias = "oadrRequestRegistration")]
    pub oadr_request_registration: OadrRequestReregistrationType,

    // Query VTN for registration information without actually registering
    #[serde(rename = "oadr:oadrQueryRegistration")]
    #[serde(alias = "oadrQueryRegistration")]
    pub oadr_query_registration: OadrQueryRegistrationType,

    // Query pull VTN for payloads with new or modified information
    #[serde(rename = "oadr:oadrPoll")]
    #[serde(alias = "oadrPoll")]
    pub oadr_poll: OadrPollType,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub enum OadrTransportTypeEnum {
    //token
    SimpleHttp(String),
    Xmpp(String),
}

#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrTransportType {
    #[serde(rename = "$value")]
    pub value: OadrTransportTypeEnum,
}
/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrTransports {
    // OpenADR transports supported by implementation
    #[serde(
        default,
        rename = "oadr:oadrTransports",
        skip_serializing_if = "Vec::is_empty"
    )]
    #[serde(alias = "oadrTransports")]
    pub oadr_transport: Vec<OadrTransportType>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrUpdatedReportType {
    #[serde(rename = "ei:eiResponse")]
    #[serde(alias = "eiResponse")]
    pub ei_response: String,

    // Cancel a report
    #[serde(
        rename = "oadr:oadrCancelReport",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(alias = "oadrCancelReport")]
    pub oadr_cancel_report: Option<OadrCancelReportType>,

    #[serde(rename = "ei:venID", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "venID")]
    pub ven_id: Option<String>,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}

/// SEE OADR_20B
#[derive(Arbitrary, Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct OadrUpdateReportType {
    #[serde(rename = "pyld:requestID")]
    #[serde(alias = "requestID")]
    pub request_id: String,

    #[serde(
        default,
        rename = "oadr:oadrReport",
        skip_serializing_if = "Vec::is_empty"
    )]
    #[serde(alias = "oadrReport")]
    pub oadr_report: Vec<OadrReportType>,

    #[serde(rename = "ei:venID", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "venID")]
    pub ven_id: Option<String>,

    #[serde(rename = "@ei:schemaVersion", skip_serializing_if = "Option::is_none")]
    #[serde(alias = "@schemaVersion")]
    pub schema_version: Option<String>,
}
