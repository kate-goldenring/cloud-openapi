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
pub struct CreateTokenCommand {
    #[serde(rename = "providerCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub provider_code: Option<Option<String>>,
    #[serde(rename = "clientId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<Option<String>>,
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<crate::models::AccountProvider>,
}

impl CreateTokenCommand {
    pub fn new() -> CreateTokenCommand {
        CreateTokenCommand {
            provider_code: None,
            client_id: None,
            provider: None,
        }
    }
}


