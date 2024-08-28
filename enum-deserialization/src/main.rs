use oadr::{OadrCancelOptType, OadrCancelPartyRegistrationType, OadrCancelReportType, OadrCreateOptType, OadrCreatePartyRegistrationType, OadrCreateReportType, OadrCreatedEventType, OadrCreatedReportType, OadrDistributeEventType, OadrEvent, OadrPollType, OadrQueryRegistrationType, OadrRegisterReportType, OadrRequestEventType, OadrRequestReregistrationType, OadrResponseType, OadrUpdateReportType};
use serde::{Deserialize, Serialize};

pub mod oadr;


pub trait IntoXmlString {
    fn into_xml_string(self) -> String;
}

pub trait TryFromXmlStr: Sized {
    fn try_from_xml_str(xml: &str) -> Result<Self, serde_path_to_error::Error<quick_xml::DeError>>;
}

macro_rules! impl_into_xml_string {
    ($($t:ty, $root:literal)*) => {
        $(
            impl IntoXmlString for $t {
                fn into_xml_string(self) -> String {
                    let mut xml = String::new();
                    let serializer = quick_xml::se::Serializer::with_root(&mut xml, Some($root)).expect(&concat!($root, " must be a valid root tag."));
                    let se_result = serde_path_to_error::serialize(&self, serializer);

                    if let Err(error) = se_result {
                        panic!("Error serializing {}: {error:?}", std::any::type_name::<$t>());
                    }

                    xml
                }
            }
        )*
    }
}


macro_rules! impl_try_from_xml_str {
    ($($t:ty)*) => {
        $(
            impl TryFromXmlStr for $t {
                fn try_from_xml_str(xml: &str) -> Result<Self, serde_path_to_error::Error<quick_xml::DeError>> {
                    let deserializer = &mut quick_xml::de::Deserializer::from_str(xml);
                    Ok(serde_path_to_error::deserialize(deserializer)?)
                }
            }
        )*
    }
}

impl_into_xml_string! {
    OadrPollType, "oadr:OadrPoll"
    OadrRequestEventType, "oadr:OadrRequestEvent"
    OadrCreatedReportType, "oadr:OadrCreatedReport"
    OadrCreateReportType, "oadr:OadrCreateReport"
    OadrRegisterReportType, "oadr:OadrRegisterReport"
    OadrCancelReportType, "oadr:OadrCancelReport"
    OadrUpdateReportType, "oadr:OadrUpdateReport"
    OadrCancelPartyRegistrationType, "oadr:OadrCancelPartyRegistration"
    OadrRequestReregistrationType, "oadr:OadrRequestReregistration"
    OadrCreatedEventType, "oadr:OadrCreatedEvent"
    OadrQueryRegistrationType, "oadr:OadrQueryRegistration"
    OadrCreatePartyRegistrationType, "oadr:OadrCreatePartyRegistration"
    OadrCancelOptType, "oadr:OadrCancelOpt"
    OadrCreateOptType, "oadr:OadrCreateOpt"
    // this is here so I can use into xml string
    PollResponse, "oadr:OadrDistributeEvent"
}

impl_try_from_xml_str! {
    PollResponse
}



#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PollResponse {
    OadrResponseType(OadrResponseType),
    OadrDistributeEventType(OadrDistributeEventType),
    OadrCreateReportType(OadrCreateReportType),
    OadrRegisterReportType(OadrRegisterReportType),
    OadrCancelReportType(OadrCancelReportType),
    OadrUpdateReportType(OadrUpdateReportType),
    OadrCancelPartyRegistrationType(OadrCancelPartyRegistrationType),
    OadrRequestReregistrationType(OadrRequestReregistrationType),
}

/// Below are the actual errors gotten from my terminal when I ran my code in my application
/// As stated in oadr.rs file structs were modified so as to quickly create an environment to showcase error hence why printed values are different
///<oadr:OadrDistributeEvent ei:schemaVersion="1"><ei:eiResponses><ei:responseCode>00001</ei:responseCode><ei:responseDescription>response</ei:responseDescription><pyld:requestID>00001</pyld:requestID></ei:eiResponses><pyld:requestID>00001</pyld:requestID><ei:vtnID>00001</ei:vtnID></oadr:OadrDistributeEvent>
///Error: data did not match any variant of untagged enum PollResponse
///OadrDistributeEventType(OadrDistributeEventType { ei_response: Some(EiResponseType { response_code: "00001", response_description: Some("response"), request_id: "00001" }), request_id: "00001", vtn_id: "00001", oadr_event: [], schema_version: Some("1") })

fn main() {
    let obj = PollResponse::OadrDistributeEventType(OadrDistributeEventType{ ei_response: Some(String::from("HI")), 
        request_id:String::from("HI"), vtn_id: String::from("HI"), 
        oadr_event: vec![OadrEvent{ ei_event: String::from("HI"), oadr_response_required: String::from("HI") }], schema_version: Some(String::from("HI")) });

        println!("{:?}", obj);

     let response = obj.into_xml_string();

     println!("{:?}", response);

     let deserialize = <PollResponse as TryFromXmlStr>::try_from_xml_str(&response);

    println!("{:?}", deserialize);
}
