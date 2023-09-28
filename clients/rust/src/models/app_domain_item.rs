/*
 * Fermyon Cloud API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AppDomainItem {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "validationStatus")]
    pub validation_status: crate::models::ValidationStatus,
    #[serde(rename = "lastModified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
}

impl AppDomainItem {
    pub fn new(name: String, validation_status: crate::models::ValidationStatus) -> AppDomainItem {
        AppDomainItem {
            name,
            validation_status,
            last_modified: None,
        }
    }
}


