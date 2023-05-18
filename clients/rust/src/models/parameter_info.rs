/*
 * Fermyon.Cloud.Web
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ParameterInfo {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<crate::models::ParameterAttributes>,
    #[serde(rename = "member", skip_serializing_if = "Option::is_none")]
    pub member: Option<Box<crate::models::MemberInfo>>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "parameterType", skip_serializing_if = "Option::is_none")]
    pub parameter_type: Option<Box<crate::models::Type>>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(rename = "isIn", skip_serializing_if = "Option::is_none")]
    pub is_in: Option<bool>,
    #[serde(rename = "isLcid", skip_serializing_if = "Option::is_none")]
    pub is_lcid: Option<bool>,
    #[serde(rename = "isOptional", skip_serializing_if = "Option::is_none")]
    pub is_optional: Option<bool>,
    #[serde(rename = "isOut", skip_serializing_if = "Option::is_none")]
    pub is_out: Option<bool>,
    #[serde(rename = "isRetval", skip_serializing_if = "Option::is_none")]
    pub is_retval: Option<bool>,
    #[serde(rename = "defaultValue", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<Option<serde_json::Value>>,
    #[serde(rename = "rawDefaultValue", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub raw_default_value: Option<Option<serde_json::Value>>,
    #[serde(rename = "hasDefaultValue", skip_serializing_if = "Option::is_none")]
    pub has_default_value: Option<bool>,
    #[serde(rename = "customAttributes", skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<Vec<crate::models::CustomAttributeData>>,
    #[serde(rename = "metadataToken", skip_serializing_if = "Option::is_none")]
    pub metadata_token: Option<i32>,
}

impl ParameterInfo {
    pub fn new() -> ParameterInfo {
        ParameterInfo {
            attributes: None,
            member: None,
            name: None,
            parameter_type: None,
            position: None,
            is_in: None,
            is_lcid: None,
            is_optional: None,
            is_out: None,
            is_retval: None,
            default_value: None,
            raw_default_value: None,
            has_default_value: None,
            custom_attributes: None,
            metadata_token: None,
        }
    }
}


