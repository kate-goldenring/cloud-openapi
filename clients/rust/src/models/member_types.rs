/*
 * Fermyon.Cloud.Web
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MemberTypes {
    #[serde(rename = "Constructor")]
    Constructor,
    #[serde(rename = "Event")]
    Event,
    #[serde(rename = "Field")]
    Field,
    #[serde(rename = "Method")]
    Method,
    #[serde(rename = "Property")]
    Property,
    #[serde(rename = "TypeInfo")]
    TypeInfo,
    #[serde(rename = "Custom")]
    Custom,
    #[serde(rename = "NestedType")]
    NestedType,
    #[serde(rename = "All")]
    All,

}

impl ToString for MemberTypes {
    fn to_string(&self) -> String {
        match self {
            Self::Constructor => String::from("Constructor"),
            Self::Event => String::from("Event"),
            Self::Field => String::from("Field"),
            Self::Method => String::from("Method"),
            Self::Property => String::from("Property"),
            Self::TypeInfo => String::from("TypeInfo"),
            Self::Custom => String::from("Custom"),
            Self::NestedType => String::from("NestedType"),
            Self::All => String::from("All"),
        }
    }
}

impl Default for MemberTypes {
    fn default() -> MemberTypes {
        Self::Constructor
    }
}



