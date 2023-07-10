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
pub struct LockedManifestMetadata {
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "version", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub version: Option<Option<String>>,
    #[serde(rename = "origin", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub origin: Option<Option<String>>,
    #[serde(rename = "trigger", skip_serializing_if = "Option::is_none")]
    pub trigger: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl LockedManifestMetadata {
    pub fn new() -> LockedManifestMetadata {
        LockedManifestMetadata {
            description: None,
            name: None,
            version: None,
            origin: None,
            trigger: None,
        }
    }
}


