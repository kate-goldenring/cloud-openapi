/*
 * Fermyon Cloud API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DomainValidationStatus {
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Ready")]
    Ready,

}

impl ToString for DomainValidationStatus {
    fn to_string(&self) -> String {
        match self {
            Self::InProgress => String::from("InProgress"),
            Self::Ready => String::from("Ready"),
        }
    }
}

impl Default for DomainValidationStatus {
    fn default() -> DomainValidationStatus {
        Self::InProgress
    }
}



