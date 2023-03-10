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
pub struct DeviceCodeItem {
    #[serde(rename = "deviceCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub device_code: Option<Option<String>>,
    #[serde(rename = "userCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_code: Option<Option<String>>,
    #[serde(rename = "verificationUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub verification_url: Option<Option<String>>,
    #[serde(rename = "expiresIn", skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
}

impl DeviceCodeItem {
    pub fn new() -> DeviceCodeItem {
        DeviceCodeItem {
            device_code: None,
            user_code: None,
            verification_url: None,
            expires_in: None,
            interval: None,
        }
    }
}


